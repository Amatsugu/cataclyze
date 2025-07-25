use bevy::prelude::*;

use crate::{
	components::{
		stats::{Life, MoveSpeed, MoveSpeedMultiplier, MoveSpeedStat},
		utils::{Cleanable, Lifetime},
	},
	state_management::{GameOverSystems, GameplaySystems},
};

pub struct UtilsPlugin;

impl Plugin for UtilsPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(
			PostUpdate,
			(process_lifetimes, process_lifetimes_life).in_set(GameOverSystems),
		);
		app.add_systems(
			PostUpdate,
			(process_lifetimes, process_lifetimes_life).in_set(GameplaySystems),
		);
		app.add_systems(PreUpdate, process_move_speed.in_set(GameplaySystems));
		app.add_systems(PreUpdate, process_move_speed.in_set(GameOverSystems));
	}
}
fn process_lifetimes(
	mut query: Query<(Entity, &mut Lifetime), Without<Life>>,
	time: Res<Time>,
	mut commands: Commands,
) {
	let delta = time.delta();
	for (entity, mut despawn) in query.iter_mut() {
		despawn.0.tick(delta);
		if despawn.0.finished() {
			commands.entity(entity).despawn();
		}
	}
}

fn process_lifetimes_life(mut query: Query<(&mut Lifetime, &mut Life)>, time: Res<Time>) {
	let delta = time.delta();
	for (mut lifetime, mut life) in query.iter_mut() {
		if life.is_dead() {
			continue;
		}
		lifetime.0.tick(delta);
		if lifetime.0.finished() {
			life.0 = false;
		}
	}
}

fn process_move_speed(mut query: Query<(&mut MoveSpeed, &MoveSpeedStat, &MoveSpeedMultiplier)>) {
	for (mut adj, speed, multi) in &mut query {
		adj.0 = speed.0 * multi.0;
	}
}

pub fn play_audio_onshot(commands: &mut Commands, audio: Handle<AudioSource>) {
	commands.spawn((AudioPlayer::new(audio), PlaybackSettings::DESPAWN, Cleanable));
}

#[allow(dead_code)]
pub fn play_audio_onshot_spacial(commands: &mut Commands, pos: Vec3, audio: Handle<AudioSource>) {
	commands.spawn((
		Transform::from_translation(pos),
		AudioPlayer::new(audio),
		PlaybackSettings::DESPAWN.with_spatial(true),
		Cleanable,
	));
}
