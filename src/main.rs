fn ret_two() -> u8 {
    2u8
}
fn ret_one() -> u8 {
    1u8
}

fn main() {
    println!("One: {}", ret_one());
    println!("Two: {}", ret_two());
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ret_one() {
        assert_eq!(1, ret_one());
    }

    #[test]
    fn test_ret_two() {
        assert_eq!(2, ret_two());
    }
}
