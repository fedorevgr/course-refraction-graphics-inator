mod renderer;

fn main() {

    use std::fs::OpenOptions;
    let mut file = OpenOptions::new().read(true).open("mesh.stl").unwrap();
    let stl = stl_io::read_stl(&mut file).unwrap();
}
