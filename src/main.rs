#[macro_use]
extern crate nom;

mod stages;

fn main() -> std::io::Result<()> {
    println!("sum: {:?}", stages::stage6_2());

    Ok(())
}
