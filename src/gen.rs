use bevy::{
    prelude::*,
    render::{
        mesh::{Indices, VertexAttributeValues},
        pipeline::PrimitiveTopology,
    },
};
use std::f32::consts::PI;

// Unit wire cube
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
        color.resize(6, [1.0; 4]);
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
    let mut mesh = Mesh::from(shape::Icosphere {
        radius: 1.0,
        subdivisions: 2,
    });

    let vertex_count = if let Some(VertexAttributeValues::Float3(vertices)) =
        mesh.attribute(Mesh::ATTRIBUTE_POSITION)
    {
        vertices.len()
    } else {
        unreachable!()
    };

    // Add vertex color (required by shader)
    mesh.set_attribute(Mesh::ATTRIBUTE_COLOR, {
        let mut color: Vec<[f32; 4]> = vec![];
        color.resize(vertex_count, [1.0; 4]);
        color
    });

    mesh
}

pub fn wire_cylinder() -> Mesh {
    let mut positions: Vec<[f32; 3]> = Vec::with_capacity(16 * 2);
    let mut indices: Vec<u16> = Vec::with_capacity(16 * 2 * 2 + 2 * 4);

    // Top
    for i in 0..16u16 {
        let t = (i as f32) * (2.0 / 16.0);
        let (y, x) = f32::sin_cos(t * PI);
        positions.push([x, 0.5, y]);
        if i < 15 {
            indices.push(i);
            indices.push(i + 1);
        }
    }

    // Bottom
    for i in 0..16u16 {
        let p = positions[i as usize];
        positions.push([p[0], -0.5, p[2]]);
        if i < 15 {
            indices.push(i + 16);
            indices.push(i + 16 + 1);
        }
    }

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

// TODO: Hemisphere

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
