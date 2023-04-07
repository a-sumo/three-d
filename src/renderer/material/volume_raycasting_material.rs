use crate::core::*;
use crate::renderer::*;

///
/// A material that renders the volume data in the [VolumeRaycastingMaterial::voxels] using volume raycasting.
/// This material should be applied to a cube with center in origo, for example [CpuMesh::cube].
///
#[derive(Clone)]
pub struct VolumeRaycastingMaterial {
    /// The voxel data that defines the volume.
    pub voxels: std::sync::Arc<Texture3D>,
    /// The size of the cube that is used to render the voxel data. The texture is scaled to fill the entire cube.
    pub size: Vec3,
}

impl Material for VolumeRaycastingMaterial {
    fn fragment_shader(&self) -> FragmentShader {
        FragmentShader {
            source: include_str!("shaders/volume_raycasting_material.frag").to_string(),
            attributes: FragmentAttributes {
                position: true,
                ..FragmentAttributes::NONE
            },
        }
    }

    fn use_uniforms(&self, program: &Program, camera: &Camera) {
        program.use_uniform("cameraPosition", camera.position());
        program.use_uniform("size", self.size);
        program.use_texture_3d("tex", &self.voxels);
    }

    fn render_states(&self) -> RenderStates {
        RenderStates {
            blend: Blend::TRANSPARENCY,
            ..Default::default()
        }
    }

    fn material_type(&self) -> MaterialType {
        MaterialType::Transparent
    }
}

impl FromCpuVoxelGrid for VolumeRaycastingMaterial {
    fn from_cpu_voxel_grid(context: &Context, cpu_voxel_grid: &CpuVoxelGrid) -> Self {
        Self {
            voxels: std::sync::Arc::new(Texture3D::new(context, &cpu_voxel_grid.voxels)),
            size: cpu_voxel_grid.size,
        }
    }
}
