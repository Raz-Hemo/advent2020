#[macro_use]
extern crate nom;

mod stages;

fn main() -> std::io::Result<()> {
    println!("result: {:?}", stages::stage12_2());

    Ok(())
}
