pub fn add(a:i32,b:i32)->i32{
    a+b
}

pub fn subtract(a:i32,b:i32)-> i32
{
    a-b
}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_add()
    {
        assert_eq!(add(2,3),5);// pass
    }

    #[test]
    fn test_subtract()
    {
        assert_eq!(subtract(10,4),6); // pass
    }

    #[test]
    fn test_subtract_fail()
    {
        assert_eq!(subtract(10,4),5);//fail
    }
}
