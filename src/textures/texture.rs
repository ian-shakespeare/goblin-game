use crate::resources::Resources;
use gl::types::{GLenum, GLuint, GLvoid};
use image::io::Reader as ImageReader;

#[derive(Clone)]
pub struct Texture {
    pub id: GLuint,
}

impl Texture {
    pub fn from_resource(res: &Resources, name: &str) -> Self {
        let mut id: GLuint = 0;
        unsafe {
            gl::GenTextures(1, &mut id);
            gl::BindTexture(gl::TEXTURE_2D, id);
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_WRAP_S,
                gl::REPEAT.try_into().unwrap(),
            ); // CLAMP_TO_EDGE or REPEAT
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_WRAP_T,
                gl::REPEAT.try_into().unwrap(),
            ); // CLAMP_TO_EDGE or REPEAT
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_MIN_FILTER,
                gl::NEAREST.try_into().unwrap(),
            ); // LINEAR or NEAREST
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_MAG_FILTER,
                gl::NEAREST.try_into().unwrap(),
            ); // LINEAR or NEAREST
        }
        let img = ImageReader::open(res.get_full_path(name))
            .unwrap()
            .decode()
            .unwrap();
        let width = img.width();
        let height = img.height();
        let data = img.into_rgb8();
        // bind image
        unsafe {
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGB.try_into().unwrap(),
                width.try_into().unwrap(),
                height.try_into().unwrap(),
                0,
                gl::RGB,
                gl::UNSIGNED_BYTE,
                data.as_ptr() as *const GLvoid,
            );
            gl::GenerateMipmap(gl::TEXTURE_2D);
        }

        Self { id }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.id);
        }
    }

    pub fn active(texture: GLenum) {
        unsafe {
            gl::ActiveTexture(texture);
        }
    }
}