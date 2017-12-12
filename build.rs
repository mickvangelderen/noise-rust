use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("gradients.rs");
    let mut f = File::create(&dest_path).unwrap();

    let angle_count = 32;
    let angle_offset = 0.001;

    f.write_all(format!("pub static GRADIENTS_2D: [(f32, f32); {}] = [", angle_count).as_bytes()).unwrap();
    for i in 0..angle_count {
        let angle = (i as f64)/(angle_count as f64)*std::f64::consts::PI*2.0 + angle_offset;

        let dx = angle.cos();
        let dy = angle.sin();

        f.write_all(format!("({}, {}),", dx as f32, dy as f32).as_bytes()).unwrap();
    }
    f.write(b"];").unwrap();
}
