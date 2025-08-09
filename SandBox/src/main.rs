use image::Rgb;

type RgbImage = image::RgbImage;

fn main() {
    let mut image = RgbImage::new(40, 60);
    image.put_pixel(0, 0, Rgb([255, 0, 255]));
    println!("{:?}", image);
}
