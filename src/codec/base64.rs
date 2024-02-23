use base64::{Engine as _, engine::general_purpose::STANDARD};

use super::{CodecImplArgs, common};

pub struct Base64Codec {}

impl common::Codec for Base64Codec {
    fn encode(&self, codec_args: &CodecImplArgs) -> Result<String, common::CodecError> {
        Ok(STANDARD.encode(&codec_args.input))
    }

    fn decode(&self, codec_args: &CodecImplArgs) -> Result<String, common::CodecError> {
        match STANDARD.decode(&codec_args.input) {
            Ok(t) => match String::from_utf8(t) {
                Ok(s) => Ok(s),
                Err(e) => Err(common::CodecError { message: e.to_string() })
            },
            Err(e) => Err(common::CodecError { message: e.to_string() })
        }
    }
}