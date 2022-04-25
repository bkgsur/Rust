use std::convert::TryInto;

fn main() {
    let a: i32 = 10;
    let b: u8 = 100;
    if a< (b as i32)
    {
        println!("{}", "10 is less than 100");
    }

}
