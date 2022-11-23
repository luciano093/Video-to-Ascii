use ffmpeg_next as ffmpeg;

use ffmpeg::frame;
use ffmpeg::software::scaling;
use ffmpeg::format::{input, Pixel};
use ffmpeg::media::Type;
use ffmpeg::codec;

use crate::error::FileError;

fn calculate_frame_count(path: &str, video_stream_index: usize) -> i64 {
    let mut input_context = ffmpeg::format::input(&path.to_string()).unwrap();

        let mut frame_count = 0;
        input_context.packets().for_each(|(stream, _)| {
            if stream.index() == video_stream_index {
                frame_count += 1;
            }
        });

    frame_count
}

pub fn get_video(path: &str) -> Result<crate::video::Video, FileError> {
    ffmpeg::init().unwrap();

    let mut input_context = match input(&path.to_string()) {
        Ok(context) => context,
        Err(_) => {
            return Err(FileError::NoFile);
        }
    };

    let input = match input_context.streams().best(Type::Video).ok_or(ffmpeg::Error::StreamNotFound) {
        Ok(input) => input,
        Err(_) => {
            return Err(FileError::InvalidFile);
        }
    };

    let video_stream_index = input.index();

    let frame_count = if input.frames() == 0 {
        calculate_frame_count(path, video_stream_index)
    } else {
        input.frames()
    };

    let fps = input.avg_frame_rate().0 as f64 / input.avg_frame_rate().1 as f64;

    let mut codec_context = codec::context::Context::from_parameters(input.parameters()).unwrap();
    
    let count = ffmpeg::threading::Config::count(0);
    let kind = ffmpeg::threading::Config::kind(ffmpeg::threading::Type::Frame);

    codec_context.set_threading(count);
    codec_context.set_threading(kind);

    let mut decoder = codec_context.decoder().video().unwrap();

    // used to change frame resolution to 96x30 and pixel format to rgb
    let mut scaler = scaling::Context::get(
        decoder.format(),
        decoder.width(),
        decoder.height(),
        Pixel::RGB24,
        96,
        30,
        scaling::Flags::BILINEAR
    ).unwrap();

    let mut frames = vec![];
    frames.reserve(frame_count as usize);
    
    input_context.packets().for_each(|(stream, packet)| {
        if stream.index() == video_stream_index {
            decoder.send_packet(&packet).unwrap();

            let decoded_frames = receive_and_process_decoded_frames(&mut decoder, &mut scaler).unwrap();

            frames.extend(decoded_frames);
        }
    });

    decoder.send_eof().unwrap();
    
    let video = crate::video::Video::new(&frames, fps);

    Ok(video)
}

fn receive_and_process_decoded_frames<'a>(decoder: &mut ffmpeg::decoder::Video, scaler: &'a mut scaling::Context) -> 
    Result<Vec<ffmpeg::frame::Video>, ffmpeg::Error>
{
    let mut frames = vec![];
    let mut decoded = frame::Video::empty();

    while decoder.receive_frame(&mut decoded).is_ok() {
        let mut video_frame = frame::Video::empty();

        scaler.run(&decoded, &mut video_frame)?;

        frames.push(video_frame);
    }

    Ok(frames)
}