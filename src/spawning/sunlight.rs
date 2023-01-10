use crate::spawning::{GameObject, PrimedGameObjectSpawner};
use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

impl<'w, 's, 'a, 'b> PrimedGameObjectSpawner<'w, 's, 'a, 'b> {
    pub fn spawn_sunlight(&'a mut self) -> EntityCommands<'w, 's, 'a> {
        // directional 'sun' light
        const HALF_SIZE: f32 = 10.0;
        self.commands.spawn((
            DirectionalLightBundle {
                directional_light: DirectionalLight {
                    // Configure the projection to better fit the scene
                    shadow_projection: OrthographicProjection {
                        left: -HALF_SIZE,
                        right: HALF_SIZE,
                        bottom: -HALF_SIZE,
                        top: HALF_SIZE,
                        near: -10.0 * HALF_SIZE,
                        far: 10.0 * HALF_SIZE,
                        ..default()
                    },
                    shadows_enabled: true,
                    ..default()
                },
                ..default()
            },
            Name::new("Light"),
        ))
    }
}
