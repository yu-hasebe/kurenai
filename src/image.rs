/// This function creates an HTMLImageElement, which you can add to the GameService implementation.
pub fn create_new_html_image_element(bytes: &[u8], extension: &str) -> web_sys::HtmlImageElement {
    let html_image_element =
        web_sys::HtmlImageElement::new().expect("Failed to create an HTMLImageElemenet.");
    let src = format!(
        "data:image/{};base64,{}",
        extension,
        base64::encode(&bytes.to_vec())
    );
    html_image_element.set_src(&src);
    html_image_element
}
