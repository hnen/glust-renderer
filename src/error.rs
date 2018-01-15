
use std::error::Error;
use glust::GlError;
use imagefmt;
use import_obj;

#[derive(Debug)]
pub enum RendererError {
    GlustError(String),
    ImageFmtError(String),
    ObjImportError(String),
    MissingValueError,
    Other
}

pub type Result<T> = ::std::result::Result<T, RendererError>;

impl Error for RendererError {
    fn description(&self) -> &str {
        match *self {
            RendererError::GlustError(_) => "Glust error.",
            RendererError::ImageFmtError(_) => "Image loading error",
            RendererError::ObjImportError(_) => "Obj importing error",
            RendererError::MissingValueError => "Value unexpectedly missing.",
            RendererError::Other => "Other error."
        }
    }
}

impl ::std::fmt::Display for RendererError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<GlError> for RendererError {
    fn from(err: GlError) -> Self {
        RendererError::GlustError(err.description().to_owned())
    }
}
impl From<imagefmt::Error> for RendererError {
    fn from(err: imagefmt::Error) -> Self {
        RendererError::ImageFmtError(err.description().to_owned())
    }
}
impl From<import_obj::ObjError> for RendererError {
    fn from(err: import_obj::ObjError) -> Self {
        RendererError::ObjImportError(err.description().to_owned())
    }
}

pub trait OptErr<T> {
    fn ok(self) -> Result<T>;
}

impl<T> OptErr<T> for Option<T> {
    fn ok(self) -> Result<T> {
        self.ok_or( RendererError::MissingValueError )
    }
}

