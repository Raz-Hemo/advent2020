#[macro_use]
extern crate nom;

mod stages;

fn main() -> std::io::Result<()> {
    println!("number of patahs: {:?}", stages::stage10_2());

    Ok(())
}
