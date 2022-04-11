pub use bevy_render::prelude::*;

pub const ATTRIBUTE_BARYCENTRIC: &'static str = "Barycentric_Position";
pub const ATTRIBUTE_DASHED: &'static str = "Vertex_Dashed";

pub trait WireframeMeshGenerator {
    fn compute_barycentric(&mut self);

    fn add_dash_to_vertex(&mut self, dash: [f32; 2]);

    fn color_vertex(&mut self, color: Color);
}

impl WireframeMeshGenerator for Mesh {
    fn compute_barycentric(&mut self) {
        let position_count = self
            .attribute(Mesh::ATTRIBUTE_POSITION)
            .expect("Need Mesh::ATTRIBUTE_POSITION to compute barycentric")
            .len();

        let barycentrics = [[0.0, 0.0, 1.0], [0.0, 1.0, 0.0], [1.0, 0.0, 0.0]];
        let barycentrics2 = [[0.0, 1.0, 0.0], [0.0, 0.0, 1.0], [1.0, 0.0, 0.0]];
        let mut barycentric = Vec::new();
        for i in 0..position_count {
            let even = (i / 3) % 2 == 0;
            if even {
                barycentric.push(barycentrics[i % 3]);
            } else {
                barycentric.push(barycentrics2[i % 3]);
            }
        }

        self.set_attribute(ATTRIBUTE_BARYCENTRIC, barycentric);
    }

    fn add_dash_to_vertex(&mut self, dash: [f32; 2]) {
        let position_count = self
            .attribute(Mesh::ATTRIBUTE_POSITION)
            .expect("Need Mesh::ATTRIBUTE_POSITION to add dash attribute")
            .len();
        let mut dashes = vec![];
        for _ in 0..position_count {
            dashes.push(dash);
        }
        self.set_attribute(ATTRIBUTE_DASHED, dashes);
    }

    fn color_vertex(&mut self, color: Color) {
        let position_count = self
            .attribute(Mesh::ATTRIBUTE_POSITION)
            .expect("Need Mesh::ATTRIBUTE_POSITION to color vertex")
            .len();
        let mut colors = vec![];
        for _ in 0..position_count {
            colors.push([color.r(), color.g(), color.b()]);
        }
        self.set_attribute(Mesh::ATTRIBUTE_COLOR, colors);
    }
}