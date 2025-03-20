use ray_tracing_one_week::image::pixel::Pixel;

fn main() {
    let pixel = Pixel::new(0, 0, 255, 255, 255);

    println!("{:?}", pixel.get_color());
}
