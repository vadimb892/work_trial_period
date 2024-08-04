mod schedule;
mod asset_loader;
mod camera;
mod asteroids;
mod spaceship;
mod movement;
mod collision_detection;
mod despawn;
mod health;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((
            schedule::SchedulePlugin,
            asset_loader::AssetLoadPlugin,
            camera::CameraPlugin,
            asteroids::AsteroidsPlugin,
            spaceship::SpaceshipPlugin,
            movement::MovementPlugin,
            collision_detection::CollisionDetectionPlugin,
            despawn::DespawnPlugin,
            health::HealthPlugin
        ))
        .run();
}
