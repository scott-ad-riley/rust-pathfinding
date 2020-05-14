#[macro_use]
extern crate serde_derive;

use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use wasm_bindgen::prelude::*;

mod helpers;
#[cfg(test)]
mod tests;

#[wasm_bindgen]
#[derive(PartialOrd, Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Coordinate {
    pub x: f64,
    pub y: f64,
}

impl Eq for Coordinate {}
impl PartialEq for Coordinate {
    fn eq(&self, other: &Self) -> bool {
        self.as_int_tuple() == other.as_int_tuple()
    }
}
impl Hash for Coordinate {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        helpers::integer_decode(self.x).hash(hasher);
        helpers::integer_decode(self.y).hash(hasher);
    }
}

impl Coordinate {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    fn heuristic(&self, target: &Coordinate) -> f64 {
        let x = (self.x - target.x).abs();
        let y = (self.y - target.y).abs();
        (x.powi(2) + y.powi(2)).sqrt()
    }

    fn in_same_box(&self, target: &Coordinate, box_size: f64) -> bool {
        let x_bounds = (self.x - box_size / 2., self.x + box_size / 2.);
        let y_bounds = (self.y - box_size / 2., self.y + box_size / 2.);

        target.x >= x_bounds.0
            && target.x <= x_bounds.1
            && target.y >= y_bounds.0
            && target.y <= y_bounds.1
    }

    fn centre_of_box(&self, box_size: f64) -> Coordinate {
        let x_mod = self.x % box_size;
        let y_mod = self.y % box_size;
        Coordinate {
            x: self.x - x_mod + box_size / 2.,
            y: self.y - y_mod + box_size / 2.,
        }
    }

    fn load_adjacents(&self, box_size: f64) -> (Vec<Coordinate>, Vec<Diagonal>) {
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

        let top_right = Coordinate {
            x: centre_point.x + box_size,
            y: centre_point.y - box_size,
        };
        let top_left = Coordinate {
            x: centre_point.x - box_size,
            y: centre_point.y - box_size,
        };
        let bottom_right = Coordinate {
            x: centre_point.x + box_size,
            y: centre_point.y + box_size,
        };
        let bottom_left = Coordinate {
            x: centre_point.x - box_size,
            y: centre_point.y + box_size,
        };

        let adjacents: Vec<Coordinate> = vec![above, below, left, right];

        let diagonals: Vec<Diagonal> = vec![
            (top_right, (above, right)),
            (top_left, (above, left)),
            (bottom_right, (below, right)),
            (bottom_left, (below, left)),
        ];

        (adjacents, diagonals)
    }

    fn as_int_tuple(&self) -> ((u64, i16, i8), (u64, i16, i8)) {
        (
            helpers::integer_decode(self.x),
            helpers::integer_decode(self.y),
        )
    }
}

type Diagonal = (Coordinate, (Coordinate, Coordinate));

type Heuristic = f64;

impl ComparableFloat for Heuristic {
    fn to_int(&self) -> i64 {
        (self * 100.).round() as i64
    }
}

trait ComparableFloat {
    fn to_int(&self) -> i64;
}

#[wasm_bindgen]
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

struct AStarState {
    stack: Vec<PathNode>,
    completed: Vec<PathNode>,
}

impl AStarState {
    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
            completed: Vec::new(),
        }
    }

    fn mark_completed(&mut self, completed_node: PathNode) {
        self.completed.push(completed_node)
    }

    fn sort(&mut self) {
        self.stack.sort_by(|adjacent_a, adjacent_b| {
            adjacent_a
                .heuristic
                .partial_cmp(&adjacent_b.heuristic)
                .unwrap()
        })
    }

    fn completed_path<'a>(
        self: &'a Self,
        child_node: &'a PathNode,
        acc: &mut Vec<PathNode>,
    ) -> Vec<PathNode> {
        acc.push(child_node.clone());

        match child_node.source {
            None => {
                acc.reverse();
                acc.clone()
            }
            Some(source) => {
                let x = self
                    .completed
                    .iter()
                    .find(|&path_node| path_node.point == source)
                    .expect("Unable to find Parent Node");
                self.completed_path(x, acc)
            }
        }
    }
}

#[wasm_bindgen]
pub fn find_path(source: JsValue, target: JsValue, box_size: f64, terrain: JsValue) -> JsValue {
    let source: Coordinate = source.into_serde().unwrap();
    let target: Coordinate = target.into_serde().unwrap();
    let terrain: Vec<Coordinate> = terrain.into_serde().unwrap();
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    let result = build_path(source, target, box_size, terrain);

    JsValue::from_serde(&result).unwrap()
}

fn build_path(
    source: Coordinate,
    target: Coordinate,
    box_size: f64,
    terrain: Vec<Coordinate>,
) -> Vec<PathNode> {
    let mut state = AStarState::new();

    let initial = PathNode::initial(source, source.heuristic(&target));

    let mut final_path: Vec<PathNode> = vec![];

    next_step(
        initial,
        target,
        &mut state,
        box_size,
        &terrain,
        &mut final_path,
    )
}

fn next_step<'a>(
    active_node: PathNode,
    target: Coordinate,
    state: &'a mut AStarState,
    box_size: f64,
    terrain: &[Coordinate],
    final_path: &'a mut Vec<PathNode>,
) -> Vec<PathNode> {
    state.mark_completed(active_node);

    if active_node.point.in_same_box(&target, box_size) {
        return state.completed_path(&active_node, final_path);
    }

    let (adjacents, diagonals) = active_node.point.load_adjacents(box_size);
    let inside_bounds = filter_out_of_bounds(adjacents);
    let not_terrain = filter_out_terrain(inside_bounds, terrain);
    let not_already_visited = filter_out_already_visited(not_terrain, &state.completed);
    let non_completed_or_terrain_diagonals: Vec<Diagonal> =
        filter_out_non_completed_diagonals(diagonals, &state.completed, terrain);
    let moveable_diagonals: Vec<Diagonal> =
        filter_moveable_diagonals(non_completed_or_terrain_diagonals, terrain);
    let adding_to_stack: Vec<Coordinate> = not_already_visited
        .into_iter()
        .chain(moveable_diagonals.into_iter().map(|(diagonal, _)| diagonal))
        .collect();
    let mut with_heuristics = to_path_nodes(adding_to_stack, &target, &active_node);
    if !state.stack.is_empty() {
        state.stack = state.stack[1..].to_vec();
    }

    state.stack.append(with_heuristics.as_mut());

    state.sort();

    next_step(state.stack[0], target, state, box_size, terrain, final_path)
}

fn filter_out_of_bounds(adjacents: Vec<Coordinate>) -> Vec<Coordinate> {
    adjacents
        .into_iter()
        .filter(|point| point.x > 0. && point.y > 0.)
        .collect()
}

fn filter_out_terrain(adjacents: Vec<Coordinate>, terrain: &[Coordinate]) -> Vec<Coordinate> {
    let moveable_adjacents: HashSet<Coordinate> = adjacents.into_iter().collect();
    let terrain_hash_set: HashSet<Coordinate> = terrain.iter().cloned().collect();
    moveable_adjacents
        .difference(&terrain_hash_set)
        .copied()
        .collect()
}

fn filter_out_already_visited(options: Vec<Coordinate>, completed: &[PathNode]) -> Vec<Coordinate> {
    let options_hashset: HashSet<Coordinate> = options.into_iter().collect();
    let completed_hash_set: HashSet<Coordinate> = completed.iter().map(|x| x.point).collect();
    options_hashset
        .difference(&completed_hash_set)
        .copied()
        .collect()
}

fn filter_out_non_completed_diagonals(
    diagonals: Vec<Diagonal>,
    completed: &[PathNode],
    terrain: &[Coordinate],
) -> Vec<Diagonal> {
    let mut diagonals: HashSet<Diagonal> = diagonals.into_iter().collect();
    let completed_points: Vec<Coordinate> = completed.iter().map(|x| x.point).collect();
    let invalids: HashSet<&Coordinate> = completed_points.iter().chain(terrain.iter()).collect();
    diagonals.retain(|(diagonal, _)| !invalids.contains(&diagonal));

    diagonals.into_iter().collect()
}

fn filter_moveable_diagonals(diagonals: Vec<Diagonal>, terrain: &[Coordinate]) -> Vec<Diagonal> {
    let mut diagonals: HashSet<Diagonal> = diagonals.into_iter().collect();
    let invalids: HashSet<&Coordinate> = terrain.iter().collect();
    diagonals.retain(|(_, (adjacent_a, adjacent_b))| {
        !invalids.contains(&adjacent_a) && !invalids.contains(&adjacent_b)
    });

    diagonals.into_iter().collect()
}

fn to_path_nodes<'a>(
    options: Vec<Coordinate>,
    target: &Coordinate,
    active_node: &'a PathNode,
) -> Vec<PathNode> {
    options
        .into_iter()
        .map(|x| PathNode::new(x, x.heuristic(&target), active_node.point))
        .collect()
}
