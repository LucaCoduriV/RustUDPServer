pub mod test{
    use std::io::Read;
    use std::mem;

    pub struct TestStruct{
        pub value : i32,
        pub value2 : String,
    }

    impl TestStruct {
        pub fn new(value:i32, value2:String) -> Self{
            Self{value, value2}
        }

        pub fn encode(&self) -> Vec<u8>{
            self.value.to_be_bytes()
                .iter()
                .chain(self.value2.len().to_be_bytes().iter())
                .chain(self.value2.as_bytes().iter())
                .map(|v| *v).collect()
        }

        pub fn decode(data: &[u8]) -> Self{
            let value = i32::from_be_bytes((&data[0..4]).try_into().expect("wrong size"));
            let size = usize::from_be_bytes((&data[4..12]).try_into().expect("wrong size"));
            let value2 = String::from_utf8_lossy(&data[12..12+size]).to_string();

            Self::new(value, value2)
        }
    }
}


#[cfg(test)]
pub mod tests {
    use crate::test::TestStruct;

    #[test]
    fn test_teststruct() {
        let test = TestStruct::new(1999, String::from("fdpcacacaca"));

        let buf = test.encode();

        let test2 = TestStruct::decode(buf.as_slice());

        println!("{} {}", test2.value, test2.value2);
    }
}