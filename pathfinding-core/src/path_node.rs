use super::*;

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct PathNode {
    pub point: Coordinate,
    pub heuristic: Heuristic,
    pub source: Option<Coordinate>,
}

impl PathNode {
    pub fn initial(point: Coordinate, heuristic: f64) -> Self {
        Self {
            point,
            heuristic,
            source: None,
        }
    }

    pub fn new(point: Coordinate, heuristic: f64, source: Coordinate) -> Self {
        Self {
            point,
            heuristic,
            source: Some(source),
        }
    }
}
