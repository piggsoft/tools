use super::{CodecImplArgs, common};

pub struct DESCodec {}

impl common::Codec for DESCodec {
    fn encode(&self, codec_args: &CodecImplArgs) -> Result<String, common::CodecError> {
        todo!()
    }

    fn decode(&self, codec_args: &CodecImplArgs) -> Result<String, common::CodecError> {
        todo!()
    }
}