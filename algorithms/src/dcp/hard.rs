//Aug 10,2021

pub fn productallbutindex(arr: &mut [i32]) -> &[i32]
{
    
    let mut prod=1;
    
    for i in 0..arr.len()
    {        
        prod = prod* arr[i];
    }

    for j in 0..arr.len()
    {
        
        arr[j] = prod/arr[j];
    }

    arr
}

pub fn productallbutindex2(arr: &mut [i32]) -> &[i32]
{
    

    let mut prefix = vec![1; arr.len()];
     
    
    let mut suffix = vec![1; arr.len()];

    for i in 0..arr.len()
    {
        if i==0 
        {
            prefix[i] =1;
        }
        else
        {
            prefix[i] = arr[i-1]* prefix[i-1];
        }
    }

    for i in (0..arr.len()).rev()
    {
        if i == arr.len()-1
        {
            suffix[i] = 1;
        }
        else
        {
            suffix[i] = arr[i+1]* suffix[i+1];
        }
    }

    let res = arr;

    for i in 0..res.len()
    {
        if i==0
        {
            res[i] = suffix[i];
        }
        else if i == res.len()-1
        {
            res[i] = prefix[1];
        }
        else
        {
            res[i] = prefix[1] * suffix[i];
        }
    }
    res
    
}

#[cfg(test)]

mod tests
{
    use super::*;

    #[test]
    pub fn productallbutindex_success()
    {
        assert_eq!(productallbutindex(&mut [1, 2, 3, 4, 5]), &[120, 60, 40, 30, 24]);
    }

    #[test]
    pub fn productallbutindex2_success()
    {
        assert_eq!(productallbutindex(&mut [1, 2, 3, 4, 5]), &[120, 60, 40, 30, 24]);
    }
}

//August 14

pub fn Xor_linkedlist()
{

}
