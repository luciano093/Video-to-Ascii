use rusty_ffmpeg::ffi as ffmpeg;
use ffmpeg::AVCodec;

use crate::error::CodecError;

pub struct DecoderCodec {
    raw: *const AVCodec
}

impl DecoderCodec {
    pub fn from_codec_id(codec_id: i32) -> Result<DecoderCodec, CodecError> {
        let codec = unsafe { ffmpeg::avcodec_find_decoder(codec_id) };

        if codec.is_null() {
            return Err(CodecError::UnsuportedCodec);
        }

        Ok(DecoderCodec{ raw: codec })
    }

    pub const fn raw(&self) -> &AVCodec {
        unsafe { &*(self.raw as *const AVCodec) }
    }
}