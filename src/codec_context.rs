use std::ptr::null;
use std::ptr::null_mut;

use rusty_ffmpeg::ffi as ffmpeg;
use ffmpeg::AVCodecContext;
use ffmpeg::avcodec_send_packet;
use rusty_ffmpeg::ffi::avcodec_alloc_context3;
use rusty_ffmpeg::ffi::avcodec_parameters_to_context;

use crate::decoder_codec::DecoderCodec;
use crate::error::CodecContextError;
use crate::error::PacketError;
use crate::format_context::FormatContext;
use crate::frame::Frame;
use crate::packet::Packet;

pub struct CodecContext {
    raw: *mut AVCodecContext,
}

impl CodecContext {
    pub const fn new(raw: *mut AVCodecContext) -> CodecContext {
        CodecContext { raw }
    }

    pub fn from_format_context(format_context: &FormatContext, video_stream_index: isize) -> CodecContext {
        let codec_context = unsafe { avcodec_alloc_context3(null()) };

        if codec_context.is_null() {
            panic!("Failed to allocate codec context");
        }

        if unsafe { avcodec_parameters_to_context(codec_context, format_context.streams().nth(video_stream_index as usize).unwrap().codec_parameters()) } < 0 {
            panic!("Failed to create codec context");
        };

        CodecContext { raw: codec_context }
    }

    pub const fn raw(&self) -> &AVCodecContext {
        unsafe { &*(self.raw as *const AVCodecContext) }
    }

    pub fn raw_mut(&mut self) -> &mut AVCodecContext {
        unsafe { &mut *self.raw }
    }

    pub const fn codec_id(&self) -> i32 {
        self.raw().codec_id
    }

    pub const fn width(&self) -> u32 {
        self.raw().width as u32
    }

    pub const fn height(&self) -> u32 {
        self.raw().height as u32
    }

    pub const fn pix_fmt(&self) -> i32 {
        self.raw().pix_fmt
    }

    pub fn set_thread_count(&mut self, thread_count: i32) {
        self.raw_mut().thread_count = thread_count
    }

    pub fn set_thread_type(&mut self, thread_type: i32) {
        self.raw_mut().thread_type = thread_type
    }

    pub fn open(&mut self, codec: &DecoderCodec) -> Result<(), CodecContextError> {
        if unsafe { ffmpeg::avcodec_open2(self.raw_mut(), codec.raw(), null_mut()) } < 0 {
            return Err(CodecContextError::OpenError);
        };

        Ok(())
    }

    pub fn send_packet(&mut self, packet: &Packet) -> Result<(), PacketError> {
        let result = unsafe { avcodec_send_packet(self.raw_mut(), packet.raw()) };

        if result < 0 {
            return Err(PacketError::SendError(result));
        }

        Ok(())
    }

    pub fn receive_frame(&mut self, input_frame: &mut Frame) -> Result<(), i32> {
        let result = unsafe { ffmpeg::avcodec_receive_frame(self.raw_mut(), input_frame.raw_mut()) };
        
        if result == 0 {
            Ok(())
        } else {
            Err(result)
        }
    }
}