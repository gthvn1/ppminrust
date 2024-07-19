use ppminrust::ppm::{Operation, Ppm};

fn main() {
    let filename = "sample.ppm";
    let width = 800;
    let height = 500;

    Ppm::create(filename, width, height)
        .rasterize(Operation::Fill(0x00FF00)) // Fil in Green
        .rasterize(Operation::Id(0x0000FF)) // Trace a Blue line x = y
        .rasterize(Operation::Circle(20, 20, 15, 0xFF0000)) // Draw a red circle at 20x20 with radius
        // of 10
        .write()
        .unwrap();

    println!("{} has been written.", filename);
}
