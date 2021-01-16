#[macro_use]
extern crate nom;

mod stages;

fn main() -> std::io::Result<()> {
    println!("sum: {:?}", stages::stage7_1());

    Ok(())
}
