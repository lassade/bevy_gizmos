use bevy::{
    prelude::*,
    render::mesh::{Indices, VertexAttributeValues},
};

macro_rules! mesh_attr {
    ($mesh:tt , $attr:expr, $var:path) => {
        if let Some($var(buffer)) = $mesh.attribute_mut($attr) {
            &mut *(buffer as *mut _)
        } else {
            panic!("missing `{}` mesh attribute", $attr)
        }
    };
}

// pub struct MeshEditXC<'a, X, C, Index> {
//     pub vertices: &'a mut Vec<X>,
//     pub colors: &'a mut Vec<C>,
//     pub indices: &'a mut Vec<Index>,
// }

// impl<'a> MeshEditXC<'a, [f32; 3], [f32; 4], u32> {
//     pub fn new(mesh: &mut Mesh) -> Self {
//         unsafe {
//             Self {
//                 vertices: mesh_attr!(
//                     mesh,
//                     Mesh::ATTRIBUTE_POSITION,
//                     VertexAttributeValues::Float3
//                 ),
//                 colors: mesh_attr!(mesh, Mesh::ATTRIBUTE_COLOR, VertexAttributeValues::Float4),
//                 indices: if let Some(Indices::U32(buffer)) = mesh.indices_mut() {
//                     &mut *(buffer as *mut _)
//                 } else {
//                     panic!("wrong mesh indices format")
//                 },
//             }
//         }
//     }
// }

pub struct MeshEditXC<'a> {
    pub vertices: &'a mut Vec<[f32; 3]>,
    pub colors: &'a mut Vec<[f32; 4]>,
    pub indices: &'a mut Vec<u32>,
    mesh: &'a mut Mesh,
}

impl<'a> From<&'a mut Mesh> for MeshEditXC<'a> {
    fn from(mesh: &'a mut Mesh) -> Self {
        unsafe {
            Self {
                vertices: mesh_attr!(
                    mesh,
                    Mesh::ATTRIBUTE_POSITION,
                    VertexAttributeValues::Float32x3
                ),
                colors: mesh_attr!(
                    mesh,
                    Mesh::ATTRIBUTE_COLOR,
                    VertexAttributeValues::Float32x4
                ),
                indices: if let Some(Indices::U32(buffer)) = mesh.indices_mut() {
                    &mut *(buffer as *mut _)
                } else {
                    panic!("wrong mesh indices format")
                },
                mesh,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    // TODO: Test for fail compilation
    //use super::*;

    // //  This test shouldn't compile
    // #[test]
    // fn is_safe() {
    //     let mut mesh = Mesh::default();

    //     let edit = MeshEditXC::from(&mut mesh);
    //     std::mem::drop(mesh);

    //     let temp = edit.vertices;

    //     println!("{:?}", temp[0]);
    //     std::mem::drop(temp);
    // }
}
