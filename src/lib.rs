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

    if buffer.to_lowercase().contains("yes")
    {
        true
    }
    else
    {
        false
    }
}

pub fn vec_read<T: FromStr>(text: &Option<&str>) -> Option<Vec<T>>
{
    if text.is_some()
    {
        println!("{}", text.unwrap());
    }

    let mut vector: Vec<T> = vec![];
    let mut buffer = String::new();

    match std::io::stdin().read_line(&mut buffer)
    {
        Err(error) => { println!("Error in reading input: {}", error.to_string()); return None },
        _ => {}
    }

    let formated_string = buffer.trim().split_whitespace();

    for word in formated_string
    {
        match word.parse::<T>()
        {   
            Ok(result) => vector.push(result),
            _ => { println!("Error in: \"{}\"", word); vector.clear(); break }
        }
    }

    if vector.is_empty()
    {
        println!("Error in converting to vector!");
        return None
    }

    Some(vector)
}

#[cfg(test)]
mod tests
{
    use crate::*;

    fn numbers()
    {
        let result = generic_read::<i32>(&Some("Insert an integer"));
        println!("Result with text: {}", result.unwrap());
        println!("");

        let result = generic_read::<i32>(&None);
        println!("Result without text: {}", result.unwrap());

        println!("");
    }

    fn bool()
    {
        let result = bool_read(&Some("Type yes or no"));
        println!("Result bool: {}", result);
        println!("");
    }

    fn vector()
    {
        let result = vec_read::<i32>(&Some("Put a list of numbers"));

        println!("Result vector {:#?}", result.unwrap());
        println!("");
    }

    #[test]
    fn testing()
    {
        numbers();
        bool();
        vector();
    }
}