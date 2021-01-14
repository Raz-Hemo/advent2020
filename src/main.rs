#[macro_use]
extern crate nom;

mod stages;

fn main() -> std::io::Result<()> {
    println!("highest seat: {}", stages::stage5_1());

    Ok(())
}
