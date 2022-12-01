use rusty_ffmpeg::ffi::{self as ffmpeg, av_packet_free};
use ffmpeg::AVPacket;

pub struct Packet {
    raw: *mut AVPacket
}

impl Packet {
    pub fn new(raw: *mut AVPacket) -> Packet {
        Packet { raw }
    }

    pub fn raw(&self) -> &AVPacket {
        unsafe { &*self.raw }
    }

    pub fn raw_mut(&mut self) -> &mut AVPacket {
        unsafe { &mut *self.raw }
    }

    pub fn stream_index(&self) -> i32 {
        self.raw().stream_index
    }
}

impl Drop for Packet {
    fn drop(&mut self) {
        unsafe { av_packet_free((&mut self.raw_mut()) as *mut &mut AVPacket as *mut *mut AVPacket) };
    }
}