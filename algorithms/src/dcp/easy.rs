pub fn cansum(arr: &mut[i32], target: i32) -> bool
{
   arr.sort();
    let mut l=0;
    let mut r = arr.len()-1;
    while l<r
    {
        if arr[l] + arr[r] == target
        {
            return true
        }
        else if arr[l] + arr[r] < target
        {
            l+=1;
        }
        else
        {
            r-=1;
        }
    }

    false
}

#[cfg(test)]
mod tests
{
    use super::*;
    #[test]
    pub fn cansum_true()
    {
        assert_eq!(cansum(&mut [10, 15, 3, 7], 17),true)
    }

    #[test]
    pub fn cansum_false()
    {
        assert_eq!(cansum(&mut [10, 15, 3, 7], 108),false)
    }

}