fn main() {
    println!("Hello, world!");
}


#[cfg(test)]
mod tests {
    #[test]
    fn first_test() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn can_connect_to_solana() {
        assert!(false);
    }
}