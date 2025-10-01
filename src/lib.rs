
// ═══════════════════════════════════════════════════════════════════════════
//                    FICHIER: src/lib.rs (MODIFIÉ)
// ═══════════════════════════════════════════════════════════════════════════

use bevy::prelude::*;

pub mod config;
pub mod materials;
pub mod geometry;
pub mod systems;

use systems::setup::setup_system;
use systems::camera::{camera_control_system, rotate_object_system};  // MODIFIÉ


pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup_system)
        .add_systems(Update, (camera_control_system,rotate_object_system,))  // NOUVEAU : contrôle souris
        .run();
}
