use std::io::Cursor;

use wasm_bindgen::prelude::wasm_bindgen;

use image::{
    codecs::{
        gif::{GifDecoder, GifEncoder, Repeat},
        png::PngEncoder,
    },
    imageops::{crop, resize, FilterType},
    AnimationDecoder, ExtendedColorType, Frame, ImageEncoder,
};

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
