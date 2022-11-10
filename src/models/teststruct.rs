pub mod test{
    pub struct teststruct{
        value : i32,
        value2 : string,
    }

    impl teststruct {
        fn new() -> Self{
            Self{value: 10, value2: String::from("coucou")}
        }

        fn encode(&self) -> Vec<i8>{
            vec![]
        }

        fn decode() -> Self{
            new()
        }
    }
}


#[cfg(test)]
pub mod tests {
    use crate::check_age;

    #[test]
    fn test_check_age_out_of_range() {
        assert_eq!(check_age(200), false);
        assert_eq!(check_age(201), false);
        assert_eq!(check_age(0), false);
        assert_eq!(check_age(-1), false);
    }

    #[test]
    fn test_check_age_in_range() {
        assert_eq!(check_age(1), true);
        assert_eq!(check_age(199), true);
    }
}