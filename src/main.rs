#[macro_use]
extern crate nom;

mod stages;

fn main() -> std::io::Result<()> {
    println!("count: {:?}", stages::stage7_1(String::from("shiny gold")));

    Ok(())
}
