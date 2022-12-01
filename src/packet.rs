use rusty_ffmpeg::ffi as ffmpeg;
use ffmpeg::{av_packet_free, av_packet_unref};
use ffmpeg::AVPacket;

pub struct Packet {
    raw: *mut AVPacket,
    needs_unref: bool
}

impl Packet {
    pub fn new(raw: *mut AVPacket, needs_unref: bool) -> Packet {
        Packet { raw, needs_unref }
    }

    pub const fn raw(&self) -> &AVPacket {
        unsafe { &*(self.raw as *const AVPacket) }
    }

    pub fn raw_mut(&mut self) -> &mut AVPacket {
        unsafe { &mut *self.raw }
    }

    pub const fn stream_index(&self) -> i32 {
        self.raw().stream_index
    }
}

impl Drop for Packet {
    fn drop(&mut self) {
        if self.needs_unref {
            unsafe { av_packet_unref(self.raw_mut()) }
        } else {
            unsafe { av_packet_free((&mut self.raw_mut()) as *mut &mut AVPacket as *mut *mut AVPacket) };
        } 
    }
}