pub fn caesar(code: &str, shift: u8) -> String
{
    code.chars()
    .map(|c| {
        if c.is_ascii_alphabetic()
        {
            let first = if c.is_ascii_lowercase()
            {
                b'a'
            }
            else
            {
                b'A'
            };           
            (first + (c as u8 + shift - first)%26) as char
        }
        else
        {
            c
        }
    }).collect()
}
#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    pub fn empty()
    {
        assert_eq!(caesar("",13), "");
    }
    #[test]
    pub fn caesar_rot_13()
    {
        assert_eq!(caesar("rust", 13), "ehfg");
    }
}
