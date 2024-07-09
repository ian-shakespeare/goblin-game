use bevy::{
    prelude::*,
    render::{
        mesh::PrimitiveTopology,
        render_asset::RenderAssetUsages,
        render_resource::{AsBindGroup, PolygonMode},
    },
};

#[derive(Component)]
pub struct Player;

#[derive(Component, Default)]
pub struct PlayerCam;

#[derive(Component)]
pub struct HitPoints;

#[derive(Default, Resource, Deref, DerefMut, Debug)]
pub struct MovementInput(Vec3);

#[derive(Default, Resource, Deref, DerefMut)]
pub struct LookInput(Vec2);

#[derive(Default, Resource, Deref, DerefMut)]
pub struct PlayerActions {
    pub shoot: bool,
}

#[derive(Asset, TypePath, Default, AsBindGroup, Debug, Clone)]
pub struct LineMaterial {
    #[uniform(0)]
    pub color: LinearRgba,
}

impl Material for LineMaterial {
    fn fragment_shader() -> bevy::render::render_resource::ShaderRef {
        "shaders/line_material.wgsl".into()
    }

    fn specialize(
        _pipeline: &bevy::pbr::MaterialPipeline<Self>,
        descriptor: &mut bevy::render::render_resource::RenderPipelineDescriptor,
        _layout: &bevy::render::mesh::MeshVertexBufferLayoutRef,
        _key: bevy::pbr::MaterialPipelineKey<Self>,
    ) -> Result<(), bevy::render::render_resource::SpecializedMeshPipelineError> {
        descriptor.primitive.polygon_mode = PolygonMode::Line;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct LineStrip {
    pub points: Vec<Vec3>,
}

impl From<LineStrip> for Mesh {
    fn from(line: LineStrip) -> Self {
        Mesh::new(
            PrimitiveTopology::LineStrip,
            RenderAssetUsages::RENDER_WORLD,
        )
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, line.points)
    }
}
