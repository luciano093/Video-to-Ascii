use std::marker::PhantomData;

use rusty_ffmpeg::ffi::{self as ffmpeg, AVCodecContext, AVCodecParameters};
use ffmpeg::AVStream;

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

    pub const fn codec(&self) -> &'a AVCodecContext {
        unsafe { &*(self.raw().codec as *const AVCodecContext) }
    }

    pub const fn codec_parameters(&self) -> &'a AVCodecParameters {
        unsafe { &*(self.raw().codecpar as *const AVCodecParameters) }
    }

    pub fn codec_mut (&mut self) -> &'a mut AVCodecContext {
        unsafe { &mut *(self.raw_mut().codec as *mut AVCodecContext) }
    }

    pub fn average_fps(&self) -> f64 {
        self.raw().avg_frame_rate.num as f64 / self.raw().avg_frame_rate.den as f64
    }

    /// Returns number of frames if known or 0 otherwise.
    pub fn frame_count(&self) -> i64 {
        self.raw().nb_frames
    }
}