#[macro_use]
extern crate nom;

mod stages;

fn main() -> std::io::Result<()> {
    println!("weakness: {:?}", stages::stage9_2());

    Ok(())
}
