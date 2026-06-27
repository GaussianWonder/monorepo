pub fn hello(name: &str) -> String {
    println!("within {{ name }}");
    format!("Hello, {}! This is {{ name }}.", name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = hello("world");
        assert_eq!(result, "Hello, world! This is {{ name }}.");
    }
}
