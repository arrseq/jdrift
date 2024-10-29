use crate::renderer::vec2::Vec2;

pub fn flatten_curve(points: [Vec2; 4], resolution: usize) -> Vec<Vec2> {
    let step_size = 1.0f32 / resolution as f32;
    let mut position = 0.0;
    
    let lengths = [
        points[0].distance(points[1]),
        points[1].distance(points[2]),
        points[2].distance(points[3])
    ];
    
    for _ in 0..resolution {
        position += step_size;
    }
    
    vec![]
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Segment {
    Curve { points: [[usize; 2]; 4] },
    Line { points: [[usize; 2]; 2] }
}

#[derive(Debug, PartialEq)]
pub struct Shape {
    pub points: Vec<Segment>
}