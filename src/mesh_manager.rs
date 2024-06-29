use crate::mesh::Mesh;
use std::collections::HashMap;

type MeshId = u32;

pub struct MeshManager {
    meshes: HashMap<MeshId, Mesh>,
    mesh_count: usize,
}

impl MeshManager {
    pub fn new() -> Self {
        Self {
            meshes: HashMap::new(),
            mesh_count: 0,
        }
    }

    pub fn add_mesh(&mut self, mesh: Mesh) -> MeshId {
        let mesh_id = self.mesh_count as u32;
        self.meshes.insert(mesh_id, mesh);
        self.mesh_count += 1;

        mesh_id
    }

    pub fn get_mesh(&self, mesh_id: MeshId) -> Option<&Mesh> {
        self.meshes.get(&mesh_id)
    }
}
