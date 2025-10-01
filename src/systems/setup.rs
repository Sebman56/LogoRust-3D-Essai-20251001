// ═══════════════════════════════════════════════════════════════════════════
//                   FICHIER: src/systems/setup.rs (VERSION BEVY 0.16.1)
// ═══════════════════════════════════════════════════════════════════════════

use bevy::prelude::*;
use crate::{config, materials, geometry};
use crate::systems::camera::{OrbitCamera, RotatingObject};

/// Système principal d'initialisation
pub fn setup_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // === CAMÉRA 3D AVEC CONTRÔLE ORBITAL ===
    let camera_angle_rad = geometry::degrees_to_radians(config::CAMERA_ANGLE);
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(
            0.0,
            -config::CAMERA_DISTANCE * camera_angle_rad.sin(),
            config::CAMERA_DISTANCE * camera_angle_rad.cos(),
        )
        .looking_at(Vec3::ZERO, Vec3::Y),
        OrbitCamera::default(),
    ));

    // === LUMIÈRES ===
    commands.spawn((
        DirectionalLight {
            illuminance: 10000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    commands.spawn((
        PointLight {
            intensity: 500000.0,
            range: 1000.0,
            ..default()
        },
        Transform::from_xyz(0.0, 200.0, 200.0),
    ));

    // === PARENT CONTAINER QUI TOURNE ===
    let parent_id = commands.spawn((
        Transform::default(),
        RotatingObject::default(),
    )).id();

    // Créer tous les enfants et les stocker dans un Vec
    let mut children = Vec::new();
    
    children.append(&mut create_main_circle(&mut commands, &mut meshes, &mut materials));
    children.append(&mut create_exterior_triangles(&mut commands, &mut meshes, &mut materials));
    children.append(&mut create_interior_triangles(&mut commands, &mut meshes, &mut materials));
    children.append(&mut create_r_logo(&mut commands, &mut meshes, &mut materials));
    
    // Attacher tous les enfants au parent
    commands.entity(parent_id).add_children(&children);

    print_creation_summary();
}

// === FONCTIONS DE CRÉATION (RETOURNENT VEC<Entity>) ===

fn create_main_circle(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) -> Vec<Entity> {
    let mesh = geometry::create_3d_ring_mesh(
        config::CIRCLE_RADIUS,
        config::CIRCLE_RADIUS - config::CIRCLE_THICKNESS,
        config::DEPTH,
        config::CIRCLE_SEGMENTS,
    );

    let entity = commands.spawn((
        Mesh3d(meshes.add(mesh)),
        MeshMaterial3d(materials.add(materials::get_main_circle_material())),
        Transform::from_xyz(0.0, 0.0, 0.0),
    )).id();
    
    vec![entity]
}

fn create_exterior_triangles(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) -> Vec<Entity> {
    let mut entities = Vec::new();
    
    for i in 0..config::EXTERIOR_TRIANGLES_COUNT {
        let base_angle = geometry::degrees_to_radians((i as f32) * 10.0);
        let (p1, p2, p3) = geometry::calculate_exterior_triangle_points(
            base_angle,
            config::CIRCLE_RADIUS,
            config::SMALL_TRIANGLE_SIDE,
        );

        let mesh = geometry::create_3d_triangle_mesh(p1, p2, p3, config::DEPTH);

        let entity = commands.spawn((
            Mesh3d(meshes.add(mesh)),
            MeshMaterial3d(materials.add(materials::get_rainbow_material(i))),
            Transform::from_xyz(0.0, 0.0, 0.0),
        )).id();
        
        entities.push(entity);
    }
    
    entities
}

fn create_interior_triangles(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) -> Vec<Entity> {
    let mut entities = Vec::new();
    
    for i in 0..config::INTERIOR_TRIANGLES_COUNT {
        let base_angle = geometry::degrees_to_radians((i as f32) * 72.0 + 90.0);
        let inner_radius = config::CIRCLE_RADIUS - config::CIRCLE_THICKNESS;

        let (p1, p2, p3) = geometry::calculate_interior_triangle_points(
            base_angle,
            inner_radius,
            config::LARGE_TRIANGLE_SIDE,
        );

        let triangle_center = geometry::calculate_triangle_centroid(p1, p2, p3);

        let triangle_entity = commands.spawn((
            Mesh3d(meshes.add(geometry::create_3d_triangle_mesh(p1, p2, p3, config::DEPTH))),
            MeshMaterial3d(materials.add(materials::get_interior_triangle_material(i))),
            Transform::from_xyz(0.0, 0.0, 0.0),
        )).id();
        
        entities.push(triangle_entity);

        let circle_entity = commands.spawn((
            Mesh3d(meshes.add(geometry::create_3d_cylinder_mesh(
                config::SMALL_CIRCLE_RADIUS,
                config::DEPTH,
                config::SMALL_CIRCLE_SEGMENTS,
            ))),
            MeshMaterial3d(materials.add(materials::get_small_circle_material())),
            Transform::from_xyz(triangle_center.x, triangle_center.y, config::DEPTH / 2.0),
        )).id();
        
        entities.push(circle_entity);
    }
    
    entities
}

fn create_r_logo(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) -> Vec<Entity> {
    let mut entities = Vec::new();
    let r_material = materials.add(materials::get_r_logo_material());

    for part in geometry::get_all_r_parts() {
        if part.points.len() < 3 {
            continue;
        }

        let entity = commands.spawn((
            Mesh3d(meshes.add(geometry::create_3d_polygon_mesh(&part.points, config::DEPTH))),
            MeshMaterial3d(r_material.clone()),
            Transform::from_xyz(0.0, 0.0, 0.0),
        )).id();
        
        entities.push(entity);
    }
    
    entities
}

fn print_creation_summary() {
    println!("\n╔═══════════════════════════════════════════════════════════╗");
    println!("║          LOGO 3D AVEC ROTATION AUTOMATIQUE                ║");
    println!("╚═══════════════════════════════════════════════════════════╝");
    println!("\n🎮 CONTRÔLES :");
    println!("   • Clic gauche + souris : Rotation caméra");
    println!("   • Molette : Zoom");
    println!("   • Touche R : Réinitialiser vue");
    println!("   • L'objet tourne automatiquement");
    println!("\n╚═══════════════════════════════════════════════════════════╝\n");
}
