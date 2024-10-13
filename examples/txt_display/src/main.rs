use base64::{engine::general_purpose::STANDARD, Engine as _};
fn main() {
    println!("{}", STANDARD.encode(include_str!("txt.txt")))
}
