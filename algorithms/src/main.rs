mod cipher;
mod dcp;
mod search;

fn main() {
    //println!("{}",cipher::caesar("rusAt", 13));
    let result = dcp::num_encodings2("111");
    println!("{}",result);
    
}
