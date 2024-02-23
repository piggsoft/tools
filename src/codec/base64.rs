use super::{CodecImplArgs, common};

pub struct Base64Codec {}

impl common::Codec for Base64Codec {
    fn encode(&self, codec_args: &CodecImplArgs) -> Result<String, common::CodecError> {
        todo!()
    }

    fn decode(&self, codec_args: &CodecImplArgs) -> Result<String, common::CodecError> {
        todo!()
    }
}