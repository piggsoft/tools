use super::{CodecImplArgs, common};

pub struct AESCodec {}

impl common::Codec for AESCodec {
    fn encode(&self, codec_args: &CodecImplArgs) -> Result<String, common::CodecError> {
        todo!()
    }

    fn decode(&self, codec_args: &CodecImplArgs) -> Result<String, common::CodecError> {
        todo!()
    }
}