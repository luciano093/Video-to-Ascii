use rusty_ffmpeg::ffi::{self as ffmpeg, av_frame_get_buffer};
use ffmpeg::AVFrame;
use ffmpeg::{av_frame_alloc, av_frame_free};

use crate::error::FrameError;

pub struct Frame {
    raw: *mut AVFrame
}

impl Frame {
    pub fn new() -> Result<Frame, FrameError> {
        let frame = unsafe { av_frame_alloc() };

        if frame.is_null() {
            return Err(FrameError::AllocationError);
        }

        Ok(Frame { raw: frame })
    }

    pub fn from_frame(frame: &Frame) -> Result<Frame, FrameError> {
        let mut new_frame = Frame::new().unwrap();

        new_frame.set_pixel_format(frame.pixel_format());
        new_frame.set_width(frame.width());
        new_frame.set_height(frame.height());
        new_frame.set_channels(frame.channels());
        new_frame.set_channel_layout(frame.channel_layout());
        new_frame.set_nb_samples(frame.nb_samples());

        if unsafe { av_frame_get_buffer(new_frame.raw_mut(), 32) } < 0 {
            return Err(FrameError::AllocationError);
        };

        if unsafe { ffmpeg::av_frame_copy(new_frame.raw_mut(), frame.raw()) } < 0 {
            return Err(FrameError::CopyError);
        };

        if unsafe { ffmpeg::av_frame_copy_props(new_frame.raw_mut(), frame.raw()) } < 0 {
            return Err(FrameError::MetadataCopyError);
        };

        Ok(new_frame)
    }

    pub const fn raw(&self) -> &AVFrame {
        unsafe { &*(self.raw as *const AVFrame) }
    }

    pub fn raw_mut(&mut self) -> &mut AVFrame {
        unsafe { &mut *self.raw }
    }

    pub const fn data(&self) -> &[*mut u8; 8] {
        &self.raw().data
    }

    pub fn data_mut(&mut self) -> &mut [*mut u8; 8] {
        &mut self.raw_mut().data
    }

    pub const fn linesize(&self) -> [i32; 8] {
        self.raw().linesize
    }

    pub const fn width(&self) -> u32 {
        self.raw().width as u32
    }

    pub const fn height(&self) -> u32 {
        self.raw().height as u32
    }

    pub const fn pixel_format(&self) -> i32 {
        self.raw().format
    }

    pub const fn channels(&self) -> i32 {
        self.raw().channels
    }

    pub const fn channel_layout(&self) -> u64 {
        self.raw().channel_layout
    }

    pub const fn nb_samples(&self) -> i32 {
        self.raw().nb_samples
    }

    pub fn is_empty(&self) -> bool {
        self.data()[0].is_null()
    }

    pub fn set_width(&mut self, width: u32) {
        unsafe { (*self.raw).width = width as i32 }
    }

    pub fn set_height(&mut self, height: u32) {
        unsafe { (*self.raw).height = height as i32 }
    }

    pub fn set_pixel_format(&mut self, pixel_format: i32) {
        unsafe { (*self.raw).format = pixel_format }
    }

    pub fn set_channels(&mut self, channels: i32) {
        unsafe { (*self.raw).channels = channels }
    }

    pub fn set_channel_layout(&mut self, channel_layout: u64) {
        unsafe { (*self.raw).channel_layout = channel_layout }
    }

    pub fn set_nb_samples(&mut self, nb_samples: i32) {
        unsafe { (*self.raw).nb_samples = nb_samples }
    }
}

impl Drop for Frame {
    fn drop(&mut self) {
        unsafe { av_frame_free(&mut self.raw) }
    }
}