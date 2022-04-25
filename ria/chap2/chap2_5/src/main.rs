use std::convert::TryInto;
use num::complex::Complex;
fn main() {
    let a: i32 = 10;
    let b: u8 = 100;
    if a< (b as i32)
    {
        println!("{}", "10 is less than 100");
        println!("{}", 300_i32 as i8);
    }

    let b_ = b.try_into().unwrap();
    if a< b_
    {
        println!("{}", "10 is less than 100");
        
    }

   // floatingpointnum1();
    //floatingpointnum2();
    //floatingpointnum3();
    //nan1();
    //nan2();
    complexnum1();
}
pub fn floatingpointnum1()
{
    assert!(0.1 + 0.2 == 0.3);
}

pub fn floatingpointnum2()
{
    let abc: (f32,f32,f32) =  (0.1,0.2,0.3);
    println!(" abc (f32)");
    println!(" 0.1 + 0.2 : {:x}", (abc.0 + abc.1).to_bits());
    println!(" 0.3: {:x}", (abc.2).to_bits());

    let xyz: (f64,f64,f64) =  (0.1,0.2,0.3);
    println!(" xyz (f64)");
    println!(" 0.1 + 0.2 : {:x}", (xyz.0 + xyz.1).to_bits());
    println!(" 0.3: {:x}", (xyz.2).to_bits());
}

pub fn floatingpointnum3()
{
    let result: f64 = 0.1+0.2;
    let desired: f64 = 0.3;
    assert!((result-desired).abs()<=f64::EPSILON);
}

pub fn nan1()
{
    let x = (-42.0_f32).sqrt();
    assert!(x==x);
}

pub fn nan2()
{
    let x:f32 = 1.1/0.0;
    assert!(!x.is_finite());
    println!("{}",x);
}

pub fn complexnum1()
{
    let c1 = Complex{ re:1.2, im:-1.2};
    let c2 = Complex::new(1.1,22.2);
    let result = c1+c2;
    println!(" c1 + c2 = {}", result);
    println!("{}, {}i", result.re, result.im);
}