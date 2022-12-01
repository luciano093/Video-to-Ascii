use std::marker::PhantomData;

use rusty_ffmpeg::ffi as ffmpeg;
use ffmpeg::AVStream;

use crate::stream::Stream;

pub struct StreamIterator<'a> {
    _raw: *mut *mut AVStream,
    current: *mut AVStream,
    current_index: usize,
    max_streams: u32,
    _phantom_data: PhantomData<&'a ()>
}

impl<'a> StreamIterator<'a> {
    pub fn new(raw: *mut *mut AVStream, max_streams: u32) -> Result<StreamIterator<'a>, String> {
        let current = if raw.is_null() {
            return Err("Pointer is null".to_string());
        } else {
            unsafe { *raw }
        };

        Ok(StreamIterator { _raw: raw, current, current_index: 0, max_streams, _phantom_data: PhantomData })
    }
}

impl<'a> Iterator for StreamIterator<'a> {
    type Item = Stream<'a>;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        if self.current_index == self.max_streams as usize {
            return None;
        }

        let stream = unsafe { Stream::new(self.current.add(self.current_index)) };
        Some(stream)
    }
}