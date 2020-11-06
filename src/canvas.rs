use wasm_bindgen::JsCast;

/// This function crates a CanvasRenderingContext2D, from which you can draw images on the canvas.
pub fn get_canvas_rendering_context_2d(canvas_id: &str) -> web_sys::CanvasRenderingContext2d {
    let canvas = get_html_canvas_element_by_id(canvas_id);
    get_canvas_rendering_context_2d_from_html_canvas_element(&canvas)
}

fn get_html_canvas_element_by_id(id: &str) -> web_sys::HtmlCanvasElement {
    web_sys::window()
        .expect("No global window.")
        .document()
        .expect("The window should have document.")
        .get_element_by_id(id)
        .expect(format!("The document has no element with id {}.", id).as_str())
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .expect("The element with id {} should be a HTMLCanvasElement.")
}

fn get_canvas_rendering_context_2d_from_html_canvas_element(
    canvas: &web_sys::HtmlCanvasElement,
) -> web_sys::CanvasRenderingContext2d {
    canvas
        .get_context("2d")
        .expect("Failed to create a Context.")
        .expect("No Context.")
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .expect("The element should be a CanvasRenderingContext2D.")
}
