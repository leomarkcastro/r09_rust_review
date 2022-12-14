use core::fmt;
use std::convert::From;
use std::convert::TryFrom;

#[derive(Debug)]
pub struct Person<'a> {
    pub name: &'a str,
    pub age: u8,
}

impl fmt::Display for Person<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Name: {name} || Age: {age}",
            name = self.name,
            age = self.age
        )
    }
}

#[derive(Debug)]
pub struct Number {
    pub value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Struct_Number: [{}]", self.value)
    }
}

#[derive(Debug, PartialEq)]
pub struct EvenNumber {
    pub value: i32,
}

impl TryFrom<i32> for EvenNumber {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(EvenNumber { value })
        } else {
            Err(())
        }
    }
}

impl fmt::Display for EvenNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Struct_EvenNumber: [{}]", self.value)
    }
}

#[derive(Debug, PartialEq)]
pub struct FizzBuzz {
    pub num: i32,
    pub data: String,
}

impl TryFrom<i32> for FizzBuzz {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value < 0 {
            return Err(());
        }
        let mut msg = String::new();
        if value % 3 == 0 {
            msg += "Fizz"
        }
        if value % 5 == 0 {
            msg += "Buzz"
        }
        if msg.len() == 0 {
            msg += &value.to_string()
        }
        Ok(FizzBuzz {
            num: value,
            data: msg,
        })
    }
}

impl fmt::Display for FizzBuzz {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "FizzBuzz: [{} -> {}]", self.num, self.data)
    }
}
