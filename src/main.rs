#[macro_use]
extern crate nom;

mod stages;

fn main() -> std::io::Result<()> {
    println!("product: {:?}", stages::stage10_1());

    Ok(())
}
