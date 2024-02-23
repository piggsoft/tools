use std::error;
use std::fmt::{Display, Formatter};

use crate::codec::CodecImplArgs;

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct CodecError {
    pub(crate) message: String,
}

impl Display for CodecError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.message.fmt(f)
    }
}

impl error::Error for CodecError {}

pub trait Codec {
    fn encode(&self, codec_args: &CodecImplArgs) -> Result<String, CodecError>;
    fn decode(&self, codec_args: &CodecImplArgs) -> Result<String, CodecError>;
}