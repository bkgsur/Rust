//aug 11 2021

pub fn serialize_deserialize()
{

}


//Aug 15, 2021
//O(2^n)
pub fn num_encodings1(s:&str) -> i32{


    if s.len()<=1
    {   
        return 1;

    }
    if &s.chars().nth(0).unwrap() == &'0'
    {
        return 0;
    }
    let mut  total =0;
    if s[0..2].parse::<i32>().unwrap() <= 26
    {
        total +=  num_encodings1(&s[2..]);
    }

    total +=  num_encodings1(&s[1..]);

    total
}

//cache
pub fn num_encodings2(s:&str) -> i32
{

    let mut cache =  vec![0; s.len()+1];
    cache[0]=1;
    cache[1]=1;

    for i in 2..s.len()+1 as usize
    {
       let mut prev = &s[i-1..i];
        if  prev != "0"
        {
            cache[i] = cache[i-1];
        }
        let mut prev_2 = &s[i-2..i].parse::<i32>().unwrap();
        if prev_2 <= &26
        {
            cache[i] += cache[i - 2];
        }        

    }
    

    cache[s.len()]
}
#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    pub fn num_encodings1_pass()
    {
        assert_eq!(num_encodings1("111"),3);
    }

    #[test]
    pub fn num_encodings1_pass_2()
    {
        assert_eq!(num_encodings1(""),1);
    }

    #[test]
    pub fn num_encodings1_pass_3()
    {
        assert_eq!(num_encodings1("123"),3);
    }

    #[test]
    pub fn num_encodings1_fail_1()
    {
        assert_eq!(num_encodings1("011"),0);
    }

    #[test]
    pub fn num_encodings2_pass()
    {
        assert_eq!(num_encodings2("111"),3);
    }

    
}
