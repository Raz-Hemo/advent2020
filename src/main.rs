#[macro_use]
extern crate nom;

mod stages;

fn main() -> std::io::Result<()> {
    println!("acc: {:?}", stages::stage8_2());

    Ok(())
}
