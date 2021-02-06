#[macro_use]
extern crate nom;

mod stages;

fn main() -> std::io::Result<()> {
    println!("number of taken seats: {:?}", stages::stage11_2());

    Ok(())
}
