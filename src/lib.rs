
pub struct Person {
    name: String,
    age: i32
}

impl Person {
    pub fn new(name: &str, age: i32) -> Person {
        Person { name: name.to_string(), age }
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
