#[derive(Clone, Debug)]
pub struct HtmlImage {
    image: web_sys::HtmlImageElement,
}

impl HtmlImage {
    pub fn image(&self) -> &web_sys::HtmlImageElement {
        &self.image
    }

    pub fn new(bytes: &[u8], extension: &str) -> Self {
        let image = web_sys::HtmlImageElement::new().unwrap();
        let src = format!(
            "data:image/{};base64,{}",
            extension,
            base64::encode(&bytes.to_vec())
        );
        image.set_src(&src);
        Self { image }
    }
}
