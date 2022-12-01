use rusty_ffmpeg::ffi as ffmpeg;

use crate::codec_context::CodecContext;
use crate::decoder_codec::DecoderCodec;
use crate::error::{FormatContextError, ffmpegError};
use crate::format_context::FormatContext;
use crate::frame::Frame;
use crate::scaler_context::ScalerContext;
use crate::video::Video;

pub fn get_video_stream_index(format_context: &FormatContext) -> Result<isize, ffmpegError> {
    match format_context.video_stream_index() {
        Some(index) => Ok(index),
        None => Err(FormatContextError::NoVideoStream.into()),
    }
}

fn calculate_frame_count(path: &str, video_stream_index: i32) -> usize {
    let mut format_context = FormatContext::new(path).unwrap();

        let mut frame_count = 0;
        format_context.read_frames().for_each(|packet| {
            if packet.stream_index() == video_stream_index {
                frame_count += 1;
            }
        });

    frame_count
}

pub fn get_video(path: &str) -> Result<Video, ffmpegError> {
    let mut format_context = FormatContext::new(path)?;

    format_context.find_stream_info();

    let video_stream_index = get_video_stream_index(&format_context)?;
    
    let mut codec_context = CodecContext::from_format_context(&format_context);

    let codec = DecoderCodec::from_codec_id(codec_context.codec_id())?;

    codec_context.set_thread_count(0);
    codec_context.set_thread_type(1);

    codec_context.open(&codec)?;

    let (width, height) = (codec_context.width(), codec_context.height());

    let mut scaler = ScalerContext::new(
        width,
        height,
        codec_context.pix_fmt(),
        96,
        30,
        ffmpeg::AVPixelFormat_AV_PIX_FMT_RGB24,
        ffmpeg::SWS_BILINEAR
    )?;

    let mut input_frame = Frame::new()?;
    let mut output_frame = Frame::new()?;

    let frame_count = if format_context.streams().nth(video_stream_index as usize).unwrap().frame_count() == 0 {
        calculate_frame_count(path, video_stream_index as i32)
    } else {
        format_context.streams().nth(0).unwrap().frame_count() as usize
    };

    let mut vec = Vec::with_capacity(frame_count);

    for packet in format_context.read_frames() {
        if packet.stream_index() as isize == video_stream_index {
            codec_context.send_packet(&packet)?;

            while codec_context.receive_frame(&mut input_frame).is_ok() {
                scaler.scale(&input_frame, &mut output_frame)?;

                let frame = Frame::from_frame(&output_frame)?;
                vec.push(frame);
            }
        }
    };

    let fps = format_context.streams().nth(video_stream_index as usize).unwrap().average_fps();

    let video = Video::new(&mut vec, fps);

    Ok(video)
}