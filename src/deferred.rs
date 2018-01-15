use glust::GlIndexBuffer;
use glust::GlBuffer;
use glust::GlTexture;
use ::load_tex;

use import_obj::ObjRenderable;
use import_obj;


use error::*;

pub struct RenderObject {
    pub ib         : GlIndexBuffer<[u32;3]>,
    pub buf_pos    : GlBuffer<[f32;3]>,
    pub buf_uv     : GlBuffer<[f32;2]>,
    pub buf_norm   : GlBuffer<[f32;3]>,
    pub tx_diffuse : GlTexture,
    pub tx_normal  : GlTexture,
    pub tx_spec    : GlTexture
}

pub fn load_obj(obj_data : &str, diffuse_tx_filename : &str, normal_tx_filename : &str, spec_tx_filename : &str) -> Result<RenderObject> {
    let obj_rock = import_obj::obj_load(obj_data)?.pop().ok()?;
    let bfs = create_buffers(&obj_rock)?;

    let tx = load_tex(diffuse_tx_filename)?;
    let tx_normal = load_tex(normal_tx_filename)?;
    let tx_spec = load_tex(spec_tx_filename)?;

    Ok(RenderObject {
        ib: bfs.ib,
        buf_pos: bfs.pos,
        buf_uv: bfs.tx,
        buf_norm: bfs.norm,
        tx_diffuse: tx,
        tx_normal, tx_spec
    })
}

struct ObjBuffers {
    pub ib   : GlIndexBuffer<[u32;3]>,
    pub pos  : GlBuffer<[f32;3]>,
    pub tx   : GlBuffer<[f32;2]>,
    pub norm : GlBuffer<[f32;3]>,
}

fn create_buffers(obj : &ObjRenderable) -> Result<ObjBuffers> {
    Ok(ObjBuffers {
        ib   : GlIndexBuffer::new(&obj.inds)?,
        pos  : GlBuffer::new(&obj.v_pos)?,
        tx   : GlBuffer::new(&obj.v_tx)?,
        norm : GlBuffer::new(&obj.v_norm)?,
    })
}

