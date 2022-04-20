
pub fn binarysearch(arr: &[i32], target:&i32) -> bool
{

    if arr.len()==0
    {
        return false;
    }

    let mut l =0;
    let mut r:usize = arr.len()-1;

    while l<=r 
    {
        let  mid = l + ((r-l)/2);
        

        if &arr[mid]==target
        {
            return true;
        }
        else if &arr[mid]<target
        {
            l = mid+1;
        }
        else
        {
            r = mid-1;
        }
    }

    false
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    pub fn binary_search_true()
    {
      assert_eq!( binarysearch(&[1,2,3,4,5], &5), true);
    }
    #[test]
    pub fn binary_search_fail()
    {
      assert_eq!( binarysearch(&[], &5), false);
    }
}