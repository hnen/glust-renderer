extern crate glust;
extern crate import_obj;
extern crate imagefmt;

mod error;
pub mod deferred;
mod shader;

pub use error::RendererError;

pub use shader::Shader;
pub use shader::ShaderUniforms;
pub use shader::ShaderVertexArray;
pub use shader::render;
pub use shader::render_indexed;
pub use shader::OfShader;

use glust::GlTexture;
use error::*;


pub fn load_tex(filename : &str) -> Result<GlTexture> {
    let img = imagefmt::read(filename, imagefmt::ColFmt::RGBA)?;
    Ok(GlTexture::new_rgba8(img.w, img.h, &img.buf)?.mipmapped())
}





