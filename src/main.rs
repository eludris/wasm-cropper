// This file just serves to test the implementation;
// anything here is a reimplementation of everything in lib.rs, but in
// native rust types (hence there is no use::wasm_bindgen).

use std::io::{BufReader, Cursor};
use std::{fs::File, io::Read};

use image::{
    codecs::gif::{GifDecoder, GifEncoder, Repeat},
    imageops::crop,
    AnimationDecoder, Delay, Frame, RgbaImage,
};

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub fn chunk_gif(buffer: Vec<u8>, chunks: usize) -> Vec<Vec<Vec<u8>>> {
    let decoder = GifDecoder::new(Cursor::new(buffer)).unwrap();
    let frames: Vec<Vec<u8>> = decoder
        .into_frames()
        .map(|f| {
            let frame = f.unwrap();
            let delay = frame.delay().numer_denom_ms();

            let mut buffer: Vec<u8> = vec![];
            buffer.extend(&frame.left().to_be_bytes());
            buffer.extend(&frame.top().to_be_bytes());
            buffer.extend(&delay.0.to_be_bytes());
            buffer.extend(&delay.1.to_be_bytes());
            buffer.extend(&frame.into_buffer().into_vec());

            buffer
        })
        .collect();

    let chunk_size = (frames.len() / chunks) + ((frames.len() % chunks) > 0) as usize;
    frames.chunks(chunk_size).map(|c| c.to_vec()).collect()
}

pub fn crop_chunk(
    buffer: Vec<Vec<u8>>,
    w: u32,
    h: u32,
    sx: u32,
    sy: u32,
    sw: u32,
    sh: u32,
) -> Vec<Vec<u8>> {
    buffer
        .into_iter()
        .map(|a| {
            let a_vec = a.to_vec();
            let mut reader = BufReader::new(&*a_vec);

            let mut buf = [0; 16];
            reader
                .read_exact(&mut buf)
                .expect("Failed to read header data.");

            let mut actual_buf = vec![];
            reader
                .read_to_end(&mut actual_buf)
                .expect("Failed to read buffer data.");

            let mut buf1 = a_vec.clone();
            let buf2 = buf1.split_off(16);

            println!(
                "buf1 {}, buf {}, buf2 {}, actual_buf {}",
                buf1.len(),
                buf.len(),
                buf2.len(),
                actual_buf.len()
            );

            let mut img =
                RgbaImage::from_vec(w, h, actual_buf).expect("Failed to convert buffer to image.");
            let cropped = crop(&mut img, sx, sy, sw, sh);

            let mut buf_fr = buf.to_vec();
            buf_fr.extend(cropped.to_image().into_vec());

            buf_fr
        })
        .collect()
}

pub fn merge_frames(buffer: Vec<Vec<u8>>, w: u32, h: u32) -> Vec<u8> {
    let mut out = Cursor::new(vec![]);

    {
        let mut encoder = GifEncoder::new(&mut out);
        encoder.set_repeat(Repeat::Infinite).unwrap();
        encoder
            .encode_frames(buffer.into_iter().map(|a| {
                let a_vec = a.to_vec();
                let mut reader = BufReader::new(&*a_vec);

                let mut buf = [0; 4];
                reader
                    .read_exact(&mut buf)
                    .expect("Failed to read left from header data.");
                let left = u32::from_be_bytes(buf);

                reader
                    .read_exact(&mut buf)
                    .expect("Failed to read top from header data.");
                let top = u32::from_be_bytes(buf);

                reader
                    .read_exact(&mut buf)
                    .expect("Failed to read numerator from header data.");
                let num = u32::from_be_bytes(buf);

                reader
                    .read_exact(&mut buf)
                    .expect("Failed to read denominator from header data.");
                let denom = u32::from_be_bytes(buf);

                let mut buf_fr = vec![];
                reader
                    .read_to_end(&mut buf_fr)
                    .expect("Failed to read image data from buffer.");

                let img = RgbaImage::from_vec(w, h, buf_fr)
                    .expect("Failed to read buffer into RgbaImage.");
                Frame::from_parts(img, left, top, Delay::from_numer_denom_ms(num, denom))
            }))
            .expect("Failed to encode GIF.");
    }

    out.into_inner()
}

fn main() {
    let mut buf = vec![];
    let mut file = File::open("./test/silverash.gif").unwrap();
    file.read_to_end(&mut buf).unwrap();

    let chunks = 4;
    let chunked = chunk_gif(buf, chunks);

    let mut data: Vec<Vec<u8>> = vec![];
    for i in 0..chunks {
        data.extend(crop_chunk(
            chunked[i].to_owned(),
            500,
            500,
            100,
            100,
            300,
            300,
        ))
    }

    let _new_buf = merge_frames(data, 300, 300);

    // If this is reached, everything worked, yay!
    println!("we did a thing");
}
