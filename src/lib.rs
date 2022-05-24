pub fn hello(name: &str, invert: bool) {
    if invert {
        println!("!{} ,olleH", name.chars().rev().collect::<String>());
    } else {
        println!("Hello, {}!", name);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
