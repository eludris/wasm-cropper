use std::{fs::File, io::Read};
use std::io::Cursor;


use image::{
    codecs::{
        gif::{GifDecoder, GifEncoder, Repeat},
        png::PngEncoder,
    }, imageops::{crop, resize, FilterType}, ImageDecoder, AnimationDecoder, Delay, ExtendedColorType, Frame, ImageBuffer, ImageEncoder, Rgba, RgbaImage, SubImage
};

use serde::{Serialize, Deserialize};
use wasm_bindgen_test::console_log;
// use itertools::Itertools;


#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


#[derive(Clone)]
pub struct FrameInfo {
    delay: Delay,
    left: u32,
    top: u32,
    buffer: Vec<u8>,
}


impl FrameInfo {
    fn from_frame(frame: Frame) -> Self {
        Self {
            delay: frame.delay(),
            left: frame.left(),
            top: frame.top(),
            buffer: frame.into_buffer().into_vec()
        }
    }

    fn update_buffer(&mut self, buffer: Vec<u8>) {
        self.buffer = buffer;
    }

    fn into_frame(self, w: u32, h: u32) -> Frame {
        let img = RgbaImage::from_vec(w, h, self.buffer.to_vec()).unwrap();
        Frame::from_parts(img, self.left, self.top, self.delay)
    }
}


pub fn chunk_gif(buffer: Vec<u8>, chunks: usize) -> Vec<Vec<FrameInfo>>
{
    let decoder = GifDecoder::new(Cursor::new(buffer)).unwrap();
    let frames: Vec<FrameInfo> = decoder
        .into_frames()
        .map(|f| {
            FrameInfo::from_frame(f.unwrap())
        })
        .collect();

    let chunk_size = (frames.len() / chunks) + ((frames.len() % chunks) > 0) as usize;
     frames
        .chunks(chunk_size)
        .map(|c| c.to_vec())
        .collect()
}


pub fn crop_chunk(
    buffer: Vec<FrameInfo>,
    w: u32,
    h: u32,
    sx: u32,
    sy: u32,
    sw: u32,
    sh: u32,
) -> Vec<FrameInfo> {
    buffer
        .into_iter()
        .map(|mut f| {
            let mut img = RgbaImage::from_vec(w, h, f.buffer.to_vec()).unwrap();
            let cropped = crop(&mut img, sx, sy, sw, sh);
            f.update_buffer(cropped.to_image().into_vec());
            f
        })
        .collect()
}


pub fn combine_chunks(
    chunks: Vec<FrameInfo>, w: u32, h: u32,
) -> Vec<u8> {
    let mut out = Cursor::new(vec![]);
    {
        let mut encoder = GifEncoder::new(&mut out);
        encoder.set_repeat(Repeat::Infinite).unwrap();
        encoder.encode_frames(
            chunks.into_iter().map(|f| {
                print!("issa frame");
                f.into_frame(w, h)}
            ).collect::<Vec<Frame>>()
        ).unwrap();
    }
    out.into_inner()
}


fn main() {
    let mut buf = vec![];
    let mut file = File::open("./test/silverash.gif").unwrap();
    file.read_to_end(&mut buf).unwrap();

    let chunks = 4;
    let chunked = chunk_gif(buf, chunks);

    let mut data: Vec<FrameInfo> = vec![];
    for i in 0..chunks {
        data.extend(
            crop_chunk(chunked[i].to_owned(), 500, 500, 100, 100, 200, 200)
        )
    }

    let new_buf = combine_chunks(data, 200, 200);

    image::save_buffer(
        "testgif.gif",
        &new_buf,
        500,
        500,
        ExtendedColorType::Rgba8,
    ).unwrap();
}
