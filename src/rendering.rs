use bevy::prelude::*;
use crate::config::*;
use crate::world::{VoxelWorld, VoxelPos};

/// Component to mark the world mesh
#[derive(Component)]
pub struct WorldMesh;

/// Resource to track when world needs re-meshing
#[derive(Resource)]
pub struct RenderState {
    pub needs_update: bool,
    pub update_timer: Timer,
}

impl Default for RenderState {
    fn default() -> Self {
        Self {
            needs_update: true,
            update_timer: Timer::from_seconds(0.1, TimerMode::Repeating),
        }
    }
}

/// Setup rendering
pub fn setup_rendering(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    world: Res<VoxelWorld>,
) {
    // Create initial mesh
    let mesh = create_world_mesh(&world);
    let mesh_handle = meshes.add(mesh);

    let material = materials.add(StandardMaterial {
        base_color: Color::WHITE,
        perceptual_roughness: 0.8,
        // Enable vertex colors so voxels show their actual colors
        alpha_mode: AlphaMode::Opaque,
        ..default()
    });

    commands.spawn((
        Mesh3d(mesh_handle),
        MeshMaterial3d(material),
        WorldMesh,
    ));

    // Add lighting
    commands.spawn((
        DirectionalLight {
            illuminance: 10000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(50.0, 100.0, 50.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // Ambient light
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 300.0,
        affects_lightmapped_meshes: false,
    });
}

/// Update mesh when world changes
pub fn update_world_mesh_system(
    mut state: ResMut<RenderState>,
    world: Res<VoxelWorld>,
    mut meshes: ResMut<Assets<Mesh>>,
    query: Query<&Mesh3d, With<WorldMesh>>,
    time: Res<Time>,
) {
    state.update_timer.tick(time.delta());

    if !state.update_timer.just_finished() {
        return;
    }

    state.needs_update = true;

    if state.needs_update {
        for mesh_handle in query.iter() {
            if let Some(mesh) = meshes.get_mut(&mesh_handle.0) {
                *mesh = create_world_mesh(&world);
            }
        }
        state.needs_update = false;
    }
}

/// Create a mesh from the voxel world
fn create_world_mesh(world: &VoxelWorld) -> Mesh {
    let mut positions = Vec::new();
    let mut normals = Vec::new();
    let mut uvs = Vec::new();
    let mut indices = Vec::new();
    let mut colors = Vec::new();

    // Iterate through all voxels and create faces for solid ones
    for pos in world.iter_positions().collect::<Vec<_>>() {
        if let Some(voxel) = world.get(&pos) {
            if !voxel.voxel_type.is_solid() {
                continue;
            }

            let color = voxel.voxel_type.get_color();
            let world_pos = pos.to_world_pos();

            // Check each face
            add_voxel_faces(
                &pos,
                &world_pos,
                &color,
                world,
                &mut positions,
                &mut normals,
                &mut uvs,
                &mut colors,
                &mut indices,
            );
        }
    }

    // Create mesh from collected voxel data
    if positions.is_empty() {
        // If no voxels, return a simple ground plane
        return Mesh::from(Plane3d::default().mesh().size(
            WORLD_WIDTH as f32 * VOXEL_SIZE,
            WORLD_DEPTH as f32 * VOXEL_SIZE,
        ));
    }

    // Start with a plane mesh and replace its data
    let mut mesh = Mesh::from(Plane3d::default().mesh().size(1.0, 1.0));

    // Replace all attributes with our voxel data
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);

    // Set indices
    mesh.insert_indices(bevy::mesh::Indices::U32(indices));

    mesh
}

/// Add faces for a voxel
fn add_voxel_faces(
    pos: &VoxelPos,
    world_pos: &Vec3,
    color: &Color,
    world: &VoxelWorld,
    positions: &mut Vec<[f32; 3]>,
    normals: &mut Vec<[f32; 3]>,
    uvs: &mut Vec<[f32; 2]>,
    colors: &mut Vec<[f32; 4]>,
    indices: &mut Vec<u32>,
) {
    let s = VOXEL_SIZE / 2.0;
    let c = [color.to_srgba().red, color.to_srgba().green, color.to_srgba().blue, color.to_srgba().alpha];

    // Check each direction and add face if neighbor is empty
    let neighbors = pos.neighbors();

    // Top face (counter-clockwise when viewed from above)
    if should_render_face(&neighbors[2], world) {
        let base = positions.len() as u32;
        positions.extend_from_slice(&[
            [world_pos.x - s, world_pos.y + s, world_pos.z + s],
            [world_pos.x + s, world_pos.y + s, world_pos.z + s],
            [world_pos.x + s, world_pos.y + s, world_pos.z - s],
            [world_pos.x - s, world_pos.y + s, world_pos.z - s],
        ]);
        normals.extend_from_slice(&[[0.0, 1.0, 0.0]; 4]);
        uvs.extend_from_slice(&[[0.0, 1.0], [1.0, 1.0], [1.0, 0.0], [0.0, 0.0]]);
        colors.extend_from_slice(&[c; 4]);
        indices.extend_from_slice(&[base, base + 1, base + 2, base, base + 2, base + 3]);
    }

    // Bottom face (counter-clockwise when viewed from below)
    if should_render_face(&neighbors[3], world) {
        let base = positions.len() as u32;
        positions.extend_from_slice(&[
            [world_pos.x - s, world_pos.y - s, world_pos.z - s],
            [world_pos.x + s, world_pos.y - s, world_pos.z - s],
            [world_pos.x + s, world_pos.y - s, world_pos.z + s],
            [world_pos.x - s, world_pos.y - s, world_pos.z + s],
        ]);
        normals.extend_from_slice(&[[0.0, -1.0, 0.0]; 4]);
        uvs.extend_from_slice(&[[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]]);
        colors.extend_from_slice(&[c; 4]);
        indices.extend_from_slice(&[base, base + 1, base + 2, base, base + 2, base + 3]);
    }

    // Front face (+Z) (counter-clockwise when viewed from front)
    if should_render_face(&neighbors[1], world) {
        let base = positions.len() as u32;
        positions.extend_from_slice(&[
            [world_pos.x - s, world_pos.y - s, world_pos.z + s],
            [world_pos.x + s, world_pos.y - s, world_pos.z + s],
            [world_pos.x + s, world_pos.y + s, world_pos.z + s],
            [world_pos.x - s, world_pos.y + s, world_pos.z + s],
        ]);
        normals.extend_from_slice(&[[0.0, 0.0, 1.0]; 4]);
        uvs.extend_from_slice(&[[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]]);
        colors.extend_from_slice(&[c; 4]);
        indices.extend_from_slice(&[base, base + 1, base + 2, base, base + 2, base + 3]);
    }

    // Back face (-Z) (counter-clockwise when viewed from back)
    if should_render_face(&neighbors[0], world) {
        let base = positions.len() as u32;
        positions.extend_from_slice(&[
            [world_pos.x - s, world_pos.y - s, world_pos.z - s],
            [world_pos.x + s, world_pos.y - s, world_pos.z - s],
            [world_pos.x + s, world_pos.y + s, world_pos.z - s],
            [world_pos.x - s, world_pos.y + s, world_pos.z - s],
        ]);
        normals.extend_from_slice(&[[0.0, 0.0, -1.0]; 4]);
        uvs.extend_from_slice(&[[1.0, 0.0], [0.0, 0.0], [0.0, 1.0], [1.0, 1.0]]);
        colors.extend_from_slice(&[c; 4]);
        indices.extend_from_slice(&[base, base + 1, base + 2, base, base + 2, base + 3]);
    }

    // Right face (+X) (counter-clockwise when viewed from right)
    if should_render_face(&neighbors[5], world) {
        let base = positions.len() as u32;
        positions.extend_from_slice(&[
            [world_pos.x + s, world_pos.y - s, world_pos.z + s],
            [world_pos.x + s, world_pos.y - s, world_pos.z - s],
            [world_pos.x + s, world_pos.y + s, world_pos.z - s],
            [world_pos.x + s, world_pos.y + s, world_pos.z + s],
        ]);
        normals.extend_from_slice(&[[1.0, 0.0, 0.0]; 4]);
        uvs.extend_from_slice(&[[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]]);
        colors.extend_from_slice(&[c; 4]);
        indices.extend_from_slice(&[base, base + 1, base + 2, base, base + 2, base + 3]);
    }

    // Left face (-X) (counter-clockwise when viewed from left)
    if should_render_face(&neighbors[4], world) {
        let base = positions.len() as u32;
        positions.extend_from_slice(&[
            [world_pos.x - s, world_pos.y - s, world_pos.z - s],
            [world_pos.x - s, world_pos.y - s, world_pos.z + s],
            [world_pos.x - s, world_pos.y + s, world_pos.z + s],
            [world_pos.x - s, world_pos.y + s, world_pos.z - s],
        ]);
        normals.extend_from_slice(&[[-1.0, 0.0, 0.0]; 4]);
        uvs.extend_from_slice(&[[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]]);
        colors.extend_from_slice(&[c; 4]);
        indices.extend_from_slice(&[base, base + 1, base + 2, base, base + 2, base + 3]);
    }
}

fn should_render_face(neighbor_pos: &VoxelPos, world: &VoxelWorld) -> bool {
    if let Some(voxel) = world.get(neighbor_pos) {
        voxel.voxel_type.is_air()
    } else {
        true // Render if outside bounds
    }
}
