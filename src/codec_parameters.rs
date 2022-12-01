use rusty_ffmpeg::ffi::AVCodecParameters;

pub struct CodecParameters {
    raw: *mut AVCodecParameters
}

impl CodecParameters {
    pub fn new(raw: *mut AVCodecParameters) -> CodecParameters {
        CodecParameters { raw }
    }

    pub const fn raw(&self) -> &AVCodecParameters {
        unsafe { &*(self.raw as *const AVCodecParameters) }
    }

    pub const fn codec_type(&self) -> i32 {
        self.raw().codec_type
    }
}