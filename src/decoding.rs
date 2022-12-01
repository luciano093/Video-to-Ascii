use rusty_ffmpeg::ffi as ffmpeg;

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

pub fn get_video(path: &str) -> Result<Video, ffmpegError> {
    let mut format_context = FormatContext::new(path)?;

    format_context.find_stream_info();

    let video_stream_index = get_video_stream_index(&format_context)?;
    
    let mut codec_context = format_context.codec_context();

    let codec = DecoderCodec::from_codec_id(codec_context.codec_id())?;
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

    let mut vec = vec![];

    for packet in format_context.read_frames() {
        if packet.stream_index() as isize == video_stream_index {
            codec_context.send_packet(&packet)?;

            let returned = codec_context.receive_frame(&mut input_frame);

            if returned == 0 {
                scaler.scale(&input_frame, &mut output_frame)?;

                let frame = Frame::from_frame(&output_frame)?;
                vec.push(frame);
            }
        }
    };

    let fps = format_context.streams().nth(0).unwrap().average_fps();
    println!("fps: {}", fps);

    let video = Video::new(&mut vec, fps);

    Ok(video)

    // // drain frames
    // unsafe { ffmpeg::avcodec_send_packet(codec_context, packet) };

    // while unsafe { ffmpeg::avcodec_receive_frame(codec_context, input_frame) } == 0 {
    //         unsafe {
    //             ffmpeg::sws_scale(
    //             conversion_context,
    //             (*input_frame).data.as_ptr() as *const *const u8,
    //             (*input_frame).linesize.as_ptr(),
    //             0,
    //             (*codec_context).height,
    //             (*output_frame).data.as_ptr(),
    //             (*output_frame).linesize.as_ptr()
    //         );
    //     }
    // } 
    
    // unsafe {
    //     ffmpeg::av_free(buffer as *mut std::ffi::c_void);
    //     ffmpeg::av_free(input_frame as *mut std::ffi::c_void);
    //     ffmpeg::avcodec_close(codec_context);
    //     ffmpeg::avformat_close_input(&mut format_context);
    // };
}

// fn calculate_frame_count(path: &str, video_stream_index: usize) -> i64 {
//     let mut input_context = ffmpeg::format::input(&path.to_string()).unwrap();

//         let mut frame_count = 0;
//         input_context.packets().for_each(|(stream, _)| {
//             if stream.index() == video_stream_index {
//                 frame_count += 1;
//             }
//         });

//     frame_count
// }