use bevy::{
    prelude::*,
    render::{mesh::Indices, render_resource::PrimitiveTopology},
};

pub struct Sector {
    pub radius: f32,
    pub angle: f32,
}

impl From<Sector> for Mesh {
    fn from(value: Sector) -> Self {
        use std::f32::consts::TAU;

        let perimeter_vertex_count = (64.0 * value.angle / TAU).ceil() as usize;

        // 1 vertex for the point and 64 vertices for a full circle.
        let vertex_count = 1 + perimeter_vertex_count;

        let mut positions: Vec<Vec3> = Vec::with_capacity(vertex_count);

        positions.push(Vec3::ZERO);
        for i in 0..perimeter_vertex_count {
            let angle =
                ((i as f32) / (perimeter_vertex_count as f32)) * value.angle - value.angle / 2.0;
            positions.push(Vec3 {
                x: value.radius * angle.cos(),
                y: value.radius * angle.sin(),
                z: 0.0,
            });
        }

        let mut indices = Vec::with_capacity(perimeter_vertex_count - 1);
        for i in 0..(perimeter_vertex_count as u32 - 1) {
            indices.push(0);
            indices.push(i);
            indices.push(i + 1);
        }

        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        mesh.set_indices(Some(Indices::U32(indices)));
        mesh
    }
}
