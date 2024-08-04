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
        .run();
}
