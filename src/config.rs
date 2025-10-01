
// ╔══════════════════════════════════════════════════════════════════════════╗
// ║                        FICHIER: src/config.rs                            ║
// ╚══════════════════════════════════════════════════════════════════════════╗

/// Rayon du cercle principal
pub const CIRCLE_RADIUS: f32 = 200.0;

/// Épaisseur de l'anneau du cercle
pub const CIRCLE_THICKNESS: f32 = 30.0;

/// Qualité des cercles (segments)
pub const CIRCLE_SEGMENTS: usize = 64;

/// Nombre de triangles extérieurs
pub const EXTERIOR_TRIANGLES_COUNT: usize = 36;

/// Taille des petits triangles extérieurs
pub const SMALL_TRIANGLE_SIDE: f32 = 25.0;

/// Nombre de triangles intérieurs
pub const INTERIOR_TRIANGLES_COUNT: usize = 5;

/// Taille des grands triangles intérieurs
pub const LARGE_TRIANGLE_SIDE: f32 = 80.0;

/// Rayon des petits cercles
pub const SMALL_CIRCLE_RADIUS: f32 = 15.0;

/// Qualité des petits cercles
pub const SMALL_CIRCLE_SEGMENTS: usize = 32;

// === NOUVEAUX PARAMÈTRES 3D ===

/// Épaisseur de tous les éléments en 3D (profondeur)
pub const DEPTH: f32 = 10.0;

/// Distance de la caméra
pub const CAMERA_DISTANCE: f32 = 500.0;

/// Angle de la caméra en degrés
pub const CAMERA_ANGLE: f32 = 25.0;
