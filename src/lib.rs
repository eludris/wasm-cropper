mod utils;

use std::io::{BufReader, Cursor, Read};

use js_sys::{Array, Uint8Array};
use wasm_bindgen::prelude::wasm_bindgen;

use image::{
    codecs::{
        gif::{GifDecoder, GifEncoder, Repeat},
        png::PngEncoder,
    }, AnimationDecoder, Delay, ExtendedColorType, Frame, ImageEncoder, RgbaImage
};

use console_error_panic_hook;

use utils::crop_to_size;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen(js_name = "chunkGif")]
pub fn chunk_gif(buffer: Vec<u8>, chunks: usize) -> Array {
    let decoder = GifDecoder::new(Cursor::new(buffer)).unwrap();

    // Decode frames into a JavaScript Uint8Array, then collect.
    let frames = decoder
        .into_frames()
        .map(|f| {
            let frame = f.unwrap();
            let delay = frame.delay().numer_denom_ms();

            // To get around a number of limitations, we prepend important frame
            // data to the buffer of each frame, so we can extract it later.
            // We *would* do this with a more sensible approach such as a struct,
            // but due to a limitation in web-workers -- where the prototype of
            // an object is stripped when it is moved into/out of the worker --
            // we found that this is the most reliable way.
            let mut buffer: Vec<u8> = vec![];
            buffer.extend(&frame.left().to_be_bytes());
            buffer.extend(&frame.top().to_be_bytes());
            buffer.extend(&delay.0.to_be_bytes());
            buffer.extend(&delay.1.to_be_bytes());
            buffer.extend(&frame.into_buffer().into_vec());

            Uint8Array::from(&buffer[..])
        })
        .collect::<Vec<Uint8Array>>();

    // Chunk frames into `chunks` chunks of equal length.
    let chunk_size = (frames.len() / chunks) + ((frames.len() % chunks) > 0) as usize;
    frames
        .chunks(chunk_size)
        .map(|c| c.iter().collect::<Array>())
        .collect()
}

#[wasm_bindgen(js_name = "cropChunk")]
pub fn crop_chunk(
    buffer: Vec<Uint8Array>,
    w: u32,
    h: u32,
    sx: i32,
    sy: i32,
    sw: u32,
    sh: u32,
) -> Vec<Uint8Array> {
    // Crop a chunk to the desired size.
    buffer
        .into_iter()
        .map(|a| {
            let chunk_buf = a.to_vec();
            let mut reader = BufReader::new(&*chunk_buf);

            // Temporarily strip off/ignore.
            let mut header_buf = [0; 16];
            reader
                .read_exact(&mut header_buf)
                .expect("Failed to read header data.");

            let mut image_buf = vec![];
            reader
                .read_to_end(&mut image_buf)
                .expect("Failed to read buffer data.");

            let img =
                RgbaImage::from_vec(w, h, image_buf).expect("Failed to convert buffer to image.");

            let cropped = crop_to_size(img, sx, sy, sw, sh);

            // Re-prefix with the header data.
            let mut cropped_buf = header_buf.to_vec();
            cropped_buf.extend(cropped);

            Uint8Array::from(&cropped_buf[..])
        })
        .collect()
}

#[wasm_bindgen(js_name = "mergeFrames")]
pub fn merge_frames(buffer: Vec<Uint8Array>, w: u32, h: u32) -> Vec<u8> {
    let mut out = Cursor::new(vec![]);

    {
        let mut encoder = GifEncoder::new(&mut out);
        encoder.set_repeat(Repeat::Infinite).unwrap();
        encoder
            .encode_frames(buffer.into_iter().map(|a| {
                let chunk_buf = a.to_vec();
                let mut reader = BufReader::new(&*chunk_buf);

                // Now we actually extract the individual header items;
                // each item is a u32, so 4 items of our Vec<u8>.
                let mut header_buf = [0; 4];
                reader
                    .read_exact(&mut header_buf)
                    .expect("Failed to read left from header data.");
                let left = u32::from_be_bytes(header_buf);

                reader
                    .read_exact(&mut header_buf)
                    .expect("Failed to read top from header data.");
                let top = u32::from_be_bytes(header_buf);

                reader
                    .read_exact(&mut header_buf)
                    .expect("Failed to read numerator from header data.");
                let num = u32::from_be_bytes(header_buf);

                reader
                    .read_exact(&mut header_buf)
                    .expect("Failed to read denominator from header data.");
                let denom = u32::from_be_bytes(header_buf);

                // Now we extract the image data.
                let mut image_buf = vec![];
                reader
                    .read_to_end(&mut image_buf)
                    .expect("Failed to read image data from buffer.");

                let img = RgbaImage::from_vec(w, h, image_buf)
                    .expect("Failed to read buffer into RgbaImage.");
                Frame::from_parts(img, left, top, Delay::from_numer_denom_ms(num, denom))
            }))
            .expect("Failed to encode GIF.");
    }

    out.into_inner()
}

#[wasm_bindgen(js_name = "cropImage")]
pub fn crop_image(buffer: Vec<u8>, sx: i32, sy: i32, sw: u32, sh: u32) -> Vec<u8> {
    let img = image::load_from_memory(&buffer).expect("Failed to read buffer into image.");
    let buf = crop_to_size(img.into(), sx, sy, sw, sh);

    let mut out = Cursor::new(vec![]);
    PngEncoder::new(&mut out)
        .write_image(&buf[..], sw, sh, ExtendedColorType::Rgba8)
        .unwrap();

    out.into_inner()
}
