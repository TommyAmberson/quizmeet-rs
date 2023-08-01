use std::path::Path;

use quizmeet_rs_io::parse_ods::read_from_file;

fn main() {
    env_logger::init();

    let d1q1 = read_from_file(Path::new("../ods/D1Q1.ods")).unwrap();

    log::info!("{d1q1:#?}");
}
