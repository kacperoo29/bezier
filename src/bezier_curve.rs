use crate::vec::Vector2f;

pub struct BezierCurve {
    points: Vec<Vector2f>,
}

impl BezierCurve {
    pub fn new() -> Self {
        Self { points: Vec::new() }
    }

    pub fn add_point(&mut self, point: Vector2f) {
        self.points.push(point);
    }

    pub fn calculate_point(&self, t: f32) -> Vector2f {
        let mut tmp = self.points.clone();
        let mut n = self.points.len() - 1;
        while n > 0 {
            for i in 0..n {
                tmp[i] = tmp[i] * (1.0 - t) + tmp[i + 1] * t;
            }
            n -= 1;
        }

        tmp[0]
    }

    pub fn points(&self) -> &Vec<Vector2f> {
        &self.points
    }

    pub fn intersect_with_control_point(&self, point: Vector2f) -> Option<usize> {
        for (i, p) in self.points.iter().enumerate() {
            if p.distance_to(point) < 10.0 {
                return Some(i);
            }
        }

        None
    }

    pub fn set_point(&mut self, index: usize, point: Vector2f) {
        self.points[index] = point;
    }
}