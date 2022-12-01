use std::ptr::null_mut;

use rusty_ffmpeg::ffi as ffmpeg;
use ffmpeg::AVCodecContext;
use ffmpeg::avcodec_send_packet;
use ffmpeg::FF_THREAD_SLICE;

use crate::decoder_codec::DecoderCodec;
use crate::error::CodecContextError;
use crate::error::PacketError;
use crate::frame::Frame;
use crate::packet::Packet;

pub struct CodecContext {
    raw: *mut AVCodecContext,
}

impl CodecContext {
    pub fn new(raw: *mut AVCodecContext) -> CodecContext {
        CodecContext { raw }
    }

    pub fn raw(&self) -> &AVCodecContext {
        unsafe { &*(self.raw as *const AVCodecContext) }
    }

    pub fn raw_mut(&self) -> &mut AVCodecContext {
        unsafe { &mut *(self.raw as *mut AVCodecContext) }
    }

    pub fn codec_id(&self) -> i32 {
        self.raw().codec_id
    }

    pub fn width(&self) -> u32 {
        self.raw().width as u32
    }

    pub fn height(&self) -> u32 {
        self.raw().height as u32
    }

    pub fn pix_fmt(&self) -> i32 {
        self.raw().pix_fmt
    }

    pub fn open(&mut self, codec: &DecoderCodec) -> Result<(), CodecContextError> {
        let old = self.raw;

        self.raw = unsafe {ffmpeg::avcodec_alloc_context3(codec.raw()) };

        if self.raw.is_null() {
            return Err(CodecContextError::AllocationError);
        }

        if unsafe { ffmpeg::avcodec_copy_context(self.raw_mut(), old) } != 0 {
            return Err(CodecContextError::CopyError);
        }

        self.raw_mut().thread_count = 0;
        self.raw_mut().thread_type = FF_THREAD_SLICE as i32;

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

    pub fn receive_frame(&mut self, input_frame: &mut Frame) -> i32 {
        unsafe { ffmpeg::avcodec_receive_frame(self.raw_mut(), input_frame.raw_mut()) }
    }
}