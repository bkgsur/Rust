fn main() {
    
    same_bitsequence_diff_types();
    f32_as_u32();
}

pub fn same_bitsequence_diff_types()
{
    let a:u16 = 50115;
    let b: i16 = -15421;

    println!("a: {:016b} {}", a,a);
    println!("b: {:016b} {}", b,b);
}

pub fn f32_as_u32()
{
    let a: f32 = 42.42;
    let utype:u32 = unsafe
    {
        std::mem::transmute(a)
    };
    println!("a: {}", a);
    println!("{}",utype);
    println!("{:032b}",utype);
  
 
    let b:f32 = unsafe
    {
        std::mem::transmute(utype)
    };
    println!("b: {}", b);
    assert_eq!(a,b);
}
