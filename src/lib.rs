use std::io::Write;

pub mod decoding;
pub mod error;
pub mod video;
pub mod format_context;
pub mod codec_context;
pub mod stream_iterator;
pub mod stream;
pub mod packet_iterator;
pub mod packet;
pub mod decoder_codec;
pub mod frame;
pub mod scaler_context;
pub mod codec_parameters;

fn get_char(grayscale: f64) -> char {
    // dark(black) -> light(white)
    let chars = " .:-=+*#%@";
    
    // let chars = "@MBHENR#KWXDFPQASUZbdehx*8Gm&04LOVYkpq5Tagns69owz$CIu23Jcfry%1v7l+it[] {}?j|()=~!-/<>\"^_';,:`. ".chars().rev().collect::<String>();

    let max = chars.len();

    let index = (0.0 + ((max as f64 - 0.0) / (1.0 - 0.0) * (grayscale - 0.0))) as usize;

    chars.chars().nth(index).unwrap()
}

pub fn print_frame(frame: &video::RGBFrame) {
    let mut output = String::new();

    let mut col = 1;

    for pixel in frame.data() {
        let scale = ((0.2126 * pixel.r() as f64) + (0.7152 * pixel.g() as f64) + (0.0722 * pixel.b() as f64)) / 255.0;
        
        output += &get_char(scale).to_string();

        if col == frame.width() {
            output += "\n            ";
            col = 0;
        }

        col += 1;
    }

    print!("{}", output);
    std::io::stdout().flush().unwrap();
}