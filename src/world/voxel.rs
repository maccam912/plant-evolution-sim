use bevy::prelude::*;
use crate::config::*;

/// Represents the type of material in a voxel
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VoxelType {
    Air,
    Soil,
    PlantMaterial { plant_id: u32 },
}

impl VoxelType {
    pub fn is_solid(&self) -> bool {
        !matches!(self, VoxelType::Air)
    }

    pub fn is_air(&self) -> bool {
        matches!(self, VoxelType::Air)
    }

    pub fn is_plant(&self) -> bool {
        matches!(self, VoxelType::PlantMaterial { .. })
    }

    pub fn get_color(&self) -> Color {
        match self {
            VoxelType::Air => Color::srgba(0.0, 0.0, 0.0, 0.0),
            VoxelType::Soil => Color::srgb(0.4, 0.3, 0.2),
            VoxelType::PlantMaterial { .. } => Color::srgb(0.2, 0.8, 0.3),
        }
    }
}

/// Environmental data for each voxel
#[derive(Debug, Clone, Copy)]
pub struct VoxelEnvironment {
    pub light_level: f32,
    pub nutrients: f32,
    pub water: f32,
}

impl Default for VoxelEnvironment {
    fn default() -> Self {
        Self {
            light_level: 0.0,
            nutrients: SOIL_NUTRIENT_MAX,
            water: SOIL_WATER_MAX,
        }
    }
}

/// Complete voxel data
#[derive(Debug, Clone, Copy)]
pub struct Voxel {
    pub voxel_type: VoxelType,
    pub environment: VoxelEnvironment,
}

impl Default for Voxel {
    fn default() -> Self {
        Self {
            voxel_type: VoxelType::Air,
            environment: VoxelEnvironment::default(),
        }
    }
}

/// 3D coordinate in the world grid
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub struct VoxelPos {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl VoxelPos {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    pub fn to_world_pos(&self) -> Vec3 {
        Vec3::new(
            self.x as f32 * VOXEL_SIZE,
            self.y as f32 * VOXEL_SIZE,
            self.z as f32 * VOXEL_SIZE,
        )
    }

    pub fn neighbors(&self) -> [VoxelPos; 6] {
        [
            VoxelPos::new(self.x + 1, self.y, self.z),
            VoxelPos::new(self.x - 1, self.y, self.z),
            VoxelPos::new(self.x, self.y + 1, self.z),
            VoxelPos::new(self.x, self.y - 1, self.z),
            VoxelPos::new(self.x, self.y, self.z + 1),
            VoxelPos::new(self.x, self.y, self.z - 1),
        ]
    }
}

/// The main voxel world grid
#[derive(Resource)]
pub struct VoxelWorld {
    voxels: Vec<Voxel>,
    width: usize,
    height: usize,
    depth: usize,
}

impl VoxelWorld {
    pub fn new(width: usize, height: usize, depth: usize) -> Self {
        let total_voxels = width * height * depth;
        let mut voxels = vec![Voxel::default(); total_voxels];

        // Initialize soil in lower half
        for y in 0..(height / 2) {
            for x in 0..width {
                for z in 0..depth {
                    let idx = Self::pos_to_index(x, y, z, width, depth);
                    voxels[idx].voxel_type = VoxelType::Soil;
                }
            }
        }

        Self {
            voxels,
            width,
            height,
            depth,
        }
    }

    fn pos_to_index(x: usize, y: usize, z: usize, width: usize, depth: usize) -> usize {
        x + z * width + y * width * depth
    }

    pub fn get(&self, pos: &VoxelPos) -> Option<&Voxel> {
        if !self.is_in_bounds(pos) {
            return None;
        }
        let idx = Self::pos_to_index(
            pos.x as usize,
            pos.y as usize,
            pos.z as usize,
            self.width,
            self.depth,
        );
        self.voxels.get(idx)
    }

    pub fn get_mut(&mut self, pos: &VoxelPos) -> Option<&mut Voxel> {
        if !self.is_in_bounds(pos) {
            return None;
        }
        let idx = Self::pos_to_index(
            pos.x as usize,
            pos.y as usize,
            pos.z as usize,
            self.width,
            self.depth,
        );
        self.voxels.get_mut(idx)
    }

    pub fn is_in_bounds(&self, pos: &VoxelPos) -> bool {
        pos.x >= 0
            && pos.y >= 0
            && pos.z >= 0
            && (pos.x as usize) < self.width
            && (pos.y as usize) < self.height
            && (pos.z as usize) < self.depth
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn depth(&self) -> usize {
        self.depth
    }

    /// Iterate over all voxel positions
    pub fn iter_positions(&self) -> impl Iterator<Item = VoxelPos> + '_ {
        (0..self.height).flat_map(move |y| {
            (0..self.depth).flat_map(move |z| {
                (0..self.width).map(move |x| VoxelPos::new(x as i32, y as i32, z as i32))
            })
        })
    }
}
