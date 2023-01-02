use thiserror::Error;

#[derive(Error, Debug)]
pub enum InternalError {
    //#[error("missing or invalid `{0}` rom file")]
    //InvalidRom(String),
    #[error("missing or invalid audio output")]
    InvalidAudioOutput,
}
