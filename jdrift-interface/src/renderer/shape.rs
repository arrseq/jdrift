fn length(segment: [[usize; 2]; 2]) -> usize {
    let first = (segment[1][0] - segment[0][0]);
    let second = (segment[1][1] - segment[0][1]);
    (first + second).isqrt()
}

pub fn flatten_curve(points: [[usize; 2]; 4], resolution: usize) -> Vec<[usize; 2]> {
    let step_size = 1.0f32 / resolution as f32;
    let mut position = 0.0;
    let lengths = [
        length([ points[0], points[1] ]),
        length([ points[1], points[2] ]),
        length([ points[2], points[3] ])
    ];
    
    
    dbg!(lengths);
    
    for _ in 0..resolution {
        println!("{position}");
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