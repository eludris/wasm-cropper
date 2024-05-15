use std::io::Cursor;

use js_sys::{Array, Uint8Array};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

use image::{
    codecs::{
        gif::{GifDecoder, GifEncoder, Repeat},
        png::PngEncoder,
    }, imageops::{crop, resize, FilterType}, AnimationDecoder, Delay, ExtendedColorType, Frame, ImageEncoder, RgbaImage
};

use console_error_panic_hook;

use wasm_bindgen_test::console_log;
// use itertools::Itertools;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
    console_log!("wee");
}

#[wasm_bindgen]
pub struct FrameInfo {
    delay: Delay,
    left: u32,
    top: u32,
    buffer: Uint8Array,
}


impl FrameInfo {
    fn from_frame(frame: Frame) -> Self {
        Self {
            delay: frame.delay(),
            left: frame.left(),
            top: frame.top(),
            buffer: Uint8Array::from(&frame.into_buffer().into_vec()[..])
        }
    }

    fn update_buffer(&mut self, buffer: Vec<u8>) {
        self.buffer = Uint8Array::from(&buffer[..]);
    }

    fn into_frame(self, w: u32, h: u32) -> Frame {
        let img = RgbaImage::from_vec(w, h, self.buffer.to_vec()).unwrap();
        Frame::from_parts(img, self.left, self.top, self.delay)
    }
}


#[wasm_bindgen(js_name = "chunkGif")]
pub fn chunk_gif(buffer: Vec<u8>, chunks: usize) -> Array
{
    let decoder = GifDecoder::new(Cursor::new(buffer)).unwrap();
    let frames = decoder
        .into_frames()
        .map(|f| {
            let xd = FrameInfo::from_frame(f.unwrap());
            JsValue::from(xd)
        })
        .collect::<Vec<JsValue>>();

    let chunk_size = (frames.len() / chunks) + ((frames.len() % chunks) > 0) as usize;
     frames
        .chunks(chunk_size)
        .map(|c| c.iter().collect::<Array>())
        .collect()
}


#[wasm_bindgen(js_name = "cropChunk")]
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


#[wasm_bindgen(js_name = "combineChunks")]
pub fn combine_chunks(
    chunks: Vec<FrameInfo>, w: u32, h: u32,
) -> Vec<u8> {
    let mut out = Cursor::new(vec![]);
    {
        let mut encoder = GifEncoder::new(&mut out);
        encoder.encode_frames(
            chunks.into_iter().map(|f| f.into_frame(w, h)).collect::<Vec<Frame>>()
        ).unwrap();
        encoder.set_repeat(Repeat::Infinite).unwrap();
    }
    out.into_inner()
}


#[wasm_bindgen(js_name = "cropGif")]
pub fn crop_gif(buffer: Vec<u8>, sx: u32, sy: u32, sw: u32, sh: u32, dw: u32, dh: u32) -> Vec<u8> {
    let decoder = GifDecoder::new(Cursor::new(buffer)).unwrap();
    let frames: Vec<Frame> = decoder
        .into_frames()
        .map(|f| {
            let mut frame = f.unwrap();
            let mut cropped = crop(frame.buffer_mut(), sx, sy, sw, sh);
            let resized = resize(&mut *cropped, dw, dh, FilterType::Nearest);
            Frame::from_parts(resized, frame.left(), frame.top(), frame.delay())
        })
        .collect();

    let mut out = Cursor::new(vec![]);
    {
        let mut encoder = GifEncoder::new(&mut out);
        encoder.encode_frames(frames).unwrap();
        encoder.set_repeat(Repeat::Infinite).unwrap();
    }
    out.into_inner()
}


#[wasm_bindgen(js_name = "cropImage")]
pub fn crop_image(
    buffer: Vec<u8>,
    sx: u32,
    sy: u32,
    sw: u32,
    sh: u32,
    dw: u32,
    dh: u32,
) -> Vec<u8> {
    let mut img = image::load_from_memory(&buffer).unwrap();
    let mut cropped = crop(&mut img, sx, sy, sw, sh);
    let resized = resize(&mut *cropped, dw, dh, FilterType::Nearest);

    let mut out = Cursor::new(vec![]);
    PngEncoder::new(&mut out)
        .write_image(&resized, dw, dh, ExtendedColorType::Rgba8)
        .unwrap();

    out.into_inner()
}
