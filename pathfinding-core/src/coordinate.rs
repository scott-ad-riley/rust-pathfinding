use super::*;
use std::hash::{Hash, Hasher};

impl Eq for Coordinate {}
impl PartialEq for Coordinate {
    fn eq(&self, other: &Self) -> bool {
        self.as_int_tuple() == other.as_int_tuple()
    }
}

#[derive(PartialOrd, Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Coordinate {
    pub x: f64,
    pub y: f64,
}

impl Coordinate {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn heuristic(&self, target: &Coordinate) -> f64 {
        let x = (self.x - target.x).abs();
        let y = (self.y - target.y).abs();
        (x.powi(2) + y.powi(2)).sqrt()
    }

    pub fn in_same_box(&self, target: &Coordinate, box_size: f64) -> bool {
        let x_bounds = (self.x - box_size / 2., self.x + box_size / 2.);
        let y_bounds = (self.y - box_size / 2., self.y + box_size / 2.);

        target.x >= x_bounds.0
            && target.x <= x_bounds.1
            && target.y >= y_bounds.0
            && target.y <= y_bounds.1
    }

    pub fn centre_of_box(&self, box_size: f64) -> Coordinate {
        let x_mod = self.x % box_size;
        let y_mod = self.y % box_size;
        Coordinate {
            x: self.x - x_mod + box_size / 2.,
            y: self.y - y_mod + box_size / 2.,
        }
    }

    pub fn load_options(&self, box_size: f64) -> (Vec<Coordinate>, Vec<Diagonal>) {
        let centre_point = self.centre_of_box(box_size);

        let above = Coordinate {
            x: centre_point.x,
            y: centre_point.y - box_size,
        };
        let below = Coordinate {
            x: centre_point.x,
            y: centre_point.y + box_size,
        };
        let left = Coordinate {
            x: centre_point.x - box_size,
            y: centre_point.y,
        };
        let right = Coordinate {
            x: centre_point.x + box_size,
            y: centre_point.y,
        };

        let top_right = Diagonal {
            point: Coordinate {
                x: centre_point.x + box_size,
                y: centre_point.y - box_size,
            },
            route_sides: (above, right),
        };
        let top_left = Diagonal {
            point: Coordinate {
                x: centre_point.x - box_size,
                y: centre_point.y - box_size,
            },
            route_sides: (above, left),
        };
        let bottom_right = Diagonal {
            point: Coordinate {
                x: centre_point.x + box_size,
                y: centre_point.y + box_size,
            },
            route_sides: (below, right),
        };
        let bottom_left = Diagonal {
            point: Coordinate {
                x: centre_point.x - box_size,
                y: centre_point.y + box_size,
            },
            route_sides: (below, left),
        };

        (
            vec![above, below, left, right],
            vec![top_right, top_left, bottom_right, bottom_left],
        )
    }

    pub fn as_int_tuple(&self) -> ((u64, i16, i8), (u64, i16, i8)) {
        (
            helpers::integer_decode(self.x),
            helpers::integer_decode(self.y),
        )
    }
}

impl Hash for Coordinate {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        helpers::integer_decode(self.x).hash(hasher);
        helpers::integer_decode(self.y).hash(hasher);
    }
}
