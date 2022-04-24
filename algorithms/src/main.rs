mod cipher;
mod dcp;
mod search;

fn main() {
    //println!("{}",cipher::caesar("rusAt", 13));
    //let result = dcp::num_encodings2("111");
    //println!("{}",result);
    let mut root  = dcp::binarytree::binarytree_sample1();
    println!("{:#?}", root.content );
    
}
