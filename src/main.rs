#[macro_use]
extern crate nom;

mod stages;

fn main() -> std::io::Result<()> {
    println!("first number: {:?}", stages::stage9_1());

    Ok(())
}
