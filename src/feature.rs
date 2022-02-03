use std::vec::Vec;

pub struct Feature {
    pub array: Vec<f64>,
}

impl Feature {
    pub fn ground_distance(&self, other: &Feature) -> f64 {
        let mut result: f64 = 0.0;
        for i in 0..self.array.len() {
            let diff = self.array[i] - other.array[i];
            result += diff * diff;
        }

        result.sqrt()
    }
}
