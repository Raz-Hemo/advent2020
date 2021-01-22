#[macro_use]
extern crate nom;

mod stages;

fn main() -> std::io::Result<()> {
    println!("total bags: {:?}", stages::stage7_2(String::from("shiny gold")));

    Ok(())
}
