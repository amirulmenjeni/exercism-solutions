/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {

    if let Some(_) = code.chars().find(|&x| !(x.is_digit(10) || x == ' ')) {
        return false;
    }
    
    let digits: Vec<u32> = code
        .chars()
        .filter(|&x| x.is_digit(10))
        .map(|x| x.to_digit(10).unwrap())
        .rev()
        .collect();

    if digits.len() == 1 {
        return false;
    }

    println!("digits: {:?}", digits);
    
    let sum = digits
        .iter()
        .enumerate()
        .map(|(i, &digit)| {
            let dbl = digit * 2;
            match i % 2 {
                0 => digit,
                _ => match dbl {
                    10.. => dbl - 9,
                    _ => dbl,
                },
            }
        })
        .inspect(|x| print!("{},", x))
        .sum::<u32>();

    match sum % 10 {
        0 => true,
        _ => false,
    }
}
