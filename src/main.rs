#[macro_use]
extern crate nom;

mod stages;

fn main() -> std::io::Result<()> {
    // TODO modularize stage 4
    println!("highest seat: {}", stages::stage5_1());

    Ok(())
}
