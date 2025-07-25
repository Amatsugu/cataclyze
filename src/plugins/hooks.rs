use bevy::prelude::*;

use crate::components::stats::{Health, MaxHealth, MoveSpeed, MoveSpeedStat};

pub struct HooksPlugin;

impl Plugin for HooksPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(PreStartup, setup);
	}
}

fn setup(world: &mut World) {
	//Move Speed Hook
	world
		.register_component_hooks::<MoveSpeedStat>()
		.on_insert(|mut world, ctx| {
			let s = world.get::<MoveSpeedStat>(ctx.entity).unwrap().0;
			let mut commands = world.commands();
			let mut entity_commands = commands.entity(ctx.entity);
			entity_commands.insert(MoveSpeed(s));
		});
	//Health Hook
	world.register_component_hooks::<MaxHealth>().on_add(|mut world, ctx| {
		let max_health = world.get::<MaxHealth>(ctx.entity).unwrap().0;
		let mut commands = world.commands();
		let mut entity_commands = commands.entity(ctx.entity);
		entity_commands.insert(Health(max_health));
	});
}
