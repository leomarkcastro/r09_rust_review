mod mod1;

#[allow(dead_code)]

mod tests {

    use crate::mod1;
    use std::mem;

    pub fn text_formatters() {
        println!(
            "{num} => 0x{numhex}",
            num = mod1::format_string::with_zeroes(4235, 5),
            numhex = mod1::format_string::to_hex(4235)
        );
    }

    pub fn struct_printing() {
        let person = mod1::struct_data::Person {
            name: "Leo Mark Castro",
            age: 22,
        };
        println!("{:#?}", person);
        println!("{}", person);
    }

    pub fn primitive_testings() {
        println!("{}", 1i32 - 5)
    }

    pub fn tuple_playing() {
        let tuple = ("Leo", 2, 3u32, 0x465, 0b101010);
        let (a, b, c, d, e) = tuple;
        println!("{} {} {} {} {}", a, b, c, d, e);
    }

    pub fn array_testings() {
        fn array_taking_fx(arr: &[i64]) {
            // `len` returns the count of elements in the array
            println!("number of elements in array: {}", arr.len());

            // Arrays are stack allocated
            println!("array occupies {} bytes", mem::size_of_val(arr));
        }

        let _100_0s: [i64; 100] = [0; 100];

        // println!("{:?}", _100_0s);

        array_taking_fx(&_100_0s);
    }

    pub fn enum_tests() {
        enum Food {
            Apple = 1,
            Sandwich = 2,
            Pizza = 3,
        }

        enum ActionEvents {
            Walk { x: i64, y: i64 },
            Idle,
            Sleep(u32),
            Speak(String),
            Eat(u32),
        }

        // So we dont have to type ["ActionEvents::"] over and over again
        use ActionEvents::*;

        let action_list = [
            Idle,
            Walk { x: 5, y: 8 },
            Speak("Hello There!".to_string()),
            Eat(Food::Apple as u32),
        ];

        for act in action_list.iter() {
            match act {
                Idle => println!("You are idle"),
                Walk { x, y } => {
                    println!("You walked {} horizontally and {} vertically", x, y)
                }
                Sleep(h) => println!("You slept for {} hours", h),
                Speak(message) => println!("You said: \"{}\"", message),
                Eat(food) => println!("You ate food with id {}", food),
            }
        }
    }

    pub fn mutability_testings() {
        let mut mutable_var = 0;
        let immutable_var = 1;

        println!("v1 == v2 ? {}", mutable_var == immutable_var);

        mutable_var += 1;
        println!("v1 == v2 ? {}", mutable_var == immutable_var);
    }

    pub fn var_init_testings() {
        // Declare a variable binding
        let a_binding;

        {
            let x = 2;

            // Initialize the binding
            a_binding = x * x;
        }

        println!("a binding: {}", a_binding);

        let another_binding;

        // Error! Use of uninitialized binding
        // println!("another binding: {}", another_binding);
        // FIXME ^ Comment out this line

        another_binding = 1;

        println!("another binding: {}", another_binding);
    }

    pub fn var_conversions() {
        let val_in_float = 64.541592_f32;

        let val_in_int = val_in_float as u8;
        let val_in_char = val_in_int as char;

        println!("{} => {} => {}", val_in_float, val_in_int, val_in_char);

        let val2_in_i16 = 1000_u16;
        let val2_in_i8 = val2_in_i16 as u8;

        println!("{} => {}", val2_in_i16, val2_in_i8);
    }

    pub fn from_and_intos_1() {
        let my_str = "hello";
        let my_string = String::from(my_str);

        println!("{} => {}", my_str, my_string);
    }

    pub fn from_and_intos_2() {
        use mod1::struct_data::Number;

        let number = 5;
        let strct_num_from = Number::from(number);
        let strct_num_into: Number = number.into();

        println!("{} => {} => {}", number, strct_num_from, strct_num_into);
    }

    pub fn from_and_into_try_1() {
        let n1 = 2;
        let n2 = 3;

        use mod1::struct_data::EvenNumber;

        let n1_en: Result<EvenNumber, _> = n1.try_into();
        match n1_en {
            Ok(n) => println!("n1 is an even number: {}", n),
            Err(_) => println!("n1 is odd"),
        }

        let n2_en: Result<EvenNumber, _> = n2.try_into();
        match n2_en {
            Ok(n) => println!("n2 is an even number: {}", n),
            Err(_) => println!("n2 is odd"),
        }
    }

    pub fn declare_value_using_headless_scope() {
        let x = 2;
        let y = {
            let _v1 = x * x;
            let _v2 = _v1 * x;

            _v2 * _v1
        };

        println!("{} {}", x, y);
    }

    pub fn nested_loop() {
        let mut l1 = 0;
        'l1: loop {
            l1 += 1;
            if l1 == 10000 {
                break 'l1;
            }
            let mut l2 = 0;
            'l2: loop {
                l2 += 1;
                if l2 == 100000 {
                    break 'l2;
                }

                let l3 = l1 * l2;

                // println!("{} x {} = {}", l1, l2, l1 * l2);
            }
        }
        println!("Loop Done!")
    }

    pub fn match_binding() {
        fn age() -> u32 {
            15
        }
        match age() {
            0 => println!("I haven't celebrated my first birthday yet"),
            // Could `match` 1 ..= 12 directly but then what age
            // would the child be? Instead, bind to `n` for the
            // sequence of 1 ..= 12. Now the age can be reported.
            n @ 1..=12 => println!("I'm a child of age {:?}", n),
            n @ 13..=19 => println!("I'm a teen of age {:?}", n),
            // Nothing bound. Return the result.
            n => println!("I'm an old person of age {:?}", n),
        }
    }
}

#[allow(dead_code)]

mod fizzbuzz {
    use crate::mod1;

    pub fn via_try_into() {
        use mod1::struct_data::FizzBuzz;

        for n in 1..100 + 1 {
            let n_fb: Result<FizzBuzz, _> = n.try_into();
            match n_fb {
                Ok(n) => println!("{}", n),
                Err(_) => {}
            }
        }
    }

    pub fn via_match() {
        for n in 1..100 + 1 {
            match n {
                n if n % 15 == 0 => println!("FizzBuzz"),
                n if n % 3 == 0 => println!("Fizz"),
                n if n % 5 == 0 => println!("Buzz"),
                _ => println!("{}", n.to_string()),
            }
        }
    }
}

#[allow(dead_code)]

mod calculator {

    use std::io;

    pub fn get_input_from_console() -> String {
        let mut user_input = String::new();
        let stdin = io::stdin();
        let result = stdin.read_line(&mut user_input);

        match result {
            Ok(_t) => user_input.clone().trim().to_string(),
            Err(_e) => "0".to_string(),
        }
    }

    pub fn test_addition() {
        let a = get_input_from_console();
        let b = get_input_from_console();

        let a_n = a.parse::<i32>();
        let b_n = b.parse::<i32>();

        let mut a_x = 0;
        let mut b_x = 0;

        match a_n {
            Ok(a) => a_x = a,
            Err(_) => println!("Error input for var A"),
        }
        match b_n {
            Ok(b) => b_x = b,
            Err(_) => println!("Error input for var B"),
        }

        println!("{a} + {b} = {ans}", a = a, b = b, ans = a_x + b_x);
    }
}

fn main() {
    tests::match_binding();
}
