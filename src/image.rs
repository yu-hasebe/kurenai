#[derive(Clone, Debug)]
pub struct Image {
    image: web_sys::HtmlImageElement,
}

impl Image {
    pub fn image(&self) -> &web_sys::HtmlImageElement {
        &self.image
    }

    pub fn new(bytes: &[u8], extension: &str) -> Result<Self, String> {
        // TODO: Add validation
        let image = web_sys::HtmlImageElement::new().unwrap();
        let src = format!(
            "data:image/{};base64,{}",
            extension,
            base64::encode(&bytes.to_vec())
        );
        image.set_src(&src);
        Ok(Self { image })
    }
}
