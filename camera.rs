// ═══════════════════════════════════════════════════════════════════════════
//         NOUVEAU FICHIER: src/systems/camera.rs (À CRÉER)
// ═══════════════════════════════════════════════════════════════════════════

//! Module de contrôle de la caméra
//! 
//! Ce module gère l'interaction avec la caméra :
//! - Rotation avec bouton gauche de la souris
//! - Zoom avec molette
//! - Réinitialisation avec touche R

use bevy::prelude::*;
use bevy::input::mouse::{MouseMotion, MouseWheel};
use crate::config;

/// Composant marker pour identifier la caméra contrôlable
#[derive(Component)]
pub struct OrbitCamera {
    /// Distance actuelle de la caméra par rapport au centre
    pub distance: f32,
    /// Angle horizontal (rotation autour de l'axe Y) en radians
    pub yaw: f32,
    /// Angle vertical (élévation) en radians
    pub pitch: f32,
    /// Point focal (centre de rotation)
    pub focus: Vec3,
    /// Distance minimale de zoom
    pub min_distance: f32,
    /// Distance maximale de zoom
    pub max_distance: f32,
}

impl Default for OrbitCamera {
    fn default() -> Self {
        Self {
            distance: config::CAMERA_DISTANCE,
            yaw: 0.0,
            pitch: config::CAMERA_ANGLE.to_radians(),
            focus: Vec3::ZERO,
            min_distance: 200.0,
            max_distance: 1500.0,
        }
    }
}

/// Système de contrôle de la caméra avec la souris
/// 
/// Contrôles :
/// - Clic gauche + déplacement souris : Rotation (orbite)
/// - Molette : Zoom in/out
/// - Touche R : Réinitialiser la vue
pub fn camera_control_system(
    mouse_button: Res<ButtonInput<MouseButton>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut mouse_motion: EventReader<MouseMotion>,
    mut mouse_wheel: EventReader<MouseWheel>,
    mut query: Query<(&mut OrbitCamera, &mut Transform)>,
) {
    for (mut orbit, mut transform) in query.iter_mut() {
        
        // === ROTATION AVEC SOURIS ===
        // Clic gauche maintenu + déplacement de la souris
        if mouse_button.pressed(MouseButton::Left) {
            for motion in mouse_motion.read() {
                // Sensibilité de la rotation
                let sensitivity = 0.003;
                
                // Mise à jour des angles
                orbit.yaw -= motion.delta.x * sensitivity;
                orbit.pitch -= motion.delta.y * sensitivity;
                
                // Limitation de l'angle vertical (éviter le gimbal lock)
                // On empêche de passer complètement au-dessus ou en-dessous
                orbit.pitch = orbit.pitch.clamp(-1.5, 1.5);
            }
        } else {
            // Vider les événements non utilisés
            mouse_motion.clear();
        }
        
        // === ZOOM AVEC MOLETTE ===
        for wheel in mouse_wheel.read() {
            // Sensibilité du zoom
            let zoom_speed = 20.0;
            
            // Mise à jour de la distance
            orbit.distance -= wheel.y * zoom_speed;
            
            // Limitation du zoom
            orbit.distance = orbit.distance.clamp(
                orbit.min_distance,
                orbit.max_distance
            );
        }
        
        // === RÉINITIALISATION AVEC TOUCHE R ===
        if keyboard.just_pressed(KeyCode::KeyR) {
            orbit.distance = config::CAMERA_DISTANCE;
            orbit.yaw = 0.0;
            orbit.pitch = config::CAMERA_ANGLE.to_radians();
        }
        
        // === CALCUL DE LA NOUVELLE POSITION ===
        // Conversion des coordonnées sphériques en cartésiennes
        let x = orbit.distance * orbit.pitch.cos() * orbit.yaw.sin();
        let y = orbit.distance * orbit.pitch.sin();
        let z = orbit.distance * orbit.pitch.cos() * orbit.yaw.cos();
        
        // Mise à jour de la position de la caméra
        transform.translation = orbit.focus + Vec3::new(x, y, z);
        
        // La caméra regarde toujours vers le point focal
        *transform = transform.looking_at(orbit.focus, Vec3::Y);
    }
}


/// Composant marker pour identifier les objets qui doivent tourner
#[derive(Component)]
pub struct RotatingObject {
    /// Vitesse de rotation en radians par seconde
    pub speed: f32,
}

impl Default for RotatingObject {
    fn default() -> Self {
        Self {
            speed: 0.5,  // 0.5 rad/s ≈ 28.6°/s (rotation complète en ~22 secondes)
        }
    }
}

/// Système qui fait tourner automatiquement les objets marqués
pub fn rotate_object_system(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<RotatingObject>>,
) {
    for mut transform in query.iter_mut() {
        // Rotation autour de l'axe Y (vertical)
        transform.rotate_y(time.delta_secs() * 0.5);
    }
}
