use bevy::{
    prelude::*,
    render::{mesh::Indices, pipeline::PrimitiveTopology},
};

use crate::{mesh_helper::MeshEditXC, GizmoMeshBundle};

#[derive(Default)]
pub struct Line {
    entity: Option<Entity>,
    mesh_handle: Handle<Mesh>,
}

impl Line {
    pub fn new(commands: &mut Commands, meshes: &mut Assets<Mesh>) -> Self {
        let mesh_handle = {
            let mut mesh = Mesh::new(PrimitiveTopology::LineList);
            mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, Vec::<[f32; 3]>::with_capacity(32));
            mesh.set_attribute(Mesh::ATTRIBUTE_COLOR, Vec::<[f32; 4]>::with_capacity(32));
            mesh.set_indices(Some(Indices::U32(Vec::with_capacity(32))));
            meshes.add(mesh)
        };

        Self {
            entity: commands
                .spawn(GizmoMeshBundle {
                    mesh: mesh_handle.clone(),
                    material: Color::WHITE.into(),
                    ..Default::default()
                })
                .current_entity(),
            mesh_handle,
        }
    }

    pub fn edit<'a>(&self, meshes: &'a mut Assets<Mesh>) -> MeshEditXC<'a> {
        let mesh = meshes.get_mut(&self.mesh_handle).unwrap();
        MeshEditXC::from(mesh)
    }

    // TODO: Dispose line and his mesh custom warning on drop;
    // pub fn dispose(self, commands: &mut Commands, meshes: &mut Assets<Mesh>) -> Self {
    //     commands.despawn(self.entity.take())
    // }
}
