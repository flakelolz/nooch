use crate::prelude::*;

#[derive(Component)]
pub struct Assets {
    pub ken: Texture2D,
}

impl Assets {
    pub fn new(rl: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        let image =
            Image::load_image_from_mem(".png", get_file("data/char/ken/ken.png").unwrap()).unwrap();
        let ken = rl.load_texture_from_image(thread, &image).unwrap();
        Self { ken }
    }

    pub fn get(&self, name: Name) -> Option<&Texture2D> {
        match name {
            Name::Ken => Some(&self.ken),
            _ => None,
        }
    }
}
pub fn get_file(path: &str) -> Option<&'static [u8]> {
    Some(ASSETS.get_file(path)?.contents())
}
