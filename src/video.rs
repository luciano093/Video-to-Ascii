use ffmpeg_next::frame;

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

pub struct Frame {
    width: u32,
    height: u32,
    data: Vec<Pixel>
}

impl Frame {
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
    frames: Vec<Frame>,
    fps: f64,
}

impl Video {
    pub fn new(raw_frames: &[frame::Video], fps: f64) -> Video {
        let mut frames = vec![];

        let width = raw_frames[0].width();
        let height = raw_frames[0].height();

        for raw_frame in raw_frames {
            let mut pixels = vec![];

            for rgb in raw_frame.data(0).windows(3).step_by(3) {
                let pixel = Pixel { r: rgb[0], g: rgb[1], b: rgb[2] };
                pixels.push(pixel);
            }

            let frame = Frame { width, height, data: pixels };
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

    pub const fn frames(&self) -> &Vec<Frame> {
        &self.frames
    }

    pub const fn fps(&self) -> f64 {
        self.fps
    }
}