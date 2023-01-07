use rusty_ffmpeg::ffi as ffmpeg;
use ffmpeg::{av_packet_free, av_packet_unref, av_packet_alloc};
use ffmpeg::AVPacket;

pub struct Packet {
    raw: *mut AVPacket,
    needs_unref: bool
}

impl Packet {
    pub fn new(needs_unref: bool) -> Packet {
        let raw = unsafe { av_packet_alloc() };   

        if raw.is_null() {
            panic!("Failed to allocate packet");
        }

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

    pub fn is_empty(&self) -> bool {
        self.raw.is_null()
    }

    pub const fn needs_unref(&self) -> bool {
        self.needs_unref
    }

    pub fn unref(&mut self) {
        unsafe { av_packet_unref(self.raw_mut()) }
    }
}

impl Drop for Packet {
    fn drop(&mut self) {
        if self.needs_unref {
            unsafe { av_packet_unref(self.raw_mut()) }
        }

        unsafe { av_packet_free((&mut self.raw_mut()) as *mut &mut AVPacket as *mut *mut AVPacket) };
    }
}