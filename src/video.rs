use crate::frame::Frame;

pub struct Pixel {
    r: u8,
    g: u8,
    b: u8,
}

impl Pixel {
    pub const fn r(&self) -> u8 {
        self.r
    }

    pub const fn g(&self) -> u8 {
        self.g
    }

    pub const fn b(&self) -> u8 {
        self.b
    }
}

pub struct RGBFrame {
    width: u32,
    height: u32,
    data: Vec<Pixel>
}

impl RGBFrame {
    pub const fn width(&self) -> u32 {
        self.width
    }

    pub const fn height(&self) -> u32 {
        self.height
    }

    pub const fn data(&self) -> &Vec<Pixel> {
        &self.data
    }
}

pub struct Video {
    frames: Vec<RGBFrame>,
    fps: f64,
}

impl Video {
    pub fn new(raw_frames: &mut [Frame], fps: f64) -> Video {
        let mut frames = vec![];

        let width = raw_frames[0].width() as u32;
        let height = raw_frames[0].height() as u32;

        for raw_frame in raw_frames {
            let mut pixels = vec![];

            let mut i = 0;
            while i < (width * height * 3) as usize {
                unsafe { 
                    let r = *raw_frame.data_mut()[0].add(i + 0);
                    let g = *raw_frame.data_mut()[0].add(i + 1);
                    let b = *raw_frame.data_mut()[0].add(i + 2);

                    let pixel = Pixel { r, g, b };
                    pixels.push(pixel);
                }

                i += 3;
            }

            let frame = RGBFrame { width, height, data: pixels };
            frames.push(frame);
        }

        Video { frames, fps }
    }

    pub fn width(&self) -> u32 {
        self.frames[0].width
    }

    pub fn height(&self) -> u32 {
        self.frames[0].height
    }

    pub const fn frames(&self) -> &Vec<RGBFrame> {
        &self.frames
    }

    pub const fn fps(&self) -> f64 {
        self.fps
    }
}