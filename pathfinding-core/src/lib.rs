#[macro_use]
extern crate serde_derive;

use std::collections::HashSet;

mod coordinate;
mod diagonal;
mod helpers;
mod path_node;
pub use coordinate::Coordinate;
use diagonal::Diagonal;
use path_node::PathNode;
#[cfg(test)]
mod tests;

type Heuristic = f64;

impl ComparableFloat for Heuristic {
    fn to_int(&self) -> i64 {
        (self * 100.).round() as i64
    }
}

trait ComparableFloat {
    fn to_int(&self) -> i64;
}

struct AStarState {
    stack: Vec<PathNode>,
    completed: Vec<PathNode>,
    completed_hash_set: HashSet<Coordinate>,
}

impl AStarState {
    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
            completed: Vec::new(),
            completed_hash_set: HashSet::new(),
        }
    }

    fn mark_completed(&mut self, completed_node: PathNode) {
        self.completed.push(completed_node);
        self.completed_hash_set.insert(completed_node.point);
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

pub fn build_path(
    source: Coordinate,
    target: Coordinate,
    box_size: f64,
    terrain_vec: Vec<Coordinate>,
) -> Vec<PathNode> {
    let mut state = AStarState::new();

    let initial = PathNode::initial(source, source.heuristic(&target));

    let mut final_path: Vec<PathNode> = vec![];

    let terrain_set: HashSet<Coordinate> = terrain_vec.into_iter().collect();

    next_step(
        initial,
        target,
        &mut state,
        box_size,
        &terrain_set,
        &mut final_path,
    )
}

fn next_step<'a>(
    active_node: PathNode,
    target: Coordinate,
    state: &'a mut AStarState,
    box_size: f64,
    terrain: &HashSet<Coordinate>,
    final_path: &'a mut Vec<PathNode>,
) -> Vec<PathNode> {
    state.mark_completed(active_node);

    if active_node.point.in_same_box(&target, box_size) {
        return state.completed_path(&active_node, final_path);
    }

    let (adjacents, diagonals) = active_node.point.load_options(box_size);
    let valid_adjacents = adjacents
        .into_iter()
        .filter(|point| point.x > 0. && point.y > 0.)
        .filter(|point| !terrain.contains(point))
        .filter(|point| !state.completed_hash_set.contains(point));

    let valid_diagonals = diagonals
        .into_iter()
        .filter(|diag| diag.point.x > 0. && diag.point.y > 0.)
        .filter(|diag| {
            !terrain.contains(&diag.point)
                && !terrain.contains(&diag.route_sides.0)
                && !terrain.contains(&diag.route_sides.1)
        })
        .filter(|diag| !state.completed_hash_set.contains(&diag.point))
        .map(|diag| diag.point);

    let mut valid_path_nodes: Vec<PathNode> = valid_adjacents
        .chain(valid_diagonals)
        .map(|x| PathNode::new(x, x.heuristic(&target), active_node.point))
        .collect();

    if !state.stack.is_empty() {
        state.stack = state.stack[1..].to_vec();
    }

    state.stack.append(valid_path_nodes.as_mut());

    state.sort();

    next_step(state.stack[0], target, state, box_size, terrain, final_path)
}
