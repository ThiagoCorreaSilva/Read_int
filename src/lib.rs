use std::str::FromStr;

pub fn generic_read<T: FromStr>(text: &Option<&str>) -> Option<T>
{
    if text.is_some()
    {
        println!("{}", text.unwrap());
    }

    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Error in reading!");

    match buffer.trim().parse::<T>() 
    {
        Ok(value) => Some(value),
        _ => None
    }
}

pub fn bool_read(text: &Option<&str>) -> bool
{
    if text.is_some()
    {
        println!("{}", text.unwrap());
    }

    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Error in reading!");

    if buffer.contains("yes")
    {
        true
    }
    else
    {
        false
    }
}

#[cfg(test)]
mod tests
{
    use crate::*;

    #[test]
    fn numbers()
    {
        let result = generic_read::<i32>(&Some("With text"));
        println!("Generic: {}", result.unwrap());

        let result = generic_read::<u32>(&None);
        println!("Generic: {}",  result.unwrap());
    }
}