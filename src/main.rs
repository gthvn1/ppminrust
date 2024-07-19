use ppminrust::ppm::{Operation, Ppm};

fn main() {
    let filename = "sample.ppm";
    let width = 800;
    let height = 500;

    let mut ppm = Ppm::create(filename, width, height);

    ppm.rasterize(Operation::Fill(0x00FF00));
    ppm.rasterize(Operation::Id(0x0000FF));
    ppm.write().unwrap();

    println!("{} has been written.", filename);
}
