use ppminrust::ppm::PPM;

fn main() {
    let filename = "sample.ppm";
    let width = 10;
    let height = 10;

    let mut ppm = PPM::create(filename, width, height);

    ppm.write_header().unwrap();
    ppm.rasterize().unwrap();

    println!("{} has been written.", filename);
}
