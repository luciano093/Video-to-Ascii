use std::ffi::CString;
use std::ptr::null_mut;

use rusty_ffmpeg::ffi::{self as ffmpeg, av_packet_alloc};
use ffmpeg::AVFormatContext;

use crate::codec_context::CodecContext;
use crate::error::FileError;
use crate::packet::Packet;
use crate::packet_iterator::PacketIterator;
use crate::stream_iterator::StreamIterator;

pub struct FormatContext {
    raw: *mut AVFormatContext,
    path: CString,
    video_stream_index: Option<isize>,
}

impl FormatContext {
    pub fn new(path: &str) -> Result<FormatContext, FileError> {
        let mut format_context = null_mut();

        let path = CString::new(path).unwrap();
        
        unsafe { if ffmpeg::avformat_open_input(&mut format_context, path.as_ptr(), null_mut(), null_mut()) < 0 {
            return Err(FileError::UnableToOpenFile(path.into_string().unwrap()));
        }};

        let mut format_context = FormatContext { raw: format_context, path , video_stream_index: None};

        format_context.video_stream_index = format_context.calculate_video_stream_index();

        Ok(format_context)
    }

    pub fn raw(&self) -> *const AVFormatContext {
        self.raw
    }

    pub fn raw_mut(&mut self) -> *mut AVFormatContext {
        self.raw
    }

    pub fn as_ref(&self) -> &AVFormatContext {
        unsafe { &*(self.raw as *const AVFormatContext) }
    }

    pub fn as_ref_mut(&self) -> &mut AVFormatContext {
        unsafe { &mut *(self.raw as *mut AVFormatContext) }
    }

    pub fn find_stream_info(&mut self) {
        if unsafe {ffmpeg::avformat_find_stream_info(self.as_ref_mut(), null_mut()) } < 0 {
            panic!("av_find_stream_info() failed");
        }
    }

    pub fn av_dump_format(&mut self) {
        unsafe { ffmpeg::av_dump_format(self.as_ref_mut(),0, self.path.as_ptr(), 0) }
    }

    pub fn video_stream_index(&self) -> Option<isize> {
        self.video_stream_index
    }

    pub fn read_frames(&mut self) -> PacketIterator {
        PacketIterator::new(self)
    }

    pub fn streams(&self) -> StreamIterator {
        StreamIterator::new(self.as_ref().streams, self.as_ref().nb_streams).unwrap()
    }

    pub fn codec_context(&self) -> CodecContext {
        let mut streams = self.streams();

        let codec_context = streams.nth(self.video_stream_index.unwrap() as usize).unwrap().codec_mut();

        CodecContext::new(codec_context)
    }

    pub fn next_packet(&mut self) -> Option<Packet> {
        let packet = unsafe { av_packet_alloc() };

        if packet.is_null() {
            panic!("Failed to allocate packet");
        }
        
        match unsafe { ffmpeg::av_read_frame(self.as_ref_mut(), packet) } {
            0 => (),
            _ => return None,
        }

        if packet.is_null() {
            panic!("Error reading stream frame");
        }

        Some(Packet::new(packet))
    }

    fn calculate_video_stream_index(&self) -> Option<isize>{
        let mut video_stream_index = -1;

        for (i, stream) in self.streams().enumerate() {
            if stream.codec().codec_type == ffmpeg::AVMediaType_AVMEDIA_TYPE_VIDEO {
                video_stream_index = i as isize;
                break;
            }
        }

        if video_stream_index == -1 {
            return None;
        }

        Some(video_stream_index)
    }
}