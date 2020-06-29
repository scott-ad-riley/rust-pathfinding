use pathfinding_core::{build_path, Coordinate};
use wasm_bindgen::prelude::*;

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
