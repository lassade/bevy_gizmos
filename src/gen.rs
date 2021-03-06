use bevy::{
    prelude::*,
    render::{mesh::Indices, pipeline::PrimitiveTopology},
};
use std::f32::consts::PI;

pub fn wire_cube() -> Mesh {
    let mut color: Vec<[f32; 4]> = vec![];
    color.resize(8, [1.0; 4]);

    let mut mesh = Mesh::new(PrimitiveTopology::LineList);
    mesh.set_attribute(Mesh::ATTRIBUTE_COLOR, color);
    mesh.set_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vec![
            // Front
            [0.5, 0.5, 0.5],
            [0.5, -0.5, 0.5],
            [-0.5, -0.5, 0.5],
            [-0.5, 0.5, 0.5],
            // Back
            [0.5, 0.5, -0.5],
            [0.5, -0.5, -0.5],
            [-0.5, -0.5, -0.5],
            [-0.5, 0.5, -0.5],
        ],
    );
    mesh.set_indices(Some(Indices::U16(vec![
        0, 1, 1, 2, 2, 3, 3, 0, // Front
        4, 5, 5, 6, 6, 7, 7, 4, // Back
        0, 4, 1, 5, 2, 6, 3, 7, // Bridge
    ])));
    mesh
}

pub fn cube() -> Mesh {
    let mut mesh = Mesh::from(shape::Cube::new(1.0));

    // Add vertex color (required by shader)
    mesh.set_attribute(Mesh::ATTRIBUTE_COLOR, {
        let mut color: Vec<[f32; 4]> = vec![];
        color.resize(24, [1.0; 4]);
        color
    });

    mesh
}

pub fn wire_sphere() -> Mesh {
    let mut positions: Vec<[f32; 3]> = Vec::with_capacity(16 * 3);
    let mut indices: Vec<u16> = Vec::with_capacity(16 * 3 * 2);

    for i in 0..16u16 {
        let t = (i as f32) * (2.0 / 15.0);
        let (y, x) = f32::sin_cos(t * PI);
        positions.push([x, y, 0.0]);
        if i < 15 {
            indices.push(i);
            indices.push(i + 1);
        }
    }

    for i in 0..16u16 {
        let p = positions[i as usize];
        positions.push([p[0], 0.0, p[1]]);
        if i < 15 {
            indices.push(i + 32);
            indices.push(i + 1 + 32);
        }
    }

    for i in 0..16u16 {
        let p = positions[i as usize];
        positions.push([0.0, p[1], p[0]]);
        if i < 15 {
            indices.push(i + 48);
            indices.push(i + 1 + 48);
        }
    }

    let mut color: Vec<[f32; 4]> = vec![];
    color.resize(positions.len(), [1.0; 4]);

    let mut mesh = Mesh::new(PrimitiveTopology::LineList);
    mesh.set_attribute(Mesh::ATTRIBUTE_COLOR, color);
    mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.set_indices(Some(Indices::U16(indices)));
    mesh
}

pub fn sphere() -> Mesh {
    helper::Sphere {
        hemisphere: false,
        divisions: 4,
    }
    .into()
}

pub fn wire_cylinder() -> Mesh {
    let mut positions: Vec<[f32; 3]> = Vec::with_capacity(16 * 2);
    let mut indices: Vec<u16> = Vec::with_capacity(16 * 2 * 2 + 2 * 4);

    // Top
    for i in 0..16u16 {
        let t = (i as f32) * (2.0 / 16.0);
        let (y, x) = f32::sin_cos(t * PI);
        positions.push([x, 0.5, y]);
    }

    // Bottom
    for i in 0..16u16 {
        let p = positions[i as usize];
        positions.push([p[0], -0.5, p[2]]);
    }

    for i in 0..15u16 {
        indices.push(i);
        indices.push(i + 1);
        indices.push(i + 16);
        indices.push(i + 16 + 1);
    }
    indices.push(15);
    indices.push(0);
    indices.push(15 + 16);
    indices.push(16);

    indices.push(0);
    indices.push(16);

    indices.push(4);
    indices.push(16 + 4);

    indices.push(8);
    indices.push(16 + 8);

    indices.push(12);
    indices.push(16 + 12);

    let mut color: Vec<[f32; 4]> = vec![];
    color.resize(positions.len(), [1.0; 4]);

    let mut mesh = Mesh::new(PrimitiveTopology::LineList);
    mesh.set_attribute(Mesh::ATTRIBUTE_COLOR, color);
    mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.set_indices(Some(Indices::U16(indices)));
    mesh
}

// TODO: Cylinder

pub fn wire_hemisphere() -> Mesh {
    let mut positions: Vec<[f32; 3]> = Vec::with_capacity(16 * 3);
    let mut indices: Vec<u16> = Vec::with_capacity(16 * 3 * 2);

    for i in 0..16u16 {
        let t = (i as f32) * (1.0 / 15.0);
        let (y, x) = f32::sin_cos(t * PI);
        positions.push([x, y, 0.0]);
        if i < 15 {
            indices.push(i);
            indices.push(i + 1);
        }
    }

    for i in 0..16u16 {
        let t = (i as f32) * (2.0 / 15.0);
        let (y, x) = f32::sin_cos(t * PI);
        positions.push([x, 0.0, y]);
        indices.push(i + 32);
        indices.push(i + 1 + 32);
    }

    for i in 0..16u16 {
        let p = positions[i as usize];
        positions.push([0.0, p[1], p[0]]);
        if i < 15 {
            indices.push(i + 48);
            indices.push(i + 1 + 48);
        }
    }

    let mut color: Vec<[f32; 4]> = vec![];
    color.resize(positions.len(), [1.0; 4]);

    let mut mesh = Mesh::new(PrimitiveTopology::LineList);
    mesh.set_attribute(Mesh::ATTRIBUTE_COLOR, color);
    mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.set_indices(Some(Indices::U16(indices)));
    mesh
}

pub fn hemisphere() -> Mesh {
    helper::Sphere {
        hemisphere: true,
        divisions: 4,
    }
    .into()
}

pub fn wire_capsule_cap() -> Mesh {
    let mut positions: Vec<[f32; 3]> = Vec::with_capacity(16 * 2);
    let mut indices: Vec<u16> = Vec::with_capacity(16 * 2 * 2);

    for i in 0..16u16 {
        let t = (i as f32) * (1.0 / 15.0);
        let (y, x) = f32::sin_cos(t * PI);
        positions.push([x, y, 0.0]);
        if i < 15 {
            indices.push(i);
            indices.push(i + 1)
        }
    }

    for i in 0..16u16 {
        let p = positions[i as usize];
        positions.push([0.0, p[1], p[0]]);
        if i < 15 {
            indices.push(i + 16);
            indices.push(i + 16 + 1);
        }
    }

    let mut color: Vec<[f32; 4]> = vec![];
    color.resize(positions.len(), [1.0; 4]);

    let mut mesh = Mesh::new(PrimitiveTopology::LineList);
    mesh.set_attribute(Mesh::ATTRIBUTE_COLOR, color);
    mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.set_indices(Some(Indices::U16(indices)));
    mesh
}

pub fn capsule_body() -> Mesh {
    let mut positions: Vec<[f32; 3]> = Vec::with_capacity(16 * 2);
    let mut indices: Vec<u16> = Vec::with_capacity(15 * 6);

    for i in 0..16u16 {
        let t = (i as f32) * (2.0 / 16.0);
        let (y, x) = f32::sin_cos(t * PI);
        positions.push([x, 0.5, y]);
    }

    for i in 0..16u16 {
        let p = positions[i as usize];
        positions.push([p[0], -0.5, p[2]]);
    }

    for i in 0..15u16 {
        indices.push(i);
        indices.push(i + 1);
        indices.push(i + 16);
        indices.push(i + 1);
        indices.push(i + 16 + 1);
        indices.push(i + 16);
    }
    indices.push(15);
    indices.push(0);
    indices.push(15 + 16);
    indices.push(0);
    indices.push(16);
    indices.push(15 + 16);

    let mut color: Vec<[f32; 4]> = vec![];
    color.resize(positions.len(), [1.0; 4]);

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.set_attribute(Mesh::ATTRIBUTE_COLOR, color);
    mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.set_indices(Some(Indices::U16(indices)));
    mesh
}

pub fn wire_empty() -> Mesh {
    let mut color: Vec<[f32; 4]> = vec![];
    color.resize(6, [1.0; 4]);

    let mut mesh = Mesh::new(PrimitiveTopology::LineList);
    mesh.set_attribute(Mesh::ATTRIBUTE_COLOR, color);
    mesh.set_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vec![
            // X
            [0.5, 0.0, 0.0],
            [-0.5, 0.0, 0.0],
            // Y
            [0.0, -0.5, 0.0],
            [0.0, 0.5, 0.0],
            // Z
            [0.0, 0.0, 0.5],
            [0.0, 0.0, -0.5],
        ],
    );
    mesh.set_indices(Some(Indices::U16(vec![0, 1, 2, 3, 4, 5])));
    mesh
}

/// Actually an small (0.1) octahedron
pub fn empty() -> Mesh {
    let mut color: Vec<[f32; 4]> = vec![];
    color.resize(6, [1.0; 4]);

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.set_attribute(Mesh::ATTRIBUTE_COLOR, color);
    mesh.set_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vec![
            [0.0, -0.1414, 0.0],
            [0.0, 0.1414, 0.0],
            [-0.1, 0.0, -0.1],
            [-0.1, 0.0, 0.1],
            [0.1, 0.0, 0.1],
            [0.1, 0.0, -0.1],
        ],
    );
    mesh.set_indices(Some(Indices::U16(vec![
        2, 1, 3, 3, 1, 4, 4, 1, 5, 5, 1, 2, 0, 5, 2, 0, 4, 5, 0, 3, 4, 0, 2, 3,
    ])));
    mesh
}

pub fn billboard() -> Mesh {
    let mut mesh = Mesh::from(shape::Quad::new(Vec2::one()));

    // Add vertex color (required by shader)
    mesh.set_attribute(Mesh::ATTRIBUTE_COLOR, {
        let mut color: Vec<[f32; 4]> = vec![];
        color.resize(4, [1.0; 4]);
        color
    });

    mesh
}

pub fn wire_circle() -> Mesh {
    todo!()
}

pub fn circle() -> Mesh {
    todo!()
}

mod helper {
    use super::*;

    const SPHERIFIED_CUBE_DATA: [([f32; 3], [f32; 3], [f32; 3]); 6] = [
        ([-1.0, -1.0, -1.0], [2.0, 0.0, 0.0], [0.0, 2.0, 0.0]),
        ([1.0, -1.0, -1.0], [0.0, 0.0, 2.0], [0.0, 2.0, 0.0]),
        ([1.0, -1.0, 1.0], [-2.0, 0.0, 0.0], [0.0, 2.0, 0.0]),
        ([-1.0, -1.0, 1.0], [0.0, 0.0, -2.0], [0.0, 2.0, 0.0]),
        ([-1.0, 1.0, -1.0], [2.0, 0.0, 0.0], [0.0, 0.0, 2.0]),
        ([-1.0, -1.0, 1.0], [2.0, 0.0, 0.0], [0.0, 0.0, -2.0]),
    ];

    /// Creates as spherified cube,
    #[derive(Default)]
    pub struct Sphere {
        pub hemisphere: bool,
        pub divisions: u16,
    }

    impl Into<Mesh> for Sphere {
        fn into(self) -> Mesh {
            let step = 1.0 / self.divisions as f32;
            let step3 = Vec3::new(step, step, step);

            let mut positions: Vec<[f32; 3]> = vec![];
            for face in 0..6 {
                // Helps to reduce a couple of vertices
                if self.hemisphere && face == 5 {
                    continue;
                }

                let (origin, right, up) = SPHERIFIED_CUBE_DATA[face];
                let origin = Vec3::from(origin);
                let right = Vec3::from(right);
                let up = Vec3::from(up);

                for j in 0..self.divisions + 1 {
                    let j = j as f32;
                    let j3 = Vec3::new(j, j, j);
                    for i in 0..self.divisions + 1 {
                        let i = i as f32;
                        let i3 = Vec3::new(i, i, i);

                        // Normalized
                        // let p = origin + step3 * (i3 * right + j3 * up);
                        // positions.push(p.normalize().into());

                        // Spherified
                        let p: Vec3 = origin + step3 * (i3 * right + j3 * up);
                        let p2: Vec3 = p * p;
                        positions.push([
                            p.x * (1.0 - 0.5 * (p2.y + p2.z) + p2.y * p2.z / 3.0).sqrt(),
                            p.y * (1.0 - 0.5 * (p2.z + p2.x) + p2.z * p2.x / 3.0).sqrt(),
                            p.z * (1.0 - 0.5 * (p2.x + p2.y) + p2.x * p2.y / 3.0).sqrt(),
                        ]);
                    }
                }
            }

            let k = self.divisions + 1;
            let mut indices: Vec<u16> = vec![];
            for face in 0..6 {
                for j in 0..self.divisions {
                    let bottom = j < (self.divisions / 2);
                    for i in 0..self.divisions {
                        let left = i < (self.divisions / 2);

                        // Skip bottom faces
                        if self.hemisphere && bottom && face != 4 {
                            continue;
                        }

                        let a = (face * k + j) * k + i;
                        let c = (face * k + j) * k + i + 1;
                        let d = (face * k + j + 1) * k + i;
                        let b = (face * k + j + 1) * k + i + 1;

                        if bottom ^ left {
                            indices.push(b);
                            indices.push(a);
                            indices.push(d);

                            indices.push(b);
                            indices.push(c);
                            indices.push(a);
                        } else {
                            indices.push(a);
                            indices.push(b);
                            indices.push(c);

                            indices.push(b);
                            indices.push(a);
                            indices.push(d);
                        }
                    }
                }
            }

            let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
            mesh.set_attribute(Mesh::ATTRIBUTE_COLOR, {
                let mut color: Vec<[f32; 4]> = vec![];
                color.resize(positions.len(), [1.0; 4]);
                color
            });
            mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, positions);
            mesh.set_indices(Some(Indices::U16(indices)));
            mesh
        }
    }
}
