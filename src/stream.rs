use std::marker::PhantomData;

use rusty_ffmpeg::ffi as ffmpeg;
use ffmpeg::AVStream;

use crate::codec_parameters::CodecParameters;

pub struct Stream<'a> {
    raw: *mut AVStream,
    _phantom_data: PhantomData<&'a ()>
}

impl<'a> Stream<'a> {
    pub const fn new(raw: *mut AVStream) -> Stream<'a> {
        Stream { raw, _phantom_data: PhantomData }
    }

    pub const fn raw(&self) -> &'a AVStream {
        unsafe { &*(self.raw as *const AVStream) }
    }

    pub fn raw_mut(&mut self) -> &'a mut AVStream {
        unsafe { &mut *self.raw }
    }

    pub fn codec_parameters(&self) -> CodecParameters {
        CodecParameters::new(self.raw().codecpar)
    }

    pub fn average_fps(&self) -> f64 {
        self.raw().avg_frame_rate.num as f64 / self.raw().avg_frame_rate.den as f64
    }

    /// Returns number of frames if known or 0 otherwise.
    pub fn frame_count(&self) -> i64 {
        self.raw().nb_frames
    }
}