
// ╔══════════════════════════════════════════════════════════════════════════╗
// ║                      FICHIER: src/geometry.rs                            ║
// ╚══════════════════════════════════════════════════════════════════════════╝

use bevy::prelude::*;
use bevy::render::mesh::{Indices, PrimitiveTopology};
use std::f32::consts::PI;

/// Convertit degrés en radians
pub fn degrees_to_radians(degrees: f32) -> f32 {
    degrees * PI / 180.0
}

/// Crée un anneau 3D avec épaisseur
pub fn create_3d_ring_mesh(
    outer_radius: f32,
    inner_radius: f32,
    depth: f32,
    segments: usize,
) -> Mesh {
    let mut positions = Vec::new();
    let mut normals = Vec::new();
    let mut indices: Vec<u32> = Vec::new();  // CORRECTION: Vec<u32> au lieu de Vec::new()

    // Pour chaque segment
    for i in 0..=segments {
        let angle = 2.0 * PI * i as f32 / segments as f32;
        let cos = angle.cos();
        let sin = angle.sin();

        // Face avant (z = depth/2)
        positions.push([outer_radius * cos, outer_radius * sin, depth / 2.0]);
        normals.push([0.0, 0.0, 1.0]);
        
        positions.push([inner_radius * cos, inner_radius * sin, depth / 2.0]);
        normals.push([0.0, 0.0, 1.0]);

        // Face arrière (z = -depth/2)
        positions.push([outer_radius * cos, outer_radius * sin, -depth / 2.0]);
        normals.push([0.0, 0.0, -1.0]);
        
        positions.push([inner_radius * cos, inner_radius * sin, -depth / 2.0]);
        normals.push([0.0, 0.0, -1.0]);
    }

    // Indices pour les faces avant et arrière
    for i in 0..segments {
        let base = (i * 4) as u32;  // CORRECTION: cast en u32
        
        // Face avant
        indices.extend_from_slice(&[
            base, base + 1, base + 4,
            base + 4, base + 1, base + 5,
        ]);
        
        // Face arrière
        indices.extend_from_slice(&[
            base + 2, base + 6, base + 3,
            base + 3, base + 6, base + 7,
        ]);
        
        // Bord extérieur
        indices.extend_from_slice(&[
            base, base + 4, base + 2,
            base + 2, base + 4, base + 6,
        ]);
        
        // Bord intérieur
        indices.extend_from_slice(&[
            base + 1, base + 3, base + 5,
            base + 5, base + 3, base + 7,
        ]);
    }

    Mesh::new(PrimitiveTopology::TriangleList, Default::default())
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
        .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
        .with_inserted_indices(Indices::U32(indices))
}

/// Crée un cylindre 3D (cercle avec épaisseur)
pub fn create_3d_cylinder_mesh(radius: f32, depth: f32, segments: usize) -> Mesh {
    let mut positions = Vec::new();
    let mut normals = Vec::new();
    let mut indices: Vec<u32> = Vec::new();  // CORRECTION: Vec<u32>

    // Centre face avant
    positions.push([0.0, 0.0, depth / 2.0]);
    normals.push([0.0, 0.0, 1.0]);

    // Centre face arrière
    positions.push([0.0, 0.0, -depth / 2.0]);
    normals.push([0.0, 0.0, -1.0]);

    // Points du contour (avant et arrière)
    for i in 0..=segments {
        let angle = 2.0 * PI * i as f32 / segments as f32;
        let x = radius * angle.cos();
        let y = radius * angle.sin();

        // Face avant
        positions.push([x, y, depth / 2.0]);
        normals.push([0.0, 0.0, 1.0]);

        // Face arrière
        positions.push([x, y, -depth / 2.0]);
        normals.push([0.0, 0.0, -1.0]);
    }

    // Triangles face avant
    for i in 0..segments {
        let front_idx = (2 + i * 2) as u32;  // CORRECTION: cast en u32
        indices.extend_from_slice(&[0, front_idx, front_idx + 2]);
    }

    // Triangles face arrière
    for i in 0..segments {
        let back_idx = (3 + i * 2) as u32;  // CORRECTION: cast en u32
        indices.extend_from_slice(&[1, back_idx + 2, back_idx]);
    }

    // Bords latéraux
    for i in 0..segments {
        let front = (2 + i * 2) as u32;  // CORRECTION: cast en u32
        let back = (3 + i * 2) as u32;   // CORRECTION: cast en u32
        indices.extend_from_slice(&[
            front, front + 2, back,
            back, front + 2, back + 2,
        ]);
    }

    Mesh::new(PrimitiveTopology::TriangleList, Default::default())
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
        .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
        .with_inserted_indices(Indices::U32(indices))
}

/// Crée un prisme triangulaire 3D
pub fn create_3d_triangle_mesh(p1: Vec2, p2: Vec2, p3: Vec2, depth: f32) -> Mesh {
    let half_depth = depth / 2.0;

    let positions = vec![
        // Face avant
        [p1.x, p1.y, half_depth],
        [p2.x, p2.y, half_depth],
        [p3.x, p3.y, half_depth],
        // Face arrière
        [p1.x, p1.y, -half_depth],
        [p2.x, p2.y, -half_depth],
        [p3.x, p3.y, -half_depth],
    ];

    let indices = vec![
        // Face avant
        0, 1, 2,
        // Face arrière
        3, 5, 4,
        // Côté 1
        0, 3, 1, 1, 3, 4,
        // Côté 2
        1, 4, 2, 2, 4, 5,
        // Côté 3
        2, 5, 0, 0, 5, 3,
    ];

    // Calcul des normales
    let normal_front = [0.0, 0.0, 1.0];
    let normal_back = [0.0, 0.0, -1.0];

    let normals = vec![
        normal_front, normal_front, normal_front,
        normal_back, normal_back, normal_back,
    ];

    Mesh::new(PrimitiveTopology::TriangleList, Default::default())
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
        .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
        .with_inserted_indices(Indices::U32(indices))
}

/// Crée un polygone extrudé en 3D
pub fn create_3d_polygon_mesh(points: &[Vec2], depth: f32) -> Mesh {
    if points.len() < 3 {
        panic!("Besoin de minimum 3 points");
    }

    let half_depth = depth / 2.0;
    let mut positions = Vec::new();
    let mut normals = Vec::new();
    let mut indices = Vec::new();

    // Vertices face avant
    for point in points {
        positions.push([point.x, point.y, half_depth]);
        normals.push([0.0, 0.0, 1.0]);
    }

    // Vertices face arrière
    for point in points {
        positions.push([point.x, point.y, -half_depth]);
        normals.push([0.0, 0.0, -1.0]);
    }

    let n = points.len() as u32;

    // Triangulation face avant
    for i in 1..(n - 1) {
        indices.extend_from_slice(&[0, i, i + 1]);
    }

    // Triangulation face arrière
    for i in 1..(n - 1) {
        indices.extend_from_slice(&[n, n + i + 1, n + i]);
    }

    // Bords latéraux
    for i in 0..n {
        let next = (i + 1) % n;
        indices.extend_from_slice(&[
            i, next, i + n,
            i + n, next, next + n,
        ]);
    }

    Mesh::new(PrimitiveTopology::TriangleList, Default::default())
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
        .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
        .with_inserted_indices(Indices::U32(indices))
}

/// Calcule les points d'un triangle extérieur
pub fn calculate_exterior_triangle_points(
    base_angle: f32,
    circle_radius: f32,
    triangle_side: f32,
) -> (Vec2, Vec2, Vec2) {
    let half_side = triangle_side / 2.0;
    let height = triangle_side * (3.0_f32.sqrt() / 2.0);
    
    let angle1 = base_angle - (half_side / circle_radius);
    let angle2 = base_angle + (half_side / circle_radius);
    
    let p1 = Vec2::new(circle_radius * angle1.cos(), circle_radius * angle1.sin());
    let p2 = Vec2::new(circle_radius * angle2.cos(), circle_radius * angle2.sin());
    let p3 = Vec2::new(
        (circle_radius + height) * base_angle.cos(),
        (circle_radius + height) * base_angle.sin(),
    );
    
    (p1, p2, p3)
}

/// Calcule les points d'un triangle intérieur
pub fn calculate_interior_triangle_points(
    base_angle: f32,
    inner_radius: f32,
    triangle_side: f32,
) -> (Vec2, Vec2, Vec2) {
    let half_side = triangle_side / 2.0;
    let height = triangle_side * (3.0_f32.sqrt() / 2.0);
    
    let angle1 = base_angle - (half_side / inner_radius);
    let angle2 = base_angle + (half_side / inner_radius);
    
    let p1 = Vec2::new(inner_radius * angle1.cos(), inner_radius * angle1.sin());
    let p2 = Vec2::new(inner_radius * angle2.cos(), inner_radius * angle2.sin());
    let p3 = Vec2::new(
        (inner_radius - height) * base_angle.cos(),
        (inner_radius - height) * base_angle.sin(),
    );
    
    (p1, p2, p3)
}

/// Calcule le centroïde d'un triangle
pub fn calculate_triangle_centroid(p1: Vec2, p2: Vec2, p3: Vec2) -> Vec2 {
    Vec2::new(
        (p1.x + p2.x + p3.x) / 3.0,
        (p1.y + p2.y + p3.y) / 3.0,
    )
}

/// Définition d'une partie du logo R
#[derive(Clone)]
pub struct RPartDefinition {
    pub name: &'static str,
    pub points: Vec<Vec2>,
}

/// Retourne toutes les parties du logo R
pub fn get_all_r_parts() -> Vec<RPartDefinition> {
    vec![
        RPartDefinition {
            name: "Haut du R",
            points: vec![
                Vec2::new(-140.0, 90.0),
                Vec2::new(60.0, 90.0),
                Vec2::new(60.0, 50.0),
                Vec2::new(-100.0, 50.0),
            ],
        },
        RPartDefinition {
            name: "Gauche du R",
            points: vec![
                Vec2::new(-80.0, 50.0),
                Vec2::new(-30.0, 50.0),
                Vec2::new(-30.0, -50.0),
                Vec2::new(-80.0, -50.0),
            ],
        },
        RPartDefinition {
            name: "Arrondi du R",
            points: vec![
                Vec2::new(60.0, 90.0),
                Vec2::new(85.0, 60.0),
                Vec2::new(100.0, 30.0),
                Vec2::new(85.0, 0.0),
                Vec2::new(60.0, -30.0),
            ],
        },
        RPartDefinition {
            name: "Centre du R",
            points: vec![
                Vec2::new(60.0, 50.0),
                Vec2::new(40.0, 50.0),
                Vec2::new(60.0, 10.0),
                Vec2::new(40.0, 10.0),
            ],
        },
        RPartDefinition {
            name: "Pied gauche du R",
            points: vec![
                Vec2::new(-80.0, -50.0),
                Vec2::new(-10.0, -50.0),
                Vec2::new(-10.0, -80.0),
                Vec2::new(-140.0, -80.0),
                Vec2::new(-160.0, -50.0),
            ],
        },
        RPartDefinition {
            name: "Milieu du R",
            points: vec![
                Vec2::new(60.0, -30.0),
                Vec2::new(60.0, 10.0),
                Vec2::new(-30.0, 10.0),
                Vec2::new(-30.0, -30.0),
            ],
        },
        RPartDefinition {
            name: "Jambe droite du R",
            points: vec![
                Vec2::new(60.0, -30.0),
                Vec2::new(20.0, -30.0),
                Vec2::new(60.0, -50.0),
                Vec2::new(100.0, -50.0),
            ],
        },
        RPartDefinition {
            name: "Pied droit du R",
            points: vec![
                Vec2::new(160.0, -50.0),
                Vec2::new(30.0, -50.0),
                Vec2::new(30.0, -80.0),
                Vec2::new(120.0, -80.0),
            ],
        },
    ]
}
