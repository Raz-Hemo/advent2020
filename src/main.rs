#[macro_use]
extern crate nom;

mod stages;

fn main() -> std::io::Result<()> {
    println!("your seat: {:?}", stages::stage5_2());

    Ok(())
}
