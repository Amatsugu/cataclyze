use bevy::prelude::*;

use crate::{
	components::utils::Cleanable,
	plugins::utils::play_audio_onshot,
	resources::{
		audio::AudioClips,
		utils::{Fonts, KillCount},
	},
	state_management::{GameOverState, GameOverSystems, GameplayState},
};

pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
	fn build(&self, app: &mut App) {
		app.insert_state(GameOverState::Init);
		app.add_systems(
			PreUpdate,
			spawn_menu.in_set(GameOverSystems).run_if(in_state(GameOverState::Init)),
		);
		app.add_systems(
			Update,
			menu.in_set(GameOverSystems).run_if(in_state(GameOverState::Wait)),
		);
	}
}

fn spawn_menu(
	mut commands: Commands,
	mut next: ResMut<NextState<GameOverState>>,
	fonts: Res<Fonts>,
	audio: Res<AudioClips>,
	kill_count: Res<KillCount>,
) {
	commands.spawn((
		Cleanable,
		Transform::from_xyz(0.0, 50., 0.0),
		Text2d::new("Game Over"),
		TextFont {
			font: fonts.noto_thin.clone(),
			font_size: 100.,
			..default()
		},
		TextLayout::new_with_justify(JustifyText::Center),
	));
	commands.spawn((
		Cleanable,
		Transform::from_xyz(0.0, 0.0, 0.0),
		Text2d::new(format!("Kills: {}", kill_count.0)),
		TextFont {
			font: fonts.noto_thin.clone(),
			font_size: 20.,
			..default()
		},
		TextLayout::new_with_justify(JustifyText::Center),
	));
	commands.spawn((
		Cleanable,
		Transform::from_xyz(0.0, -30., 0.0),
		Text2d::new("Press [R] to Restart"),
		TextFont {
			font: fonts.noto_thin.clone(),
			font_size: 20.,
			..default()
		},
		TextLayout::new_with_justify(JustifyText::Center),
	));
	next.set(GameOverState::Wait);
	play_audio_onshot(&mut commands, audio.gameover.clone());
}

fn menu(
	key: Res<ButtonInput<KeyCode>>,
	mut next_gm: ResMut<NextState<GameOverState>>,
	mut next_game: ResMut<NextState<GameplayState>>,
) {
	if key.just_pressed(KeyCode::KeyR) {
		next_gm.set(GameOverState::Wait);
		next_game.set(GameplayState::Cleanup);
		info!("Moving to Cleanup");
	}
}
