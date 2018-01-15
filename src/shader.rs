use glust::GlVertexArray;
use glust::GlShader;
use glust::GlShaderUniform;
use glust::GlIndexBuffer;
use glust::HasGlVertexArrayHandle;
use glust::RenderTarget;
use glust::GlBufferElementType;
use glust;

use error::*;

pub trait ShaderUniforms {
    fn uniform_array(&self) -> Vec<(&str, GlShaderUniform)>;
}

pub trait Shader {
    type VertexArray : ShaderVertexArray;
    type Uniforms : ShaderUniforms;

    fn gl_shader<'a>(&'a self) -> &'a GlShader;
}

pub trait OfShader<S> where S : Shader {}

pub struct Uniforms {
    pub uniforms : Vec<(String, GlShaderUniform)>
}

impl ShaderUniforms for Uniforms {
    fn uniform_array(&self) -> Vec<(&str, GlShaderUniform)> {
        self.uniforms.iter().map(|&(ref s, x)| (s.as_str(), x)).collect::<Vec<_>>()
    }
}

impl Shader for GlShader {
    type VertexArray = ::glust::GlVertexArray;
    type Uniforms = Uniforms;

    fn gl_shader<'a>(&'a self) -> &'a GlShader {
        &self
    }
}

pub trait ShaderVertexArray {
    fn gl_vertex_array<'a>(&'a self) -> &'a GlVertexArray;
}

impl ShaderVertexArray for GlVertexArray {
    fn gl_vertex_array<'a>(&'a self) -> &'a GlVertexArray {
        self
    }
}



pub fn render<S, V>(shader : &S, vertex_array: &V, vertex_count : i32, render_target: &RenderTarget, uniforms: &S::Uniforms) -> Result<()>
    where
        S : Shader,
        V : HasGlVertexArrayHandle + OfShader<S>
{
    glust::render(
        shader.gl_shader(),
        vertex_array,
        vertex_count,
        render_target,
        &uniforms.uniform_array()[..],
    )?;
    Ok(())
}

pub fn render_indexed<S, V, I>(shader : &S, vertex_array: &V, index_buffer: &GlIndexBuffer<I>, render_target: &RenderTarget, uniforms: &S::Uniforms) -> Result<()>
    where
        I: GlBufferElementType,
        S: Shader,
        V : HasGlVertexArrayHandle + OfShader<S>
{
    glust::render_indexed(
        shader.gl_shader(),
        vertex_array,
        &index_buffer.0,
        render_target,
        &uniforms.uniform_array()[..],
    )?;
    Ok(())
}




