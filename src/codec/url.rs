use urlencoding;

use super::{CodecImplArgs, common};

pub struct URLCodec {}

impl common::Codec for URLCodec {
    fn encode(&self, codec_args: &CodecImplArgs) -> Result<String, common::CodecError> {
        Ok(urlencoding::encode(&codec_args.input).to_string())
    }

    fn decode(&self, codec_args: &CodecImplArgs) -> Result<String, common::CodecError> {
        match urlencoding::decode(&codec_args.input) {
            Ok(t) => Ok(t.to_string()),
            Err(e) => Err(common::CodecError { message: e.to_string() })
        }
    }
}