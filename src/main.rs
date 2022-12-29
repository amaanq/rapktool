use crate::util::aapt_manager::{get_aapt2_path, get_aapt_execution_command};

mod util;

fn main() {
    println!("Hello, world!");

    let path = get_aapt2_path().unwrap();
    println!("path: {path:?}");

    let exe = get_aapt_execution_command(path).unwrap();
    println!("exe: {exe:?}");
}
