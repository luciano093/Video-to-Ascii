use rusty_ffmpeg::ffi as ffmpeg;
use ffmpeg::AVCodec;

pub struct DecoderCodec {
    raw: *mut AVCodec
}

impl DecoderCodec {
    pub fn from_codec_id(codec_id: i32) -> Result<DecoderCodec, String> {
        let codec = unsafe { ffmpeg::avcodec_find_decoder(codec_id) };

        if codec.is_null() {
            return Err("Unsupported codec".to_string());
        }

        Ok(DecoderCodec{ raw: codec })
    }

    pub fn raw(&self) -> &AVCodec {
        unsafe { &*(self.raw as *const AVCodec) }
    }
}