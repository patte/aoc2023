use wasm_bindgen::prelude::*;

mod day5;
use day5::*;

// Called when the wasm module is instantiated
#[wasm_bindgen(start)]
fn main() -> Result<(), JsValue> {
    // Use `web_sys`'s global `window` function to get a handle on the global
    // window object.
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    //let body = document.body().expect("document should have a body");

    // Manufacture the element we're gonna append
    //let val = document.create_element("p")?;
    //body.append_child(&val)?;
    if let Some(val) = document.get_element_by_id("title") {
        val.set_inner_html("Hello, adventofcode.com/2023/day/5 from rust!");
    }

    Ok(())
}

#[wasm_bindgen]
pub fn solve_day5(part: u32, input: Option<String>) -> String {
    let almanac = if let Some(input) = input {
        input
    } else {
        EXAMPLE.to_string()
    };

    let answer = get_answer_for_almanac(&almanac, Some(part));
    format!("{:?}", answer)
}
