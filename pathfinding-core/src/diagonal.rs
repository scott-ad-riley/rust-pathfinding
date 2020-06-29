use super::*;

#[derive(PartialOrd, Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Diagonal {
    pub point: Coordinate,
    pub route_sides: (Coordinate, Coordinate),
}

impl Eq for Diagonal {}
impl PartialEq for Diagonal {
    fn eq(&self, other: &Self) -> bool {
        self.point.as_int_tuple() == other.point.as_int_tuple()
            && self.route_sides.0.as_int_tuple() == other.route_sides.0.as_int_tuple()
            && self.route_sides.1.as_int_tuple() == other.route_sides.1.as_int_tuple()
    }
}
