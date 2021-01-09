#[macro_use]
extern crate nom;

mod stages;

fn main() -> std::io::Result<()> {
    println!("valid passport count: {}", stages::stage4_1());

    Ok(())
}
