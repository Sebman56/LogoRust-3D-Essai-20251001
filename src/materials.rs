
// ╔══════════════════════════════════════════════════════════════════════════╗
// ║                      FICHIER: src/materials.rs                           ║
// ╚══════════════════════════════════════════════════════════════════════════╝

use bevy::prelude::*;

/// Matériau 3D pour le cercle principal
pub fn get_main_circle_material() -> StandardMaterial {
    StandardMaterial {
        base_color: Color::srgb(0.8, 0.2, 0.1),
        metallic: 0.3,
        perceptual_roughness: 0.5,
        ..default()
    }
}

/// Matériau arc-en-ciel pour les triangles extérieurs
pub fn get_rainbow_material(index: usize) -> StandardMaterial {
    let hue = (index as f32 * 10.0) / 360.0 * 360.0;
    StandardMaterial {
        base_color: Color::hsl(hue, 0.8, 0.6),
        metallic: 0.2,
        perceptual_roughness: 0.6,
        ..default()
    }
}

/// Matériaux pour les triangles intérieurs
pub fn get_interior_triangle_material(index: usize) -> StandardMaterial {
    let colors = [
        Color::srgb(0.2, 0.6, 0.9),
        Color::srgb(0.9, 0.6, 0.2),
        Color::srgb(0.2, 0.9, 0.6),
        Color::srgb(0.9, 0.2, 0.6),
        Color::srgb(0.6, 0.2, 0.9),
    ];
    StandardMaterial {
        base_color: colors[index % colors.len()],
        metallic: 0.3,
        perceptual_roughness: 0.5,
        ..default()
    }
}

/// Matériau pour les petits cercles
pub fn get_small_circle_material() -> StandardMaterial {
    StandardMaterial {
        base_color: Color::srgba(1.0, 1.0, 1.0, 0.9),
        metallic: 0.4,
        perceptual_roughness: 0.3,
        alpha_mode: AlphaMode::Blend,
        ..default()
    }
}

/// Matériau pour le logo R
pub fn get_r_logo_material() -> StandardMaterial {
    StandardMaterial {
        base_color: Color::srgb(1.0, 0.5, 0.0),
        metallic: 0.5,
        perceptual_roughness: 0.4,
        ..default()
    }
}
