use image::{
    imageops::{crop, overlay},
    Rgba, RgbaImage,
};

pub fn crop_to_size(mut image: RgbaImage, sx: i32, sy: i32, sw: u32, sh: u32) -> Vec<u8> {
    if sx < 0 || sy < 0 {
        let mut container = RgbaImage::from_pixel(sw, sh, Rgba::from([0; 4]));
        overlay(&mut container, &image, -sx as i64, -sy as i64);
        container.into_vec()
    } else {
        let cropped = crop(&mut image, sx as u32, sy as u32, sw, sh);
        cropped.to_image().into_vec()
    }
}
