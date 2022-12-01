use std::ptr::{null_mut, null};

use rusty_ffmpeg::ffi as ffmpeg;
use ffmpeg::SwsContext;
use ffmpeg::sws_getContext;
use rusty_ffmpeg::ffi::av_frame_get_buffer;

use crate::error::FrameError;
use crate::error::ScalerError;
use crate::frame::Frame;

pub struct ScalerContext {
    raw: *mut SwsContext,
    source_height: u32,
    destiny_width: u32,
    destiny_height: u32,
    destiny_pixel_format: i32,
}

impl ScalerContext {
    pub fn new(
        source_width: u32, source_height: u32, source_pixel_format: i32, destiny_width: u32, destiny_height: u32, destiny_pixel_format: i32, flags: u32
    ) -> Result<ScalerContext, ScalerError> {
        let conversion_context = unsafe {
            sws_getContext(
                source_width as i32,
                source_height as i32,
                source_pixel_format,
                destiny_width as i32,
                destiny_height as i32,
                destiny_pixel_format,
                flags as i32, 
                null_mut(),
                null_mut(),
                null()
            )
        };

        if conversion_context.is_null() {
            return Err(ScalerError::ContextError);
        }

        Ok(ScalerContext { raw: conversion_context, source_height, destiny_width, destiny_height, destiny_pixel_format })
    }

    pub fn raw_mut(&mut self) -> &mut SwsContext {
        unsafe { &mut *self.raw }
    }

    pub fn scale(&mut self, input_frame: &Frame, output_frame: &mut Frame) -> Result<(), FrameError> {
        if output_frame.is_empty() {
            output_frame.set_width(self.destiny_width);
            output_frame.set_height(self.destiny_height);
            output_frame.set_pixel_format(self.destiny_pixel_format);

            if unsafe { av_frame_get_buffer(output_frame.raw_mut(), 32) } < 0 {
                return Err(FrameError::AllocationError);
            };
        }

        unsafe {
            ffmpeg::sws_scale(
                self.raw_mut(),
                input_frame.data().as_ptr() as *const *const u8,
                input_frame.linesize().as_ptr(),
                0,
                self.source_height as i32,
                output_frame.data_mut().as_ptr(),
                output_frame.linesize().as_ptr()
            );
        }

        Ok(())
    }
}