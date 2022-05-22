#![allow(unused)]
#![allow(dead_code)]
pub mod chapter11;
//   ____      _   _   _               ____  _             _           _
//  / ___| ___| |_| |_(_)_ __   __ _  / ___|| |_ __ _ _ __| |_ ___  __| |
// | |  _ / _ \ __| __| | '_ \ / _` | \___ \| __/ _` | '__| __/ _ \/ _` |
// | |_| |  __/ |_| |_| | | | | (_| |  ___) | || (_| | |  | ||  __/ (_| |
//  \____|\___|\__|\__|_|_| |_|\__, | |____/ \__\__,_|_|   \__\___|\__,_|
//                            |___/
mod ch01 {
    //Installation
    /*
        rustup:
            rustup update - update rust enviroment
            rustup self uninstall - uninstall rustup and rust enviroment
    */

    //Hello world
    /*
        rustc:
            Compile:
                rustc <file name> - compile file
    */
    #[test]
    fn hello_world() {
        println!("HelloWorld");
    }

    //Hello Cargo
    /*
        cargo:
            Create project:
                cargo new <project name> - create new project directory
                cargo init - create new project in current directory
            Build/Run:
                cargo build - build binary
                cargo run - build and run binary
                cargo check - check if program compiles but don't build binary
                cargo build --release - compile with optimizations
            Manage:
                cargo update - update dependencies, ignoring cargo.lock file
                cargo doc --open - build documentation for all your dependencies and open in browser
    */
}
//   ____                     _                ____
//  / ___|_   _  ___  ___ ___(_)_ __   __ _   / ___| __ _ _ __ ___   ___
// | |  _| | | |/ _ \/ __/ __| | '_ \ / _` | | |  _ / _` | '_ ` _ \ / _ \
// | |_| | |_| |  __/\__ \__ \ | | | | (_| | | |_| | (_| | | | | | |  __/
//  \____|\__,_|\___||___/___/_|_| |_|\__, |  \____|\__,_|_| |_| |_|\___|
//                                   |___/
mod ch02 {
    use std::cmp::Ordering;
    //use std::io;
    use rand::Rng;
    #[test]
    fn guessing_game() {
        //generate 'secret' number between 1 and 100
        let secret_number: u32 = rand::thread_rng().gen_range(1..101);

        loop {
            //for reading into stdin
            //let mut guess = String::new();

            //read input into stdin and panic if read fails
            //io::stdin().read_line(&mut guess).expect("Failed to read line");

            //to work for test----------------
            let guess = secret_number.to_string();
            //--------------------------------

            //trim and parse the input guess into u32 and match on it if it is a valid u32 number
            //otherwise jump to the top of the loop again
            let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

            match guess.cmp(&secret_number) {
                //use the cmp themod to compare guess against the secret number
                //do things depending on whether it is Less than, greater than or equal to value
                Ordering::Less => assert!(false),
                Ordering::Greater => assert!(false),
                Ordering::Equal => {
                    break;
                }
            }
        }
        //if we broke out of the loop and end up here the test worked
        assert!(true);
    }
}
//   ____                                         ____                           _
//  / ___|___  _ __ ___  _ __ ___   ___  _ __    / ___|___  _ __   ___ ___ _ __ | |_ ___
// | |   / _ \| '_ ` _ \| '_ ` _ \ / _ \| '_ \  | |   / _ \| '_ \ / __/ _ \ '_ \| __/ __|
// | |__| (_) | | | | | | | | | | | (_) | | | | | |__| (_) | | | | (_|  __/ |_) | |_\__ \
//  \____\___/|_| |_| |_|_| |_| |_|\___/|_| |_|  \____\___/|_| |_|\___\___| .__/ \__|___/
//                                                                       |_|
mod ch03 {
    #[test]
    fn shadowing() {
        //Shadowed variable equals 6, in next scope equals 7, previous shadowed variable goes out of scope and equals 6 again
        let x = 5;
        let x = x + 1;
        {
            let x = x + 1;
            assert_eq!(x, 7);
        }
        assert_eq!(x, 6);
    }
    #[test]
    fn shadowing2() {
        //Shadowed variable is used to declare different type
        let x = 5;
        assert_eq!(x, 5);
        let x = "five";
        assert_eq!(x, "five");
    }
    #[test]
    fn overflow() {
        //use wrapping_* methods when wrapping is intended behavior
        let x: u8 = 255;
        let x = x.wrapping_add(6);
        assert_eq!(x, 5);
        //use checked_* methods to return a Option<T> that returns None if overflow occurs
        let x: u8 = 255;
        let x: Option<u8> = x.checked_add(6);
        assert_eq!(x, None);
        //use overflowing_* methods to return the value with a boolean if an overflow occured
        let x: u8 = 255;
        let x = x.overflowing_add(6);
        assert_eq!(x, (5, true));
        //use saturate_* methods to stop at a values minimum or maximum values rather than overflowing
        let x: u8 = 250;
        let x = x.saturating_add(200);
        assert_eq!(x, 255);
    }
    #[test]
    fn chars() {
        //chars are UTF-8
        let ch = '😊';
        assert_eq!(ch, '😊');
    }
    #[test]
    fn tuples() {
        //Tuples are declared with a comma seperated list in parethesis and cannot grow or shrink
        //Values can be indexed
        let mut x: (u8, f32, &str) = (1, 32.2, "word");
        x.0 = 2;
        x.1 = x.1 + 1.0;
        x.2 = "different";
        assert_eq!(x, (2, 33.2, "different"));
        //Tuples can be destructured
        let (a, b, c) = x;
        assert_eq!(a, x.0);
        assert_eq!(b, x.1);
        assert_eq!(c, x.2);
    }
    #[test]
    fn arrays() {
        //Arrays are fixed length
        let _a: [u8; 3] = [1, 2, 3]; //Length can be specified after type followed by semicolon
        let a = [3; 2]; //You can initialize an array with the same value for each element by specifiying the value followed by length
        assert_eq!(a, [3, 3]);
    }
    #[test]
    fn expressions() {
        //scopes act as expressions so
        let y = {
            let x = 3;
            x + 5
        };
        assert_eq!(y, 8);
        //and ifs act as expressions so
        let a = if y != 8 { 6 } else { 7 };
        assert_eq!(a, 7)
        //most things are expressions and return a value
        //but assignments do not act as expressions so
        /*
        let x = (let y = 1); and
        let x = y = 1;
        */
        //is invalid
    }
    #[test]
    fn loops() {
        //the "loop" loop
        let mut x = 3;
        //doesn't break till told explicitly
        loop {
            if x == 0 {
                break;
            }
            x = x - 1;
        }
        assert_eq!(x, 0);
        x = 3;
        //use continue to skip to next execution of loop
        loop {
            if x != 0 {
                x = x - 1;
                continue;
            }
            break;
        }
        assert_eq!(x, 0);
        //break to specific loop with labeled loops
        'outer_loop: loop {
            loop {
                break 'outer_loop; //breaks out of outer labeled loop
            }
        }
        assert!(true);
        //return value from loop using break statement
        let a = loop {
            x += 1;
            if x == 10 {
                break x + 12; //this value is returned from loop expression
            }
        };
        assert_eq!(a, 22);
    }
    #[test]
    fn for_loop() {
        let a = [10, 20, 30, 40, 50];
        //the for loop
        for element in a {
            match element {
                10 => assert!(true),
                20 => assert!(true),
                30 => assert!(true),
                40 => assert!(true),
                50 => assert!(true),
                _ => assert!(false),
            }
        }
        //for range loop
        for number in (1..4).rev() {
            assert!(0 < number && number < 4);
        }
    }
}
//  _   _           _               _                  _ _                ___                               _     _
// | | | |_ __   __| | ___ _ __ ___| |_ __ _ _ __   __| (_)_ __   __ _   / _ \__      ___ __   ___ _ __ ___| |__ (_)_ __
// | | | | '_ \ / _` |/ _ \ '__/ __| __/ _` | '_ \ / _` | | '_ \ / _` | | | | \ \ /\ / / '_ \ / _ \ '__/ __| '_ \| | '_ \
// | |_| | | | | (_| |  __/ |  \__ \ || (_| | | | | (_| | | | | | (_| | | |_| |\ V  V /| | | |  __/ |  \__ \ | | | | |_) |
//  \___/|_| |_|\__,_|\___|_|  |___/\__\__,_|_| |_|\__,_|_|_| |_|\__, |  \___/  \_/\_/ |_| |_|\___|_|  |___/_| |_|_| .__/
//                                                               |___/                                             |_|
mod ch04 {
    //Ownership:
    /*
        Each value in Rust has a variable that’s called its owner.
        There can only be one owner at a time.
        When the owner goes out of scope, the value will be dropped.
    */
    #[test]
    fn movements() {
        //with non-heap allocated primitives moves have expected behavior
        let x = 5;
        let y = x;
        assert_eq!(x, 5);
        assert_eq!(y, 5);
        //with heap allocated values, moves pass ownership and invalidate the variable that was passed from
        let s1 = String::from("word");
        let s2 = s1;
        //s1 is now invalid with s2 now owning s1
        assert_eq!(s2, String::from("word"));
        //to create deep copies of heap data, use clone methods
        let s1 = String::from("another word");
        //the clone() method creates a deep copy of object, so s1 remains valid
        let s2 = s1.clone();
        assert_eq!(s1, s2);

        //passing to functions also moves heap allocated variables and takes ownership
        let s = String::from("is alive?");
        takes_ownership(s); //string s is taken here
                            //and is dropped after function

        //function returns also pass ownership
        let s = gives_ownership();
        assert_eq!(s, "Here you go".to_owned());

        let x = String::from("a thing");
        //so functions can both take and give ownership
        //here the x string is taken by the function and then moves the value into s
        let s = gives_and_takes(x);
        assert_eq!(s, "a thing".to_owned());

        //function takes ownership and eats variable
        fn takes_ownership(my_str: String) {
            println!("{}", my_str);
        }
        //function creates value and passes ownership
        fn gives_ownership() -> String {
            String::from("Here you go")
        }
        //function takes ownership of variable and passes is back out
        fn gives_and_takes(my_str: String) -> String {
            my_str
        }
    }
    //References:
    /*
        Variables can have many references or
        one mutable reference at a time

        references must always be valid

        let mut s = String::from("hello");

        let r1 = &mut s;
        let r2 = &mut s; <---This fails because second mutable borrow occurs of the same variable
    */
    #[test]
    fn borrowing() {
        //manage multiple mutable references with scope
        let mut s = String::from("hello");
        {
            let r1 = &mut s;
            assert_eq!(r1, "hello");
        }
        //r1 goes out of scope here so a new mutable reference to s can be made
        let r2 = &mut s;
        assert_eq!(r2, "hello");
        //you can have multiple immutable references
        {
            let r3 = &s;
            let r4 = &s;
            // let r5 = &mut s; <---can't make a mutable borrow on value that is already referenced in this scope
            // assert_eq!(r4, r5); <---immutable values must be able to assume their value won't change
            //but you can make a mutable ref after all other refs are used
            assert_eq!(r3, r4); //r3 and r4 are not used after this line, so the mutable borrow and usage is valid on the following lines
            let r5 = &mut s;
            assert_eq!(r5, "hello");
        }
    }
    //Dangling References:
    /*
        returning a reference from a function will result in a dangling reference when not using lifetimes
        ex.
            fn dangle() -> &String {
                let s = String::from("thing");
                s <--- s is returned but s goes out of scope at the end of this function so this wont compile
            }

        return value directly to move ownership out
            fn no_dangle() -> String {
                let s = String::from("thing");
                s <--- s is moved out without deallocation, this compiles
            }
    */
    #[test]
    fn slices() {
        //use range notation to grab reference slices of a String
        let s = String::from("hello, world");

        let hello = &s[..5]; //if starting from index zero you can ommit the zero
        let world = &s[7..12]; // if slice includes last byte you can ommit the end of the range too: "[7..]"
        let hello_world = &s[..]; //if taking whole string, you can ommit the first and last index as well
        assert_eq!(hello, "hello");
        assert_eq!(world, "world");
        assert_eq!(hello_world, "hello, world");

        //return first word using slice
        let hello_comma = first_word(&s);
        assert_eq!(hello_comma, "hello,");

        //function for returning slice of string above
        fn first_word(s: &str) -> &str {
            let bytes = s.as_bytes(); //read as bytes (assumming ascii characters here)

            //create for loop that iterates over collection using iter() and wraps iter in tuple of (index, value) using enumerate()
            for (i, &item) in bytes.iter().enumerate() {
                if item == b' ' {
                    //using byte literal syntax, check if item is equal to space and return slice to current index if it does
                    return &s[0..i];
                }
            }
            //if no space is found return the whole string
            &s[..]
        }
        //slices of other types
        let a = [1, 2, 3, 4, 5];
        let slice = &a[1..3];
        assert_eq!(slice, &[2, 3]);
    }
}
//  _   _     _               ____  _                   _
// | | | |___(_)_ __   __ _  / ___|| |_ _ __ _   _  ___| |_ ___
// | | | / __| | '_ \ / _` | \___ \| __| '__| | | |/ __| __/ __|
// | |_| \__ \ | | | | (_| |  ___) | |_| |  | |_| | (__| |_\__ \
//  \___/|___/_|_| |_|\__, | |____/ \__|_|   \__,_|\___|\__|___/
//                    |___/
mod ch05 {
    #[test]
    fn structs() {
        //Define a struct
        struct Example {
            flag: bool,
            name: String,
        }
        //create instance, if you want to mutate members the entire struct needs to be mutable
        //rust prohibits marking specific fields as mutable
        let mut thing = Example {
            flag: false,
            name: String::from("my name"),
        };
        thing.flag = true;
        thing.name = String::from("new name");
        assert_eq!(thing.flag, true);
        assert_eq!(thing.name, String::from("new name"));
        //return struct from implicit return
        let new_thing = make_example("danny".to_string());
        assert_eq!(new_thing.flag, false);
        assert_eq!(new_thing.name, String::from("danny"));

        fn make_example(name: String) -> Example {
            //The example struct gets implicitly returned here
            Example {
                flag: false,
                name, //using the field init shorthand you can pass the parameter "name" straight to the member of the same name like this
            }
        }
    }
    #[test]
    fn structs2() {
        struct Update {
            val: i32,
            val2: i32,
            val3: i32,
        }
        //create first struct object
        let first = Update {
            val: 1,
            val2: 2,
            val3: 3,
        };
        //using struct update syntax, initialize unique values and then fill the rest of the values in from previous struct using ..<object name>
        let second = Update { val2: 7, ..first };
        assert_eq!(second.val, 1);
        assert_eq!(second.val2, 7);
        assert_eq!(second.val3, 3);
    }
    #[test]
    fn structs3() {
        //tuple structs allow you to name tuples and differentiate between different data structures that may have similar types
        struct Color(i32, i32, i32);
        let black = Color(0, 0, 0);
        //tuple structs behave like tuples, you can destructure and access values with dot notation
        //they just have the benefits of being named as well
        assert_eq!(black.0, 0);

        //unit-like structs
        //used for implementing traits on types you don't want to store data in
        struct AlwaysEqual;
        let _thing = AlwaysEqual;
    }
    #[test]
    fn struct_example_program() {
        //use derive debug to give struct the debug trait
        #[derive(Debug)]
        struct Rectangle {
            width: u32,
            height: u32,
        }

        let rect1 = Rectangle {
            width: 30,
            height: 50,
        };

        //with the debug trait you can use {:?} to print struct that doesn't implement Display trait
        println!("rect1 is {:?}", rect1); //will print "rect1 is Rectangle { width: 30, height: 50 }"
                                          //use {#:?} for pretty formatting of debug printing
                                          //you can also use the dbg! macro  for printing values to error console
                                          //WARNING: be careful with passing ownership with dbg! macro
                                          //dbg! takes ownership of value then passes it along

        assert_eq!(rect1.width, 30);
        assert_eq!(rect1.height, 50);
    }
    #[test]
    fn method_syntax() {
        struct Rectangle {
            width: u32,
            height: u32,
        }

        impl Rectangle {
            //associated function that doesn't have self as first parameter, access with :: syntax (i.e. Rectangle::new(val1, val2))
            fn new(width: u32, height: u32) -> Rectangle {
                Rectangle { width, height }
            }
            //method that immutably borrows self (taking ownership with just (self) is rare)
            fn area(&self) -> u32 {
                self.width * self.height
            }
            //method that mutably borrows from self
            fn double_width(&mut self) {
                self.width = self.width * 2;
            }
            //immutable self method with additional parameters
            fn can_hold(&self, rect: &Rectangle) -> bool {
                self.area() > rect.area()
            }
        }

        let mut rect = Rectangle {
            width: 5,
            height: 10,
        };

        let area = rect.area();
        assert_eq!(area, 50);
        rect.double_width();
        let area = rect.area();
        assert_eq!(area, 100);

        let rect2 = Rectangle::new(2, 4);

        assert_eq!(rect.can_hold(&rect2), true);
        assert_eq!(rect2.can_hold(&rect), false);
    }
}
//  _____                                             _   ____       _   _                    __  __       _       _     _
// | ____|_ __  _   _ _ __ ___  ___    __ _ _ __   __| | |  _ \ __ _| |_| |_ ___ _ __ _ __   |  \/  | __ _| |_ ___| |__ (_)_ __   __ _
// |  _| | '_ \| | | | '_ ` _ \/ __|  / _` | '_ \ / _` | | |_) / _` | __| __/ _ \ '__| '_ \  | |\/| |/ _` | __/ __| '_ \| | '_ \ / _` |
// | |___| | | | |_| | | | | | \__ \ | (_| | | | | (_| | |  __/ (_| | |_| ||  __/ |  | | | | | |  | | (_| | || (__| | | | | | | | (_| |
// |_____|_| |_|\__,_|_| |_| |_|___/  \__,_|_| |_|\__,_| |_|   \__,_|\__|\__\___|_|  |_| |_| |_|  |_|\__,_|\__\___|_| |_|_|_| |_|\__, |
//                                                                                                                               |___/
mod ch06 {
    #[test]
    fn enums() {
        //define an enum with two variants (V4 & V6)
        //and pack V4 with 4 u8 types and V6 with String type
        #[derive(Debug)]
        enum IPAddrKind {
            V4(u8, u8, u8, u8),
            V6(String),
        }
        //create instances
        let four = IPAddrKind::V4(127, 0, 0, 1);
        let six = IPAddrKind::V6(String::from("::1"));
        //define function that takes IPAddrKind
        fn route(ip_kind: &IPAddrKind) {
            println!("{:?}", ip_kind)
        }
        //and call function with either variant
        route(&four);
        route(&six);
        //pattern match to extract value from enum
        match six {
            IPAddrKind::V6(addr) => {
                if addr == String::from("::1") {
                    assert!(true)
                } else {
                    assert!(false)
                }
            }
            _ => assert!(false),
        }
    }
    #[test]
    fn option() {
        //define Option<i32> var with some value of 5
        let x = Some(5);
        //define Option<String> with no value
        let y: Option<String> = None;

        //match on x, if it has a value of 5, assert true, else assert false in all other cases
        match x {
            Some(val) => {
                if val == 5 {
                    assert!(true)
                } else {
                    assert!(false)
                }
            }
            None => assert!(false),
        }

        //match on y, if it contains some value assert false, if it contains no value assert true
        match y {
            Some(_) => assert!(false),
            None => assert!(true),
        }
    }
    #[test]
    fn matching() {
        enum UsState {
            Alabama,
            //etc
        }
        enum Coin {
            Nickel,
            Quarter(UsState),
        }

        fn value_in_cents(coin: Coin) -> u8 {
            //match on coin and return value based on enum variant
            match coin {
                Coin::Nickel => 5,
                Coin::Quarter(state) => match state {
                    UsState::Alabama => 100,
                },
            }
        }
        let coin = Coin::Nickel;
        assert_eq!(value_in_cents(coin), 5);
        let coin = Coin::Quarter(UsState::Alabama);
        assert_eq!(value_in_cents(coin), 100);
    }
    #[test]
    fn matching_option() {
        //match on option parameter, modify and return an option
        fn plus_one(x: Option<i32>) -> Option<i32> {
            match x {
                None => None,
                Some(i) => Some(i + 1),
            }
        }

        let five = Some(5);
        let six = plus_one(five);
        let none = plus_one(None);

        assert!(six.is_some());
        assert!(none.is_none());
    }
    #[test]
    fn catch_all_matches() {
        let dice_roll = 9;
        //matches must be exhaustive, if you omit a possible match value the code wont compile
        //create variable to bind all other possible values and catch all possible matches
        match dice_roll {
            3 => assert!(false),
            7 => assert!(false),
            other => assert_eq!(other, 9),
        }
        //use the _ placeholder to catch and discard value
        match dice_roll {
            1 => assert!(false),
            2 => (), //use unit value '()' to tell rust we don't want to do anything
            _ => assert!(true),
        }
    }
    #[test]
    fn if_let() {
        enum Things {
            Thing1,
            Thing2(u8),
        }

        let my_thing = Things::Thing2(12);
        let my_other_thing = Things::Thing1;

        //the if let expression
        //shorter sytax for checking a match against a single type
        if let Things::Thing2(value) = my_thing {
            assert_eq!(value, 12);
        }
        //can be chained like normal if statements using 'else if let', 'else', etc
        if let Things::Thing1 = my_other_thing {
            assert!(true);
        } else {
            assert!(false);
        }
    }
}
//  __  __           _       _
// |  \/  | ___   __| |_   _| | ___  ___
// | |\/| |/ _ \ / _` | | | | |/ _ \/ __|
// | |  | | (_) | (_| | |_| | |  __/\__ \
// |_|  |_|\___/ \__,_|\__,_|_|\___||___/
//bring in external mod
mod chapter07;
mod ch07 {
    //chapter 7 modules
    mod outer_module {
        pub mod inner_module1 {
            pub fn ret_true() -> bool {
                true
            }
            pub fn self_ret_true() -> bool {
                true
            }
        }
        pub mod inner_module2 {
            pub fn ret_true() -> bool {
                true
            }
        }
        //a public struct will still have all private members unless declared explicitly public
        pub struct MyStruct {
            pub value: i32,
            private_string: String,
        }
        impl MyStruct {
            pub fn new(value: i32, private_string: String) -> MyStruct {
                MyStruct {
                    value,
                    private_string,
                }
            }
            pub fn get_str(&self) -> String {
                self.private_string.clone()
            }
        }
        //a public enums values will all be public by default
        pub enum MyEnum {
            Thing,
        }
    }
    //access outer/inner_module modules by using crate:: to get current crate root
    //Absolute path
    use crate::ch07::outer_module::inner_module1::ret_true;
    //Relative path and using the 'as' keyword to create an alias for the type
    use outer_module::inner_module2::ret_true as ret_true_alt;
    //bring in external module
    use crate::chapter07::external_module;
    //using self and re-exporting inner_module1 with pub mod
    pub use self::outer_module::inner_module1;

    //note
    /*
        the idiomatic style is to pull in the parent scope of functions
            so you would use ParentName::FunctionName() rather than pulling FunctionName into scope and calling it directly
        this makes it clear that FunctionName isn't locally defined
        for structs/enums and other items, it is idiomatic to specify the full path and use the object direction
            i.e. use std::collections::HashMap;
            letting you do
                let mut map = HashMap::new();
            rather than
                let mut map = collections::HashMap::new();
        the exception is for when you bring in two items with the same name into scope
            like std::fmt::Result and std::io::Result
        in this case you can either bring in the parent scopes (std::fmt and std::io)
        or alias the types using the 'as' keyword
            i.e. std::io::Result as IoResult; etc.
    */

    //using re-exported inner_module1 module inside another functions scope
    pub fn if_true_ret_true() -> bool {
        if inner_module1::self_ret_true() {
            true
        } else {
            false
        }
    }

    #[test]
    fn modules() {
        assert!(ret_true());
        assert!(ret_true_alt());
        assert!(if_true_ret_true());
        //call directly
        assert!(outer_module::inner_module1::ret_true());
        //call external module function
        assert!(external_module());
        //call external module using super (equivelent to using "../" in filesystem)
        assert!(super::chapter07::external_module());
        //using structs in modules and public/private intricacies
        let test_struct = outer_module::MyStruct::new(5, String::from("test"));
        assert_eq!(test_struct.value, 5);
        assert_eq!(test_struct.get_str(), String::from("test"));
        //using enums in modules
        let test_enum = outer_module::MyEnum::Thing;
        match test_enum {
            outer_module::MyEnum::Thing => assert!(true),
        }
    }

    //final notes
    /*
        you also have the option of nesting paths so instead of
            use std::cmp::Ordering;
            use std::io;
        you could do
            use std::{cmp::Ordering, io};
        you could also merge two paths like
            use std::io;
            use std::io::Write;
        like
            use std::io::{self, Write};
        which brings both std::io and std::io::Write into scope
        finally, if you want to bring all public items into scope you can use the glob operator
            use std::collections::*;
        this will bring all publicly items defined in std::collections into scope
    */
}
//   ____                                         ____      _ _           _   _
//  / ___|___  _ __ ___  _ __ ___   ___  _ __    / ___|___ | | | ___  ___| |_(_) ___  _ __  ___
// | |   / _ \| '_ ` _ \| '_ ` _ \ / _ \| '_ \  | |   / _ \| | |/ _ \/ __| __| |/ _ \| '_ \/ __|
// | |__| (_) | | | | | | | | | | | (_) | | | | | |__| (_) | | |  __/ (__| |_| | (_) | | | \__ \
//  \____\___/|_| |_| |_|_| |_| |_|\___/|_| |_|  \____\___/|_|_|\___|\___|\__|_|\___/|_| |_|___/
mod ch08 {
    use std::collections::hash_map::Entry;
    use std::collections::HashMap;
    //use std::io;

    #[test]
    fn vector() {
        //create new vector with new() function
        //rust can't infer type, so you must be explict
        //and it must be mut otherwise you can't push to collection
        let mut x: Vec<i32> = Vec::new();
        x.push(1);
        assert_eq!(x, vec![1]);

        //create new vector with vec! macro
        //rust can infer based on what you pass, so you don't need to be explicit
        let x = vec![1, 2, 3, 4, 5];
        assert_eq!(x, vec![1, 2, 3, 4, 5]);

        //referencing values store in vec (two ways)
        let x = vec![1, 2, 3, 4, 5];
        //index reference (when you want your program to crash if accessing invalid index)
        let second = &x[2];
        assert_eq!(*second, 3);
        //get method (when you want to handle attempts to access outside the bounds of vec)
        let third = x.get(3);
        match third {
            Some(value) => assert_eq!(*value, 4),
            None => assert!(false),
        }
        //iterating over vectors
        let x = vec![1, 2, 3, 4, 5];
        let mut y = 0;
        for i in &x {
            let cmp = *i;
            assert!(cmp > y);
            y += 1;
        }
        //iterating over mutable vec
        let mut x = vec![1, 2, 3, 4, 5];
        for i in &mut x {
            *i += 5;
        }
        assert_eq!(x, vec![6, 7, 8, 9, 10]);

        //using enums to store multipe types
        enum MultiType {
            TypeOne(bool),
            TypeTwo(u8),
            TypeThree(String),
        }
        let multi_thing = vec![
            MultiType::TypeOne(true),
            MultiType::TypeTwo(8),
            MultiType::TypeThree(String::from("hi")),
        ];
        for i in &multi_thing {
            match i {
                MultiType::TypeOne(b) => assert!(b),
                MultiType::TypeTwo(val) => assert_eq!(val, &8),
                MultiType::TypeThree(str) => assert_eq!(str, &String::from("hi")),
            }
        }
    }
    #[test]
    fn strings() {
        //create &str with string literal
        let init = "init";
        //assign init to new string
        //to_string() method is availble on any type that implements Display trait
        let mut x = init.to_string();
        //push onto string with push_str
        x.push_str("ialize");
        //push char onto string with push
        x.push('s');
        assert_eq!(x, String::from("initializes"));

        //using format! macro
        let a = String::from("hello");
        let b = String::from("world");
        let c = format!("{}, {}", a, b);
        assert_eq!(c, String::from("hello, world"));

        //concatenating string with + operator
        let a = String::from("hello");
        let b = String::from(" world");
        let c = a + &b;
        assert_eq!(c, String::from("hello world"));
    }
    #[test]
    fn string_indexing() {
        //you can create string slices of the bytes in a string
        let hello = "Здравствуйте";
        let s = &hello[0..4]; //<---this grabs the first four bytes which gives us the first two characters in the string 'Зд'
        assert_eq!(s, String::from("Зд"));
        //note: if you slice in the middle of a two or more byte charater like 'З' doing &hello[0..1] the program will panic

        //use chars if you want to Unicode values
        let s = String::from("नम");
        for c in s.chars() {
            match c {
                'न' => assert!(true),
                'म' => assert!(true),
                _ => assert!(false),
            }
        }

        //use bytes if you want raw bytes
        let s = String::from("न"); //<--- as bytes this character returns [224, 164, 168]
        for c in s.bytes() {
            match c {
                224 => assert!(true),
                164 => assert!(true),
                168 => assert!(true),
                _ => assert!(false),
            }
        }
    }
    #[test]
    fn hashmaps() {
        use std::collections::HashMap;
        //creating and inserting on a new hashmap
        let mut collection = HashMap::new();
        collection.insert(String::from("key1"), 1);
        collection.insert(String::from("key2"), 2);

        //create hashmap from tuple
        let x = vec![("key1", 1), ("key2", 2)];
        let collection: HashMap<&str, i32> = x.into_iter().collect();
        let val = collection.get("key1").expect("key missing");
        assert_eq!(val, &1);

        //hashmap from seperate vectors
        let keys = vec!["key1", "key2"];
        let values = vec![11, 22];
        //create iteration of keys and zip it with a iteration of values creating tuples that are added to hashmap
        let mut collection: HashMap<_, _> = keys.into_iter().zip(values.into_iter()).collect();
        let val = collection.get("key2").expect("key missing");
        assert_eq!(val, &22);

        //iterating over hashmap
        for (key, value) in &collection {
            match key {
                &"key1" => assert_eq!(value, &11),
                &"key2" => assert_eq!(value, &22),
                _ => assert!(false),
            }
        }

        //only inserting if key has no value
        collection.entry("key1").or_insert(111); //<--- "key1" already exists so or_insert doesn't change its value
        collection.entry("key3").or_insert(333); //<--- "key3" doesn't exist so or_insert adds this value to collection
        assert_eq!(collection.get("key1").expect("key missing"), &11);
        assert_eq!(collection.get("key3").expect("key missing"), &333);

        //updating based on old value
        let text = "hello world world place world";
        let mut map = HashMap::new();

        for word in text.split_whitespace() {
            let count = map.entry(word).or_insert(0);
            *count += 1;
        }

        assert_eq!(map.get("world").expect("key missing"), &3);
    }

    //TODO: HashMaps summary problems
    //8-1
    /*
        Given a list of integers, use a vector and return the median (when sorted,
        the value in the middle position) and mode (the value that occurs most
        often; a hash map will be helpful here) of the list.
    */
    #[test]
    fn exercise8_1() {
        let mut int_list = vec![1, 2, 3, 4, 5, 1, 2, 3, 4, 5, 3];
        int_list.sort();
        let int_list = int_list;

        let median = int_list.get(int_list.len() / 2);
        match median {
            Some(num) => assert_eq!(*num, 3),
            None => assert!(false),
        }

        let mut map = HashMap::new();
        for num in int_list {
            let count = map.entry(num).or_insert(0);
            *count += 1;
        }

        let mut mode: i32 = std::i32::MIN;
        let mut count: i32 = 0;
        for val in map.iter() {
            if val.1 > &count {
                count = *val.1;
                mode = *val.0;
            }
        }
        assert_eq!(mode, 3);
    }

    //8-2
    /*
        Convert strings to pig latin. The first consonant of each word is moved to
        the end of the word and “ay” is added, so “first” becomes “irst-fay.” Words
        that start with a vowel have “hay” added to the end instead (“apple” becomes
        “apple-hay”). Keep in mind the details about UTF-8 encoding!

        vowels - a, e, i, o, u, y
    */
    #[test]
    fn exercise8_2() {
        let vowels = vec!["a", "e", "i", "o", "u", "y"];
        let mut vowel_map: HashMap<&str, &str> = HashMap::new();
        for ch in vowels {
            vowel_map.insert(ch, ch);
        }

        let sentence = String::from(
            "Using a hash map and vectors create a text interface to allow a user to add
        employee names to a department in a company",
        );

        let mut sentence_vec: Vec<String> = Vec::new();
        for word in sentence.split_ascii_whitespace() {
            sentence_vec.push(word.to_string().to_lowercase());
        }

        let mut result_vec: Vec<String> = Vec::new();
        for word in sentence_vec {
            let is_vowel = vowel_map.get(&word[0..1]);
            match is_vowel {
                Some(_) => {
                    result_vec.push(format!("{}-hay", word));
                }
                None => {
                    result_vec.push(format!("{}-{}ay", &word[1..word.len()], &word[0..1]));
                }
            }
        }

        let cmp_string = String::from("using-hay a-hay ash-hay ap-may and-hay ectors-vay reate-cay a-hay ext-tay interface-hay o-tay allow-hay a-hay user-hay o-tay add-hay employee-hay ames-nay o-tay a-hay epartment-day in-hay a-hay ompany-cay");
        let final_result: String = result_vec.join(" ");

        assert_eq!(cmp_string, final_result);
    }

    //8-3
    /*
        Using a hash map and vectors, create a text interface to allow a user to add
        employee names to a department in a company. For example, “Add Sally to
        Engineering” or “Add Amir to Sales.” Then let the user retrieve a list of all
        people in a department or all people in the company by department, sorted
        alphabetically.
    */
    #[test]
    fn exercise8_3() {
        let mut registy: HashMap<String, Vec<String>> = HashMap::new();

        'take_input: loop {
            println!("Enter employee name followed by department (enter 'end' to quit)");

            // let mut input = String::new();
            // io::stdin()
            //     .read_line(&mut input)
            //     .expect("Failed to read input");

            //test input----------------------------
            let input = String::from("end");
            //--------------------------------------

            let mut commands: Vec<String> = Vec::new();
            for word in input.split(" ") {
                if word.trim() != String::from("end") {
                    commands.push(word.trim().to_string());
                } else {
                    break 'take_input;
                }
            }
            if commands.len() != 2 {
                println!("malformed input...");
                continue;
            } else {
                let com1 = commands.get(0).expect("Failed to retrieve employee name");
                let com2 = commands.get(1).expect("Failed to retrieve department");
                match registy.entry(com2.to_string()) {
                    Entry::Vacant(e) => {
                        e.insert(vec![com1.to_string()]);
                    }
                    Entry::Occupied(mut e) => {
                        e.get_mut().push(com1.to_string());
                    }
                }

                println!("added!");
            }
        }
        println!("{:?}", registy);
        assert!(true);
    }
}
//  _____                       _   _                 _ _ _
// | ____|_ __ _ __ ___  _ __  | | | | __ _ _ __   __| | (_)_ __   __ _
// |  _| | '__| '__/ _ \| '__| | |_| |/ _` | '_ \ / _` | | | '_ \ / _` |
// | |___| |  | | | (_) | |    |  _  | (_| | | | | (_| | | | | | | (_| |
// |_____|_|  |_|  \___/|_|    |_| |_|\__,_|_| |_|\__,_|_|_|_| |_|\__, |
//                                                                |___/
mod ch09 {
    #[test]
    fn unrecoverable_errors() {
        //Notes:
        /*
            To avoid unwinding and leave the abort cleanup to the OS (thus making a smaller binary)
            add "panic = 'abort'" to your Cargo.toml file
            i.e. for aborting on panic in release mode add
                [profile.release]
                panic = 'abort'

            invoke the panic! macro to immediately halt execution
            i.e.
                panic!("panic message");

            to use rust backtrace, set the enviroment variable to anything other than 0
            i.e.
                $env:RUST_BACKTRACE=1
                cargo run
            set rust backtrace variable to full for verbose backtrace
                $env:RUST_BACKTRACE='full'
        */
        assert!(true);
    }

    use std::{
        fs::{self, File},
        io::{self, ErrorKind, Read},
    };
    #[test]
    fn recoverable_errors() {
        let f = File::open("test.txt");

        let mut f = match f {
            Ok(f) => f, //upon successfully opening file, return it
            Err(error) => match error.kind() {
                //if you get an error match on the different error types
                ErrorKind::NotFound => match File::create("test.txt") {
                    //if the error is NotFound, try creating a new file
                    Ok(fc) => fc, //upon success return the new file
                    Err(e) => panic!("Problem creating file: {:?}", e), //otherwise panic
                },
                other_error => {
                    panic!("problem opening the file: {:?}", other_error) //if the problem is some other kind of error, panic
                }
            },
        };
        //Read into s and see if the file matches the test string
        let mut s = String::new();
        f.read_to_string(&mut s).expect("failed to read");
        assert_eq!(s, String::from("Obamna"));

        //Notes:
        /*
            you can use unwrap to return the value if Result is Ok and panic if it is Err
            i.e.
                let f = File::open("hello.txt").unwrap();
            you can use expect to do the same thing, but with your own error message
                let f = File::open("hello.txt").expect("Failed to open file");
        */
    }

    #[test]
    fn propagating_errors() {
        fn read_username_from_file() -> Result<String, io::Error> {
            let f = File::open("test.txt");

            let mut f = match f {
                Ok(file) => file,
                Err(e) => return Err(e),
            };

            let mut s = String::new();

            match f.read_to_string(&mut s) {
                Ok(_) => Ok(s),
                Err(e) => Err(e),
            }
        }
        let s = read_username_from_file().unwrap();
        assert_eq!(s, "Obamna".to_string());
    }

    #[test]
    fn propagation_shortcut() {
        fn read_username_from_file() -> Result<String, io::Error> {
            //you can use the propagation operator '?' to return Err from expressions that return Result
            let mut f = File::open("test.txt")?;
            let mut s = String::new();
            f.read_to_string(&mut s)?;
            Ok(s)
        }
        fn read_username_from_file_shorter() -> Result<String, io::Error> {
            let mut s = String::new();
            //you can also chain propagation operators
            File::open("test.txt")?.read_to_string(&mut s)?;
            Ok(s)
        }
        let s = read_username_from_file().unwrap();
        assert_eq!(s, "Obamna".to_string());
        let s = read_username_from_file_shorter().unwrap();
        assert_eq!(s, "Obamna".to_string());

        //stupid short read file to string
        fn read_from_file() -> Result<String, io::Error> {
            fs::read_to_string("test.txt")
        }
        let s = read_from_file().unwrap();
        assert_eq!(s, "Obamna".to_string());

        //Note:
        /*
            The propagation operator works with functions that return Result, Option or types the implement FromResidual
            You can expect similar behavior with Option, were ? returns early with None or returns Some otherwise
        */
    }

    #[test]
    fn type_validation() {
        //create a type that has a private member and initilizes with checks for validity
        //then create a getter to access the member, thus using type to preserve needed invariant
        pub struct Guess {
            value: i32,
        }

        impl Guess {
            pub fn new(value: i32) -> Guess {
                if value < 1 || value > 100 {
                    panic!("Guess value must be between 1 and 100, got {}.", value);
                }

                Guess { value }
            }

            pub fn value(&self) -> i32 {
                self.value
            }
        }
        let x = Guess::new(8);
        assert_eq!(x.value(), 8);
    }
}
//   ____                      _              _____          _ _           _     _  __      _   _
//  / ___| ___ _ __   ___ _ __(_) ___ ___    |_   _| __ __ _(_) |_ ___    | |   (_)/ _| ___| |_(_)_ __ ___   ___  ___
// | |  _ / _ \ '_ \ / _ \ '__| |/ __/ __|     | || '__/ _` | | __/ __|   | |   | | |_ / _ \ __| | '_ ` _ \ / _ \/ __|
// | |_| |  __/ | | |  __/ |  | | (__\__ \_    | || | | (_| | | |_\__ \_  | |___| |  _|  __/ |_| | | | | | |  __/\__ \_
//  \____|\___|_| |_|\___|_|  |_|\___|___(_)   |_||_|  \__,_|_|\__|___(_) |_____|_|_|  \___|\__|_|_| |_| |_|\___||___(_)
mod ch10 {
    #[test]
    fn generic_function_definition() {
        fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
            let mut largest = list[0];

            for &item in list {
                if item > largest {
                    largest = item;
                }
            }
            largest
        }

        let number_list = vec![1, 2, 3, 4, 5, 22, 1, 4];
        let result = largest(&number_list);
        assert_eq!(result, 22);

        let number_list = vec!['a', 'c', 'z'];
        let result = largest(&number_list);
        assert_eq!(result, 'z');
    }
    #[test]
    fn generic_struct_definition() {
        struct Point<T> {
            x: T,
            y: T,
        }

        let integer = Point { x: 5, y: 5 };
        let float = Point { x: 1.2, y: 2.2 };
        assert_eq!(integer.x + integer.y, 10);
        assert_eq!((float.x + float.y) as i32, 3);
    }
    #[test]
    fn generic_enum_definition() {
        enum Enumerate<T> {
            Thing(T),
            NoThing,
        }

        let my_enum = Enumerate::Thing('z');
        assert_eq!(
            match my_enum {
                Enumerate::Thing(x) => x,
                Enumerate::NoThing => 'f',
            },
            'z'
        );

        let my_enum: Enumerate<i32> = Enumerate::NoThing;
        assert_eq!(
            match my_enum {
                Enumerate::Thing(_) => 12,
                Enumerate::NoThing => 13,
            },
            13
        );
    }
    #[test]
    fn generic_method_definition() {
        struct Point<T> {
            x: T,
            y: T,
        }

        impl<T> Point<T> {
            fn x(&self) -> &T {
                &self.x
            }
        }
        //you can also specify an implementation on a concrete type
        impl Point<f32> {
            fn xf(&self) -> &f32 {
                &self.x
            }
        }

        let q = Point { x: 1, y: 2 };
        assert_eq!(q.x(), &1);
        assert_eq!(q.y, 2);

        let q: Point<f32> = Point { x: 1.1, y: 2.2 };
        assert_eq!(*q.xf() as i32, 1);

        //multiple generic parameters
        struct WeirdPoint<X1, Y1> {
            x: X1,
            y: Y1,
        }

        impl<X1, Y1> WeirdPoint<X1, Y1> {
            fn mixup<X2, Y2>(self, other: WeirdPoint<X2, Y2>) -> WeirdPoint<X1, Y2> {
                WeirdPoint {
                    x: self.x,
                    y: other.y,
                }
            }
        }

        let p1 = WeirdPoint { x: 5, y: 10.4 };
        let p2 = WeirdPoint { x: "Hello", y: 'c' };
        let p3 = p1.mixup(p2);

        assert_eq!(p3.x, 5);
        assert_eq!(p3.y, 'c');
    }
    #[test]
    fn defining_traits() {
        pub trait Summary {
            fn summarize(&self) -> String;
        }
        struct Comment {
            msg: String,
            user: String,
        }
        //syntax for implementing trait on type
        impl Summary for Comment {
            fn summarize(&self) -> String {
                format!("{}: {}", self.user, self.msg)
            }
        }
        struct Form {
            name: String,
            id: i32,
        }
        impl Summary for Form {
            fn summarize(&self) -> String {
                format!("Name: {}\nID: {}\n", self.name, self.id)
            }
        }

        let comment = Comment {
            msg: String::from("Hello, world!"),
            user: String::from("Obamna"),
        };
        let form = Form {
            name: String::from("Obamna"),
            id: 69420,
        };

        assert_eq!(comment.summarize(), String::from("Obamna: Hello, world!"));
        assert_eq!(form.summarize(), String::from("Name: Obamna\nID: 69420\n"));
    }
    #[test]
    fn default_trait_implementations() {
        pub trait Summary {
            fn summarize(&self) -> String {
                String::from("Default impl")
            }
        }
        struct Thing {
            thing: i32,
        }
        impl Summary for Thing {}

        let t = Thing { thing: 2 };
        assert_eq!(t.thing, 2);
        assert_eq!(t.summarize(), String::from("Default impl"));
    }
    #[test]
    fn traits_as_parameters_and_returns() {
        pub trait Summary {
            fn summarize(&self) -> String {
                String::from("Default Impl")
            }
        }
        struct Comment {
            msg: String,
        }
        impl Summary for Comment {
            fn summarize(&self) -> String {
                format!("Message: {}", self.msg)
            }
        }
        pub fn notify(item: &impl Summary) -> String {
            item.summarize()
        }
        //using trait bound syntax
        /*
            pub fn notify<T: Summary>(item: &T) {
                item.summarize()
            }
        */
        let notification = Comment {
            msg: String::from("my message"),
        };
        assert_eq!(notify(&notification), String::from("Message: my message"));
        //Notes:
        /*
            You can also specify multiple traits with + syntax
            short form:
                pub fn notify(item: &(impl Summary + Display)) {}
            generic form:
                pub fn notify<T: Summary + Display>(item: &T) {}

            You can also use the 'where' syntax when you need to declare lots of trait bounds

                fn some_function<T, U>(t: &T, u: &U) -> i32
                    where T: Display + Clone,
                          U: Clone + Debug
                {}
        */

        fn returns_summarizable() -> impl Summary {
            Comment {
                msg: String::from("Here is my comment"),
            }
        }

        assert_eq!(
            returns_summarizable().summarize(),
            "Message: Here is my comment"
        );
    }

    //Final notes on conditional implementations of methods using trait bounds
    /*
        The Pair::new is always implemented for all instances of Pair<T>
        but the Pair::cmp_display is only implemented for Pair<T>s that implement PartialOrd & Display traits
    */
    /*
    use std::fmt::Display;

    struct Pair<T> {
        x: T,
        y: T,
    }

    impl<T> Pair<T> {
        fn new(x: T, y: T) -> Self {
            Self { x, y }
        }
    }

    impl<T: Display + PartialOrd> Pair<T> {
        fn cmp_display(&self) {
            if self.x >= self.y {
                println!("The largest member is x = {}", self.x);
            } else {
                println!("The largest member is y = {}", self.y);
            }
        }
    }
    */
    #[test]
    fn lifetimes() {
        // &i32        // a reference
        // &'a i32     // a reference with an explicit lifetime
        // &'a mut i32 // a mutable reference with an explicit lifetime

        //references x and y could both be returned out of the function
        //so the compiler needs to know that the returned value will have a comparable lifetime to the two parameters passed in
        fn largest<'a>(x: &'a i32, y: &'a i32) -> &'a i32 {
            if x > y {
                x
            } else {
                y
            }
        }

        let val1 = &5;
        let val2 = &8;
        let result = largest(val1, val2);
        assert_eq!(result, &8);

        //since this struct has a member that is a reference, we need to let the rust compiler know that
        //the struct will not outlive the member, because msg would point to nothing if it goes out of scope
        //while Excerpt stays in scope
        struct Excerpt<'a> {
            msg: &'a str,
        }
        let excerpt = Excerpt { msg: "ref string" };
        assert_eq!(excerpt.msg, "ref string");

        //Lifetime Elision rules
        //Rule 1: Each parameter gets its own lifetime parameter (one parameter: fn foo<'a>(x: &'a i32), two params: fn foo<'a, 'b>(x: &'a i32, y: &'b i32), etc)
        //Rule 2: If there is one input parameter, that lifetime is assigned to all output lifetime parameters
        //Rule 3: If there are multiple parameters, but one of them is &self or &mut self because it is a method, the lifetime of self is applied to all output parameters.
        //If after applying all three rules the compiler still can't figure out the lifetimes of parameters, you will have to explicitly apply them yourself

        //the implementation of Excerpt requires the lifetime declaration because the struct Excerpt requires it
        impl<'a> Excerpt<'a> {
            //lifetime declarations can be ommited for this method because of the first elison rule
            fn level(&self) -> i32 {
                3
            }
            //lifetime declarations cam be ommited here because of the third elison rule
            fn comment_and_return(&self, announcement: &str) -> &str {
                println!("{}", announcement);
                self.msg
            }
        }

        assert_eq!(excerpt.level(), 3);
        assert_eq!(excerpt.comment_and_return("announcement"), "ref string");

        //Static lifetime lives for the duration of the program, it will be stored directly in the program's binary
        let s: &'static str = "I have a static lifetime";
        assert_eq!(s, "I have a static lifetime");
    }
}
//     _         _                        _           _   _____         _
//    / \  _   _| |_ ___  _ __ ___   __ _| |_ ___  __| | |_   _|__  ___| |_ ___
//   / _ \| | | | __/ _ \| '_ ` _ \ / _` | __/ _ \/ _` |   | |/ _ \/ __| __/ __|
//  / ___ \ |_| | || (_) | | | | | | (_| | ||  __/ (_| |   | |  __/\__ \ |_\__ \
// /_/   \_\__,_|\__\___/|_| |_| |_|\__,_|\__\___|\__,_|   |_|\___||___/\__|___/
mod ch11 {
    //Notes:
    /*
        #[cfg(test)] to designate something should only be compiled for test configuration
        #![cfg(test)] to apply cfg(test) to whole crate

        #[test] before function to designate it as a test
        #[should_panic] to flag a test that intentionally panics (this should follow the #[test] flag)
            you can add an expected parameter to make the test more precises "#[should_panic(expected = <the expected value>]"

        assert!() to assert on boolean value
        assert_eq!() to assert on the equality of two values
        assert_ne!() to assert on the inequality of two values

        #[derive(PartialEq, Debug)] to compare and print your own types with assert macros

        You can use Result<T, E> as return type for tests. This enables the use of ? operator.
        But you cannot use #[should_panic] flags on tests that return Result<T, E>. To assert that an operation
        returns an Err variant use assert!(value.is_err())

        cargo test:
            cargo test --help displays options for cargo test
            cargo test -- --help displays options that go after the -- seperator
            cargo test -- --test-threads=<number> to specify the number of threads, default is running tests in parallel, set to one to run on single thread
            cargo test -- --show-output to show any info passed to standard output (i.e. println!(), etc)
            cargo test <module_name> to test specific module of tests (i.e. cargo test ch11 -- --show-output)
            cargo test <function name> to test specific function (i.e. cargo test should_panic)
            cargo test <part of fn name> to specify any tests that start with something (i.e. cargo test using -> will run using_should_panic and using_results)

            you can also use the #[ignore] flag (following #[test] flag) to specify functions that should normally be ignored when running cargo test
            use
                'cargo test -- --ignored' to run ignored tests
            use
                'cargo test -- --include-ignored' to run all tests, including ignored ones
    */

    #[test]
    fn adding_custom_failure_messages() {
        let result = true;
        assert!(result, "The result should be {}", result);
        assert_eq!(
            result, true,
            "the value was {} but should have been {}",
            result, true
        );
    }
    #[test]
    #[should_panic(expected = "index out of bounds: the len is 3 but the index is 99")]
    fn using_should_panic() {
        let list = vec![1, 2, 3];
        let value = &list[99];
        assert_eq!(value, &100);
    }
    #[test]
    fn using_results() -> Result<(), String> {
        if true {
            Ok(())
        } else {
            Err(String::from("Not true"))
        }
    }
    #[test]
    fn show_output_example() {
        //shows output if you use the --show-output flag
        println!("Here is some output");
        assert!(true);
    }
}
//  ___    _____    ____            _           _
// |_ _|  / / _ \  |  _ \ _ __ ___ (_) ___  ___| |_
//  | |  / / | | | | |_) | '__/ _ \| |/ _ \/ __| __|
//  | | / /| |_| | |  __/| | | (_) | |  __/ (__| |_
// |___/_/  \___/  |_|   |_|  \___// |\___|\___|\__|
//                               |__/
mod ch12 {
    //see minigrep project for code

    //Notes:
    /*
        For pulling in arguments from the command line:
            use std::env;
            let args: Vec<String> = env::args().collect(); <- env::args() creates an iterator over the cli arguments. collect() then collects the elements into a collection

        Reading file:
            use std::fs;
            let contents = fs::read_to_string(filename).expect("something went wrong"); <- returns Result<String> so expect or some error handling is needed

        Using unwrap_or_else to define some custom, non-panic! error handling:
            use std::process; <- to pull in process::exit();
            let config = Config::new(&args).unwrap_or_else(|err| { <- take the error returned from Result and define custom error handling behavior in the anonymous function
                println!("Problem parsing arguments: {}", err);
                process::exit(1);
            })

        Returning unknown error type:
            use std::error::Error;
            fn run(config: Config) -> Result<(), Box<dyn Error>> { <- Since Error type can't be known at compile time we use an Error trait object allocated on heap
                let contents = fs::read_to_string(config.filename)?; <- use the '?' operator to return early from any possible error
                /* do something */
                Ok(()) <- return unit type if success
            }

        Handling errors within main:
            if let Err(e) = run(config) { <- if run returns an Err containing some error, do something
                println!("Application error: {}", e);
                process::exit(1);
            }

        Using returned fn collection in for loop:
            for line in search(&config.query, &contents) { <- take collection returned by search (Vec<&str>) and iterator over it directly
                /* do this */
            }

        Pulling in Enviroment variables:
            use std::env;
            let case_sensitive = env::var("CASE_INSENSITIVE").is_err(); <- if case_insensitive is set to 0, is error is true and case_sensitive is set
                                                                            otherwise is error is false and case sensitivity is turned off
    */
    #[test]
    fn minigrep() {
        assert!(true);
    }
}
//  _____                 _   _                   _   _                                               _____          _
// |  ___|   _ _ __   ___| |_(_) ___  _ __   __ _| | | |    __ _ _ __   __ _ _   _  __ _  __ _  ___  |  ___|__  __ _| |_ _   _ _ __ ___  ___
// | |_ | | | | '_ \ / __| __| |/ _ \| '_ \ / _` | | | |   / _` | '_ \ / _` | | | |/ _` |/ _` |/ _ \ | |_ / _ \/ _` | __| | | | '__/ _ \/ __|
// |  _|| |_| | | | | (__| |_| | (_) | | | | (_| | | | |__| (_| | | | | (_| | |_| | (_| | (_| |  __/ |  _|  __/ (_| | |_| |_| | | |  __/\__ \
// |_|   \__,_|_| |_|\___|\__|_|\___/|_| |_|\__,_|_| |_____\__,_|_| |_|\__, |\__,_|\__,_|\__, |\___| |_|  \___|\__,_|\__|\__,_|_|  \___||___/
//                                                                     |___/             |___/
mod ch13 {
    use std::thread;
    use std::time::Duration;
    #[test]
    fn closures_1() {
        fn generate_workout(intensity: u32, random_number: u32) -> u32 {
            //Define a closure that does our "expensive calculation" and then returns the value passed in
            let expensive_closure = |num| {
                //<- the parameter and return types are inferred by the usage of the closure below
                println!("calculating slowly...");
                thread::sleep(Duration::from_millis(2));
                num
            };

            //note: you could explicitly annotate the types on closures like:
            /* let expensive_colsure = |num: u32| -> u32 { */

            //also: attempting to call a closure whos types are infferred with two different types results in an error

            if intensity < 25 {
                println!("Today, do {} pushups", expensive_closure(intensity));
                println!("Next, do {} situps", expensive_closure(intensity));
            } else {
                if random_number == 3 {
                    println!("Take a break today");
                } else {
                    println!("Today run for {} minutes", expensive_closure(intensity));
                }
            }
            intensity
        }
        assert_eq!(generate_workout(7, 2), 7);
    }
    #[test]
    fn closures_2() {
        struct Cacher<T>
        //create struct
        where
            T: Fn(u32) -> u32, //<- with generic type of function that takes a u32 and returns a u32
        {
            calculation: T,     // <- calculation holds our function/closure
            value: Option<u32>, //<- value holds None at
        }

        impl<T> Cacher<T>
        where
            T: Fn(u32) -> u32,
        {
            fn new(calculation: T) -> Cacher<T> {
                //our new function sets calculation to our closure/function and sets value to None
                Cacher {
                    calculation,
                    value: None,
                }
            }

            fn value(&mut self, arg: u32) -> u32 {
                //when we call value we either
                match self.value {
                    Some(v) => v, //<- return value if one is already present
                    None => {
                        let v = (self.calculation)(arg); //<- or we call or closure/function and
                        self.value = Some(v); //<- assign the result to value
                        v //<- then return value, next time value will already be calculated, so you avoid redoing the calculation needlessly
                    }
                }
            }
        }
        //new and improved generate_workout function
        fn generate_workout(intensity: u32, random_number: u32) -> u32 {
            let mut expensive_result = Cacher::new(|num| {
                println!("Calculating slowly...");
                thread::sleep(Duration::from_millis(2));
                num
            });
            if intensity < 25 {
                println!("Today, do {} pushups", expensive_result.value(intensity));
                println!("Next, do {} situps", expensive_result.value(intensity));
            } else {
                if random_number == 3 {
                    println!("Take a break today");
                } else {
                    println!(
                        "Today, run for {} minutes",
                        expensive_result.value(intensity)
                    );
                }
            }
            intensity
        }
        assert_eq!(generate_workout(12, 9), 12);
    }
    use std::collections::HashMap;
    #[test]
    fn closures_3() {
        //Challenge
        /*
            Try modifying Cacher to hold a hash map rather than a single value.
            The keys of the hash map will be the arg values that are passed in,
            and the values of the hash map will be the result of calling the
            closure on that key. Instead of looking at whether self.value directly
            has a Some or a None value, the value function will look up the arg in
            the hash map and return the value if it’s present. If it’s not present,
            the Cacher will call the closure and save the resulting value in the
            hash map associated with its arg value.
        */
        struct Cacher<T>
        where
            T: Fn(u32) -> u32,
        {
            calculation: T,
            value: HashMap<u32, u32>,
        }
        impl<T> Cacher<T>
        where
            T: Fn(u32) -> u32,
        {
            fn new(calculation: T) -> Cacher<T> {
                Cacher {
                    calculation,
                    value: HashMap::<u32, u32>::new(),
                }
            }
            fn value(&mut self, arg: u32) -> u32 {
                match self.value.get(&arg) {
                    Some(v) => *v,
                    None => {
                        let v = (self.calculation)(arg);
                        self.value.insert(arg, v);
                        v + 1
                    }
                }
            }
        }

        let mut result = Cacher::new(|num| num + 1);

        assert_eq!(result.value(1), 3);
        assert_eq!(result.value(1), 2);
        assert_eq!(result.value(3), 5);
    }
    #[test]
    fn closures_4() {
        //capturing the enviroment
        let x = 5;
        //if value passed in z is equal to z return true
        let equal_to_x = |z| z == x; //<- captures x from scope
        let y = 5;
        assert!(equal_to_x(y));
    }

    //closures implement 3 traits
    /*

    FnOnce consumes the variables it captures from its enclosing scope, known as the closure’s environment.
    To consume the captured variables, the closure must take ownership of these variables and move them into
    the closure when it is defined. The Once part of the name represents the fact that the closure can’t take
    ownership of the same variables more than once, so it can be called only once.

    FnMut can change the environment because it mutably borrows values.

    Fn borrows values from the environment immutably.

    */
    #[test]
    fn closures_5() {
        //using move keyword on closures
        let x = 5;
        //the use of 'move' makes equal_to_x take ownership of captured variables (in this case x)
        let equal_to_x = move |z| z == x; //<-x goes out of scope after this line

        let y = 5;
        assert!(equal_to_x(y));
    }

    #[test]
    fn iterators() {
        //create iters with
        /*
            iter() <- returns immutable reference iterator
            iter_mut() <- returns mutable reference iterator
            into_iter() <- return owned iterator
        */

        let v1 = vec![1, 2, 3];
        let v1_iter = v1.iter(); //<- create iterator over v1

        for val in v1_iter {
            //<- loop over created iterator
            assert!(val > &0 && val < &4);
        }

        let v1 = vec![1, 2, 3];
        let mut v1_iter = v1.iter(); // <- iter() returns immutable references, if we wanted an iterator that takes ownership we could use into_iter()

        assert_eq!(v1_iter.next(), Some(&1)); //<- call next method directly on iterator to get next value
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);

        //Consuming adaptors
        let v1 = vec![1, 2, 3];
        let v1_iter = v1.iter();
        //Since sum() calls next() it takes ownership and consumes the iterator
        let total: i32 = v1_iter.sum(); //<- so v1_iter goes out of scope after it runs here
        assert_eq!(total, 6);

        //Iterator adaptors

        //Map
        let v1 = vec![1, 2, 3];
        //here map takes a closure to call on v1.iter() and return a new iterator, the collect takes that new iterator and turns it into a collection
        let v2: Vec<i32> = v1.iter().map(|x| x + 1).collect(); //<- here we collect the results of iterating over the iterator returned from map, which adds 1 to every value
        assert_eq!(v2, vec![2, 3, 4]);

        //Filter
        let v1 = vec![1, 2, -4, 5, -3, 6, 7, 0, -2];
        //here filter returns a boolean, if true, the value is added to the collection
        let v2: Vec<i32> = v1.into_iter().filter(|x| x >= &0).collect(); //<- into_iter takes ownership, so v1 goes out of scope here
        assert_eq!(v2, vec![1, 2, 5, 6, 7, 0]);

        //Capturing the enviroment with iterator and filter closure
        #[derive(PartialEq, Debug)]
        struct MyThing {
            num: i32,
            num2: i32,
        }

        let mut thing_collection: Vec<MyThing> = vec![
            MyThing { num: 1, num2: 2 },
            MyThing { num: 2, num2: -3 },
            MyThing { num: 3, num2: 8 },
        ];

        //here we create an iterator over our MyThing collection that takes ownership
        //then we capture the num2 member of MyThing objects and filter based on their value
        //finally collecting iterators that made it throug the filter into a new collection
        let new_thing_collection: Vec<MyThing> = thing_collection
            .into_iter()
            .filter(|x| x.num2 > 0)
            .collect();

        assert_eq!(
            new_thing_collection,
            vec![MyThing { num: 1, num2: 2 }, MyThing { num: 3, num2: 8 }]
        );

        //Create your own iterator
        struct CountToFive {
            count: u32,
        }
        impl CountToFive {
            fn new() -> CountToFive {
                CountToFive { count: 0 }
            }
        }
        impl Iterator for CountToFive {
            type Item = u32;

            fn next(&mut self) -> Option<Self::Item> {
                if self.count < 5 {
                    self.count += 1;
                    Some(self.count)
                } else {
                    None
                }
            }
        }

        let mut counter = CountToFive::new();

        assert_eq!(counter.next(), Some(1));
        assert_eq!(counter.next(), Some(2));
        assert_eq!(counter.next(), Some(3));
        assert_eq!(counter.next(), Some(4));
        assert_eq!(counter.next(), Some(5));
        assert_eq!(counter.next(), None);
    }
    #[test]
    fn improved_minigrep() {
        //check improved_minigrep project folder for complete code

        //The changes:
        /*
           enviroment arguments are now passed directly to Config::new()
               let config = Config::new(env::args()).unwarp_or_else(|err|) { <- env::args() returns Args which implements iterator trait, so we can use next()
                   etc...

           changes to Config::new()
                   pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
                       args.next(); <-throw away the first value that contains the program name

                       let query = match args.next() { <-match on each next() value and pass those values into Config members below
                           Some(arg) => arg,
                           None => return Err("Didn't get a query string"),
                       };

                       let filename = match args.next() {
                           Some(arg) => arg,
                           None => return Err("Didn't get a file name"),
                       };

                       etc...
                   }

           updated search functions using iterators
                   pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
                       contents
                           .lines()
                           .filter(|line| line.contains(query)) <-make iterator over lines, filter out the ones that don't contain query, collect results and return it
                           .collect()
                   }

                   pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
                       contents
                           .lines()
                           .filter(|line| line.to_lowercase().contains(&query.to_lowercase())) <-this time just change the lines and query to lowercase first before filterings
                           .collect()
                   }
        */
        assert!(true);
    }
}
//   ____                         ___      ____           _              _
//  / ___|__ _ _ __ __ _  ___    ( _ )    / ___|_ __ __ _| |_ ___  ___  (_) ___
// | |   / _` | '__/ _` |/ _ \   / _ \/\ | |   | '__/ _` | __/ _ \/ __| | |/ _ \
// | |__| (_| | | | (_| | (_) | | (_>  < | |___| | | (_| | ||  __/\__ \_| | (_) |
//  \____\__,_|_|  \__, |\___/   \___/\/  \____|_|  \__,_|\__\___||___(_)_|\___/
//                 |___/
mod ch14 {
    #[test]
    fn release_profiles() {
        //Notes:
        /*
            cargo build: defaults to dev profile
            cargo build --release: to invoke the release profile

            You can modify default profile settings in the Cargo.toml file
            i.e.
                [profile.dev]
                opt-level = 0

                [profile.release]
                opt-level = 3

            you can find all the profile options here: https://doc.rust-lang.org/cargo/reference/profiles.html
        */
        assert!(true);
    }
    #[test]
    fn publishing_crates() {
        //Documentation Comments: use three slashes to create documentation comments before function. (Supports markdown notation)

        ///Adds one to the number given
        ///
        /// # Examples
        /// ```
        /// let arg = 5;
        /// let answer = add_one(arg);
        ///
        /// assert_eq!(6, answer);
        /// ```
        pub fn add_one(x: i32) -> i32 {
            x + 1
        }
        /*
            the '# Examples' markdown heading makes a section for example code
            other commonly used sections include
                Panics: to describe ways a function could panic
                Errors: If a function returns a Result describe the kinds of errors that might occur and how to handle those errors
                Safety: If a function is unsafe to call, explain why it is unsafe and describe the invariants that are needed to remain safe
        */

        //generate documentation with 'cargo doc'
        //directly build and open docs with 'cargo doc --open'
        //running cargo test will also test documentation examples

        //use double slash followed by ! to add crate level documentation
        /*
        //! # My Crate
        //! 'my_crate' is a collection of utilities ...
         */

        //Exporting public apis
        /*
            you can use 'pub use' to re-export items to the top level of crate

            pub use self::things::other_things::deeply_nested_func;
            pub use self::different_things::SomeType;

            pub mod things {
                pub mod other_things {
                    pub fn deeply_nested_func();
                }
            }

            pub mod different_things {
                pub struct SomeType;
            }
        */
        assert!(true);
    }
    #[test]
    fn crates_io_website() {
        //after logging in with github, go to crates.io/me/ to retrive API key
        //log into crates by invoking 'cargo login <api_key>'

        //before publishing you must have these fields under [package] in Cargo.toml
        /*
            1. have a unique crate name
                i.e. 'name = "my_crate_name"'
            2. have a short description field
                i.e. 'description = "a short description of my crate"'
            3. have a license
                i.e. 'license = "MIT"'

            Example [package] section that is ready to publish
                [package]
                name = "guessing_game"
                version = "0.1.0"
                edition = "2021"
                description = "A fun game where you guess what number the computer has chosen."
                license = "MIT OR Apache-2.0"

                [dependencies]
                etc...

            when ready just run 'cargo publish'

            when ready to update your crate, change value in [package] version field and run 'cargo publish' again to upload new version

            to yank a bad version (but allow people that are already depending on it to keep using it)
            use 'cargo yank --vers <version number>

            undo by adding --undo to the command
        */
        assert!(true);
    }
    #[test]
    fn cargo_workspaces() {
        //check workspace project file
        assert!(true);
    }

    //Installing binaries with cargo install
    /*
        'cargo install' allows you to install and use binary crates locally
        for example:
            if you want to install the program ripgrep run:
                cargo install ripgrep
    */

    //Cargo custom commands
    /*
        if a binary in your $PATH is named cargo-<something> you can run it as if it was a cargo subcommand
        you invoke it by running 'cargo <something>'

        Custom commands are listed when you run 'cargo --list'
    */
}
//  ____                       _     ____       _       _
// / ___| _ __ ___   __ _ _ __| |_  |  _ \ ___ (_)_ __ | |_ ___ _ __ ___
// \___ \| '_ ` _ \ / _` | '__| __| | |_) / _ \| | '_ \| __/ _ \ '__/ __|
//  ___) | | | | | | (_| | |  | |_  |  __/ (_) | | | | | ||  __/ |  \__ \
// |____/|_| |_| |_|\__,_|_|   \__| |_|   \___/|_|_| |_|\__\___|_|  |___/
mod ch15 {
    use std::borrow::Borrow;

    //When to use box types
    /*
       -When you have a type whose size can’t be known at compile time and you want to use a value of that type in a context that requires an exact size
       -When you have a large amount of data and you want to transfer ownership but ensure the data won’t be copied when you do so
       -When you want to own a value and you care only that it’s a type that implements a particular trait rather than being of a specific type
    */

    #[test]
    fn box_type() {
        //basic syntax
        let b = Box::new(5); //<- creates a box containing 5 on the heap
        assert_eq!(*b, 5);

        //recursive types with boxes
        enum List {
            Cons(i32, Box<List>), //<- pack List in a Box which has a specific size and points to List rather than containing a recursive chain of nested Lists
            Nil,
        }
        let list = List::Cons(
            1,
            Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))),
        );
        assert!(true);
    }
    #[test]
    fn deref_trait() {
        //using box<T> like a reference
        let x = 5;
        let y = Box::new(x);
        assert_eq!(5, x);
        assert_eq!(5, *y); //<- because of the deref trait, we can dereference a Box as if it was a reference (i.e. let y = &x)

        //defining our own smart pointer
        use std::ops::Deref; //<- Deref isn't included in prelude, needs to be brought into scope

        struct MyBox<T>(T); //<- tuple struct (i.e. struct with ordered unnamed members)

        impl<T> MyBox<T> {
            fn new(x: T) -> MyBox<T> {
                MyBox(x)
            }
        }

        //This trait is required to be implemented to deref your type
        impl<T> Deref for MyBox<T> {
            type Target = T; //<- defines the associated type for the Deref trait to use

            fn deref(&self) -> &Self::Target {
                &self.0 //<- return a reference to our data allowing us to now use the * operator
            }
        }

        //with deref defined we can now use the * operator like with Box<T>
        let x = 5;
        let y = MyBox::new(x);
        assert_eq!(5, x);
        assert_eq!(5, *y); //<- now when we call *y, rust is actually calling *(y.deref()) behind the scenes

        //implicit deref coercions
        fn hello(name: &str) {
            println!("Hello, {}", name);
            assert!(true);
        }
        //deref coercion makes it possible to call hello() with a value of type MyBox<String>
        let m = MyBox::new(String::from("Rust"));
        hello(&m); //<- because of the deref trait, rust can turn &MyBox<String> into &String, and the std implmentation of Deref on String can turn &String into &str

        //without deref coercsion we would have to pass m like:
        hello(&(*m)[..]); //<- the (*m) derefs MyBox<String> into String, then & and [..] takes a string slice of String to give us a &str

        //notes:
        /*
            -The Deref trait overrides the * operator on immutable references
            -You can use the DerefMut to override the * operator on mutable references

            Rust does deref coercion when it finds types and trait impls in three cases

            -From &T to &U when T: Deref<Target=U>
            -From &mut T to &mut U when T: DerefMut<Target=U>
            -From &mut T to &U when T: Deref<Target=U>
        */
        assert!(true);
    }
    #[test]
    fn drop_trait() {
        //Drop trait is included in prelude, so we don't need to bring it in scope
        struct CustomSmartPointer {
            data: String,
        }

        //CustomSmartPoint object goes out of scope, the Drop trait will call the drop function
        impl Drop for CustomSmartPointer {
            fn drop(&mut self) {
                //use this function to clean up your Object data
                println!("Dropping CustomSmartPointer with data `{}`!", self.data);
            }
        }
        {
            let c = CustomSmartPointer {
                data: String::from("custom data"),
            }; //<- Custom smart pointer is created here
               /* do stuff */
        } //<- at this point c goes out of scope and the Drop trait will call our drop function to clean up the data
        assert!(true);

        //force an object to be dropped early
        //use std::mem::drop; //<- drop is in prelude, so we don't need to bring it into scope
        {
            let c = CustomSmartPointer {
                data: String::from("drop early please"),
            };
            /* do stuff */
            drop(c); //<- this will drop c early and stop c from being dropped again at the end of the scope
                     /* do more stuff */
        } //<-c has already been dropped and won't be dropped here
    }
    #[test]
    fn reference_count_smart_pointers() {
        use std::rc::Rc; //<- bring Rc into scope since it isn't in prelude

        enum List {
            Cons(i32, Rc<List>), //<- make our cons list again with Reference counted smart pointers this time
            Nil,
        }
        use List::{Cons, Nil};
        {
            //This time we will make Lists that branch and have multiple references, so we need to use Rc<T> instead of Box<T> to keep track of the reference count
            let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil))))); //<- Ref count 1 of 'a'
            let b = Cons(3, Rc::clone(&a)); //<- Ref count 2, b holds a reference to 'a'
            {
                let c = Cons(4, Rc::clone(&a)); //<- Ref count 3, c holds a reference to 'a'
            } //<- c goes out of scope, Ref count 2, now there are only two references to 'a'
        } //<- a and b go out of scope, Ref count 0 for 'a' so 'a' is dropped

        //Note: you can use Rc::strong_count(&<variable name>) function to get the current reference count
        assert!(true);
    }
    #[test]
    fn interior_mutability_pattern() {
        //normally you cant mutably borrow an immutable value
        /*
            let x = 5;
            let y = &mut x; <- this fails
        */
        //we can use RefCell<T> to get interior mutability that is runtime checked vs compile time checked

        pub trait Messenger {
            //Define a Messenger trait that requires a send(&self, &str); function
            fn send(&self, msg: &str);
        }

        pub struct LimitTracker<'a, T: Messenger> {
            //our limit tracker requires a messenger member the implements the Messenger trait
            messenger: &'a T,
            value: usize,
            max: usize,
        }

        impl<'a, T> LimitTracker<'a, T>
        where
            T: Messenger,
        {
            pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
                //new() takes a &T that implements the Messanger trait and a max value for our LimitTracker
                LimitTracker {
                    messenger,
                    value: 0,
                    max,
                }
            }

            pub fn set_value(&mut self, value: usize) {
                //Set value sets our value member and then sends a message if we exceed certain quotas
                //those messages are calls to send() on our messenger member that implements Messenger
                self.value = value;

                let percentage_of_max = self.value as f64 / self.max as f64;

                if percentage_of_max >= 1.0 {
                    self.messenger.send("Error: You are over your quota!");
                } else if percentage_of_max >= 0.9 {
                    self.messenger
                        .send("Urgent warning: You've used up over 90% of your quota!");
                } else if percentage_of_max >= 0.75 {
                    self.messenger
                        .send("Warning: You've used up over 75% of your quota!");
                }
            }
        }
        use std::cell::RefCell;

        //Creating a Mock messenger to test our LimitTrackers functionality
        struct MockMessenger {
            sent_messages: RefCell<Vec<String>>, //<-Store the messages sent from limit tracker to see if we get expected behavior
        }

        impl MockMessenger {
            fn new() -> MockMessenger {
                MockMessenger {
                    sent_messages: RefCell::new(vec![]),
                }
            }
        }

        //implement Messenger trait for MockMessenger
        //send takes an immutable reference to self, so to mutate the internal state of MockMessanger, we need to hold sent_messages in a RefCell<T>
        //this will give us the interior mutability we need
        impl Messenger for MockMessenger {
            fn send(&self, message: &str) {
                self.sent_messages.borrow_mut().push(String::from(message)); //<- use borrow_mut() to temporarily borrow mutability from sent_messages and push our string to it
            }
        }

        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);
        limit_tracker.set_value(80);
        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1); //<- here we don't need mutability, so we just borrow from RefCell to get the underlying lenght of the member

        //Note: RefCell<T> implements the borrow() and borrow_mut() methods to safetly access its wrapped type
        //RefCells borrow_mut() creates RefMut<T> smart pointers
        //and borrow() creates Ref<T> smart pointers
        //the same compile time rules apply. 1 mutable reference OR many immutable references, but not both at the same time
        //if you violate these rules you will get a runtime panic
        assert!(true);
    }
    #[test]
    fn mutilple_owners_of_mutable_data() {
        #[derive(Debug)]
        enum List {
            Cons(Rc<RefCell<i32>>, Rc<List>), //<- by packing i32 in a RefCell wrapped by Rc, we can borrow mutability from the value even though it
            //is contained in an immutable Rc wrapper
            Nil,
        }

        use std::cell::RefCell;
        use std::rc::Rc;
        use List::{Cons, Nil};

        fn main() {
            let value = Rc::new(RefCell::new(5));

            let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

            let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
            let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

            //first we dereference the Rc<T> with * giving us the underlying RefCell<T>
            //then we use borrow_mut() to get a mutable reference to RefCell's T value and add 10
            *value.borrow_mut() += 10;

            println!("a after = {:?}", a);
            println!("b after = {:?}", b);
            println!("c after = {:?}", c);
        }

        assert!(true);
    }
    #[test]
    fn preventing_reference_cycles() {
        use std::cell::RefCell;
        use std::rc::{Rc, Weak};

        #[derive(Debug)]
        struct Node {
            value: i32,
            parent: RefCell<Weak<Node>>, //<- we want weak refs for parents, because you may remove a child without removing a parent
            children: RefCell<Vec<Rc<Node>>>, //<- but we want strong refs for children because if we remove a parent, we want to remove its children
                                              //So now with this node structure, we have a ref to a nodes parent and can remove the current node without removing the parent
                                              //conversely if this nodes parent is removed this node will also be removed
        }

        let leaf = Rc::new(Node {
            value: 3,
            parent: RefCell::new(Weak::new()), //<- no parent ref yet
            children: RefCell::new(vec![]),    //<- no children
        });

        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()), //<- no parent
            children: RefCell::new(vec![Rc::clone(&leaf)]), //<- 1 child (leaf)
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch); //<- use downgrade to give leaf's parent member a Weak<T> ref to branch
        println!("Leaf parent = {:?}", leaf.parent.borrow().upgrade()); //<- use the upgrade method to get an Option<Rc<T>> and get our ref to leaf's parent Node

        //again, you can use Rc::strong_count and Rc::weak_count to get counts of each ref type.
        assert!(true);
    }
}
//   ____
//  / ___|___  _ __   ___ _   _ _ __ _ __ ___ _ __   ___ _   _
// | |   / _ \| '_ \ / __| | | | '__| '__/ _ \ '_ \ / __| | | |
// | |__| (_) | | | | (__| |_| | |  | | |  __/ | | | (__| |_| |
//  \____\___/|_| |_|\___|\__,_|_|  |_|  \___|_| |_|\___|\__, |
//                                                       |___/
mod ch16 {
    use std::time::Duration;

    #[test]
    fn creating_thread_with_spawn() {
        use std::thread; //<- needed for spawn method
        use std::time::Duration;

        //create a thread with spawn, passing a closure of what we want to run into the thread
        thread::spawn(|| {
            for i in 1..10 {
                println!("{i} from spawn");
                thread::sleep(Duration::from_millis(1));
            }
        });

        for i in 1..10 {
            println!("{i} from main");
            thread::sleep(Duration::from_millis(20));
        }
        assert!(true);
    } //<- if the main thread ends before the spawned thread, the spawned thread will stop along with the main thread even if it isn't finished yet
    #[test]
    fn join_handles() {
        use std::thread;
        use std::time::Duration;
        //Store our thread in a variable of type JoinHandle
        let handle = thread::spawn(|| {
            for i in 1..10 {
                println!("{i} in handle");
                thread::sleep(Duration::from_millis(1));
            }
        });

        for i in 1..5 {
            println!("{i} in main");
            thread::sleep(Duration::from_millis(1));
        }
        //Blocking thread with join
        handle.join().unwrap(); //<- join() will make sure our thread finishes execution before continuing past this point
                                //if we placed join() before our second for loop, the entire handle thread would have finished before starting the second for loop
        assert!(true);
    }
    #[test]
    fn move_closures_with_threads() {
        use std::thread;
        use std::time::Duration;

        let v = vec![1, 2, 3];

        //since we don't know how long handle will run, we need to take ownership of v, so we use the move keyword to take ownership of any variables we capture in the closure
        let handle = thread::spawn(move || {
            println!("here's a vector: {v:?}"); //<- v is now owned by the closure
        });
        handle.join().unwrap();
        assert!(true);
    }
    #[test]
    fn using_message_to_pass_data() {
        use std::sync::mpsc;
        use std::thread;

        //this creates a "multiple producer, single consumer" channel, with tx being the transmitter and rx being the reciever
        let (tx, rx) = mpsc::channel(); //<- we destructure the tuple returned from mpsc::channel() into tx and rx variables

        //we create a new thread here and use the move keyword so the closure can take ownership of our tx variable
        thread::spawn(move || {
            let val = String::from("hi");
            tx.send(val).unwrap(); //<- inside our spawn thread we will transmit a value on our channel
                                   //if our reciever on this channel is already closed our send value would return an Err, so normally we would want code to handle this situation
        });

        let received = rx.recv().unwrap(); //<- recv() will block its enclosing thread's execution until a value is retreived from the channel
                                           //when the sending end of the channel closes, recv() will retrun an error to signal that no more values will be coming
                                           //conversly you could use try_recv() to periodically check for messages on channel, and handle the message if one is available

        println!("Got: {received}");
        assert_eq!(received, String::from("hi"));
    }
    #[test]
    fn ownership_transference() {
        use std::sync::mpsc;
        use std::thread;
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let val = String::from("hi");
            tx.send(val).unwrap(); //<- At this point val's ownership is transfered to the reciever
                                   //println!("val is {}", val); //<- so this use of val would cause an error since this thread no longer owns val
        });

        let received = rx.recv().unwrap();
        println!("Got: {}", received);
        assert_eq!(received, String::from("hi"));
    }
    #[test]
    fn seeing_the_receiver_wait() {
        use std::sync::mpsc;
        use std::thread;
        use std::time::Duration;

        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("thread"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                thread::sleep(Duration::from_millis(100));
            }
        });

        //here we treat rx as an iterator, when the channel is closed the iteration will end.
        for received in rx {
            println!("Got: {received}");
        }
        assert!(true);
    }
    #[test]
    fn multiple_producers() {
        use std::sync::mpsc;
        use std::thread;
        use std::time::Duration;

        let (tx, rx) = mpsc::channel();
        let tx1 = tx.clone(); //<- clone to transmitter to have another producer we can use

        //first thread will use our cloned tx1 transmitter
        thread::spawn(move || {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("thread"),
            ];

            for val in vals {
                tx1.send(val).unwrap();
                thread::sleep(Duration::from_millis(100));
            }
        });

        //second thread will use our original tx transmitter
        thread::spawn(move || {
            let vals = vec![
                String::from("more"),
                String::from("messages"),
                String::from("for"),
                String::from("you"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                thread::sleep(Duration::from_millis(100));
            }
        });

        //now rx will be getting messages from multiple concurrent threads
        for received in rx {
            println!("Got: {received}");
        }
        assert!(true);
    }
    #[test]
    fn mutex_api() {
        use std::sync::Mutex;

        let m = Mutex::new(5); //<- create a mutex holding the value 5
        {
            let mut num = m.lock().unwrap(); //<- give num the mutex lock for m to access its underlying data
            *num = 6; //<- modify the mutex data
                      //let mut other_num = m.lock().unwrap(); //<- cant do this because num already owns m's lock
        } //<- when num lock goes out of scope, the lock is released
          //now you can hand the lock to other variables
        println!("m = {m:?}");
        assert_eq!(*m.lock().unwrap(), 6); //<- to use the value again in assertion, we need to get the lock again, unwrap it and dereference
    }
    #[test]
    fn sharing_a_mutex() {
        use std::sync::{Arc, Mutex};
        use std::thread;

        let counter = Arc::new(Mutex::new(0)); //<- we wrap counter in an atomic reference counter than can be passed into threads safely
        let mut handles = vec![];

        for _ in 0..10 {
            let counter = Arc::clone(&counter); //<-each iteration we make a new reference clone of our counter
            let handle = thread::spawn(move || {
                let mut num = counter.lock().unwrap();
                //The thread takes the counter, unlocks it and then mutates the value
                *num += 1;
            });
            handles.push(handle); //<- we then push the handle on our Vec of handles
        }

        //Finally we call join on all our handles
        for handle in handles {
            handle.join().unwrap();
        }
        //At this point we should've had 10 threads that all called += 1 on our counter
        //leaving us with a count of 10
        println!("Result: {}", *counter.lock().unwrap());
        assert!(true);
    }

    //Notes:
    /*
        std::marker traits Sync and Send
        -The Send marker inidcation that ownership of values with this trait can pass ownership between threads.
            (any type composed of entirely Send types is automatically marked as Send as well)
        -The Sync marker trait indicates a type is safe to be referenced across multiple threads.

        Implementing Send and Sync manually requires implementing Unsafe Rust code.
    */
}
//   ___   ___  ____    _____          _
//  / _ \ / _ \|  _ \  |  ___|__  __ _| |_ _   _ _ __ ___  ___
// | | | | | | | |_) | | |_ / _ \/ _` | __| | | | '__/ _ \/ __|
// | |_| | |_| |  __/  |  _|  __/ (_| | |_| |_| | | |  __/\__ \
//  \___/ \___/|_|     |_|  \___|\__,_|\__|\__,_|_|  \___||___/
mod ch17 {
    #[test]
    fn common_behavior_w_traits() {
        //Define a trait with the function draw
        pub trait Draw {
            fn draw(&self);
        }

        //define a struct that holds a Vec of Trait objects that implement Draw
        //(trait objects need to either be references '&' or held by smart pointers 'Box<T>' and the trait preceded by a 'dyn' keyword)
        pub struct Screen {
            pub components: Vec<Box<dyn Draw>>,
        }

        impl Screen {
            //now run can iterate through all the components in the Vec that implement draw() and call that function
            pub fn run(&self) {
                for component in self.components.iter() {
                    component.draw();
                }
            }
        }

        //What implementing trait for component might look like
        pub struct Button {
            pub width: u32,
            pub height: u32,
            pub label: String,
        }
        impl Draw for Button {
            fn draw(&self) {
                /* do something */
            }
        }

        //implementing mock SelectBox
        struct SelectBox {
            width: u32,
            height: u32,
            options: Vec<String>,
        }
        impl Draw for SelectBox {
            fn draw(&self) {
                /* do something */
            }
        }

        //creating our screen instance containing different types that implement Draw trait
        let mut screen = Screen { components: vec![] };

        screen.components.push(Box::new(Button {
            width: 10,
            height: 10,
            label: String::from("my box"),
        }));

        screen.components.push(Box::new(SelectBox {
            width: 90,
            height: 90,
            options: vec![
                String::from("Yes"),
                String::from("No"),
                String::from("Maybe"),
            ],
        }));

        //this will now call run on every component in Screen
        screen.run();

        assert!(true);
    }
    #[test]
    fn state_pattern() {
        //Post contains a state and content
        pub struct Post {
            state: Option<Box<dyn State>>,
            content: String,
        }
        impl Post {
            pub fn new() -> Post {
                Post {
                    state: Some(Box::new(Draft {})), //<- creating a new post sets state to Draft
                    content: String::new(),
                }
            }
            //All states implement add text
            pub fn add_text(&mut self, text: &str) {
                self.content.push_str(text);
            }
            //But states implement their own version of content, request_review and approve
            //but content has a default implementation that most states inherit
            pub fn content(&self) -> &str {
                self.state.as_ref().unwrap().content(self)
            }
            pub fn request_review(&mut self) {
                if let Some(s) = self.state.take() {
                    self.state = Some(s.request_review())
                }
            }
            pub fn approve(&mut self) {
                if let Some(s) = self.state.take() {
                    self.state = Some(s.approve())
                }
            }
        }

        trait State {
            //the required interface for State
            fn request_review(self: Box<Self>) -> Box<dyn State>;
            fn approve(self: Box<Self>) -> Box<dyn State>;
            fn content<'a>(&self, post: &'a Post) -> &'a str {
                ""
            }
        }
        struct Draft {}

        impl State for Draft {
            //here we update State to Pending
            fn request_review(self: Box<Self>) -> Box<dyn State> {
                Box::new(PendingReview {})
            }
            //and approve does nothing so we just return self
            fn approve(self: Box<Self>) -> Box<dyn State> {
                self
            }
        }

        struct PendingReview {}

        impl State for PendingReview {
            //request_review does nothing in pending state, so we just return self
            fn request_review(self: Box<Self>) -> Box<dyn State> {
                self
            }
            //approve returns our state updated to publish
            fn approve(self: Box<Self>) -> Box<dyn State> {
                Box::new(Published {})
            }
        }

        struct Published {}

        impl State for Published {
            //request_review and approve mean nothing in published state, so we just return self for these
            fn request_review(self: Box<Self>) -> Box<dyn State> {
                self
            }
            fn approve(self: Box<Self>) -> Box<dyn State> {
                self
            }
            //but now content overrides the default implementation of content and returns the content of our post
            fn content<'a>(&self, post: &'a Post) -> &'a str {
                &post.content
            }
        }

        //Now post.approve() will return nothing until we've mutated into the Published state first
        let mut post = Post::new();
        post.add_text("I ate a salad for lunch today");
        assert_eq!("", post.content());
        post.request_review();
        assert_eq!("", post.content());
        post.approve();
        assert_eq!("I ate a salad for lunch today", post.content());
        assert!(true);
    }
    #[test]
    fn encoding_behavior_as_types() {
        //Doing the same thing as our state pattern Post impl
        //but in a more concise way

        //All type states have a single member of String
        pub struct Post {
            content: String,
        }

        pub struct DraftPost {
            content: String,
        }

        pub struct PendingReviewPost {
            content: String,
        }

        //Only Post implements a content() function that returns the types String value
        //But when we create a Post we first return a DraftPost with its own impl
        impl Post {
            pub fn new() -> DraftPost {
                DraftPost {
                    content: String::new(),
                }
            }

            pub fn content(&self) -> &str {
                &self.content
            }
        }

        //DraftPost has the ability to add_text and request review, there is no interface for accessing content
        impl DraftPost {
            pub fn add_text(&mut self, text: &str) {
                self.content.push_str(text);
            }
            //Our request review simply returns the new type PendingReviewPost and we pass in our member String content
            pub fn request_review(self) -> PendingReviewPost {
                PendingReviewPost {
                    content: self.content,
                }
            }
        }

        //PendingReviewPost only implements an approve() function that returns an actual Post type where we can
        //finally access the content through the Post interface
        impl PendingReviewPost {
            pub fn approve(self) -> Post {
                Post {
                    content: self.content,
                }
            }
        }

        //Here we greatly simplified the State Pattern problem with the added benefit of not being able to get an invalid state
        //Each type implements only the behavior it needs, so there is no way to invalidate the invariant of the state

        let mut post = Post::new();

        post.add_text("I ate a salad for lunch today");
        let post = post.request_review();
        let post = post.approve();

        assert_eq!("I ate a salad for lunch today", post.content());
    }
}
//  ____       _   _                                        _   __  __       _       _     _
// |  _ \ __ _| |_| |_ ___ _ __ _ __  ___    __ _ _ __   __| | |  \/  | __ _| |_ ___| |__ (_)_ __   __ _
// | |_) / _` | __| __/ _ \ '__| '_ \/ __|  / _` | '_ \ / _` | | |\/| |/ _` | __/ __| '_ \| | '_ \ / _` |
// |  __/ (_| | |_| ||  __/ |  | | | \__ \ | (_| | | | | (_| | | |  | | (_| | || (__| | | | | | | | (_| |
// |_|   \__,_|\__|\__\___|_|  |_| |_|___/  \__,_|_| |_|\__,_| |_|  |_|\__,_|\__\___|_| |_|_|_| |_|\__, |
//                                                                                                 |___/
mod ch18 {
    use core::num;

    #[test]
    fn if_let_expressions() {
        let color: Option<String> = None;
        let num = 5;
        let age: Result<u8, _> = "35".parse();

        //we can chain multiple pattern matching types together, like if let, else if, else if let, etc
        //if the Option<String> color contains a value execute this if state, otherwise
        if let Some(color) = color {
            assert!(false);
        } else if num == 6 {
            //<- if num is equal to 6, do this else if branch, otherwise
            assert!(false);
        } else if let Ok(age) = age {
            //<- if age returns an Ok value from parse do this, otherwise
            assert!(true);
        } else {
            //<- do this
            assert!(false);
        }
    }
    #[test]
    fn while_let_loops() {
        let mut stack = Vec::new();

        stack.push(1);
        stack.push(2);
        stack.push(3);

        //while stack.pop() keeps returning a Some(T) value, the loop runs
        while let Some(top) = stack.pop() {
            println!("{top}");
        }
        assert!(true);
    }
    #[test]
    fn for_loops() {
        let v = vec!['a', 'b', 'c'];

        //even with for loops, we are just pattern matching
        //the loop will continue running as long as the iterator is returning a Some(T) value
        //enumerate returns a tuple of (value index, value) and that is the pattern we are matching against in the for loop
        for (index, value) in v.iter().enumerate() {
            println!("{value} is at index {index}");
        }
        assert!(true);
    }
    #[test]
    fn let_statements() {
        //even let statements are pattern matching (i.e. 'let PATTERN = EXPRESSION')
        let x = 5; //<- in this pattern we are saying "bind what matches here to variable x" and because x is the whole pattern "bind everything to x, whatever the value"

        //for a more clear example
        let (x, y, z) = (1, 2, 3); //<- here we are destructuring and will only have a match if the value being assigned matches the variable
                                   //for example let (x, y) = (1, 2, 3); won't match and will not compile
                                   //if we didn't need all the values we could use place holders like _ and ..
        let (x, _, z) = (1, 2, 3); //<- here we ignore the second parameter
        let (x, ..) = (1, 2, 3, 4); //<- here we only care about the first param and ignore everything else
        assert!(true);
    }
    #[test]
    fn pattern_syntax() {
        //matching literals
        let x = 1;

        match x {
            1 => assert!(true),
            2 => assert!(false),
            _ => assert!(false),
        }

        //matching named variables
        let x = Some(5);
        let y = 10;

        match x {
            Some(50) => println!("Got 50"),
            //because match is a new scope, we are declaring a new 'y' in this scope and it will match on a Some containing any value
            Some(y) => println!("Matched, y = {:?}", y),
            _ => println!("Default case, x = {:?}", x),
        }
        //here the match goes out of scope and the 'y' declared within and the 'y' in this scope is still equal to 10 and x is equal to Some(5)
        println!("at the end: x = {:?}, y = {:?}", x, y);

        //multiple patterns
        let x = 1;

        match x {
            1 | 2 => assert!(true), //<- use the | (OR) operator to match on multiple patterns
            3 => assert!(false),
            _ => assert!(false),
        }

        //matching ranges of values
        let x = 5;

        match x {
            1..=5 => assert!(true), //<- use the ..= syntax to match on an inclusive range (in this case 1 to 5 inclusive)
            _ => assert!(false),
            //note: ranges are only allowed for numberic values or char values
            //for example with char: 'a'..='j' for an inclusive range of a to j
        }
    }
    #[test]
    fn destructuring_values() {
        struct Point {
            x: i32,
            y: i32,
        }

        let p = Point { x: 0, y: 7 };

        let Point { x: a, y: b } = p; //<- destructure Point p into two variables a and b
        assert_eq!(0, a);
        assert_eq!(7, b);

        //shorter syntax
        let Point { x, y } = p; //<- since we used the names x and y we can destructure straight into those variables from p
        assert_eq! {0, x};
        assert_eq!(7, b);

        //matching on struct literals
        match p {
            Point { x, y: 0 } => assert!(false), //<- if y == 0 we match, and we have x we can also use as a variable here
            Point { x: 0, y } => assert!(true), //<- if x == 0 we match (we do), and we have y we can also use as a variable here
            Point { x, y } => assert!(false),   //<- here we match on any value of x and y
        }
    }
    #[test]
    fn destructuring_enums1() {
        enum Message {
            Quit,
            Move { x: i32, y: i32 },
            Write(String),
            ChangeColor(i32, i32, i32),
        }

        fn main() {
            let msg = Message::ChangeColor(0, 160, 255);
            //We match on the various Enums in the first match arm
            //but we can pattern match on the elements contained within the Enum as well
            match msg {
                Message::Quit => {
                    println!("The Quit variant has no data to destructure.")
                }
                Message::Move { x, y } => {
                    //<- here we use a similar pattern to the one we used for matching structs
                    println!("Move in the x direction {} and in the y direction {}", x, y);
                }
                //for these two we use a similar matching method as we used with tuples
                Message::Write(text) => println!("Text message: {}", text),
                Message::ChangeColor(r, g, b) => {
                    println!("Change the color to red {}, green {}, and blue {}", r, g, b)
                }
            }
        }
        assert!(true);
    }
    #[test]
    fn destructuring_nested_structs_and_enums() {
        enum Color {
            Rgb(i32, i32, i32),
            Hsv(i32, i32, i32),
        }

        enum Message {
            Quit,
            Move { x: i32, y: i32 },
            Write(String),
            ChangeColor(Color),
        }

        let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

        //the first level matches on our Message enum
        match msg {
            Message::ChangeColor(Color::Rgb(r, g, b)) => {
                //<- then we destructure our Color Enum into our match arm
                println!("Change the color to red {}, green {}, and blue {}", r, g, b)
            }
            Message::ChangeColor(Color::Hsv(h, s, v)) => println!(
                "Change the color to hue {}, saturation {}, and value {}",
                h, s, v
            ),
            _ => (),
        }

        //complicated example of nested destructuring of structs and tuples
        // let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
        assert!(true);
    }
    #[test]
    fn ignoring_values() {
        //ignoring with _
        //we can use _ as a wildcard to match any value but not bind the value
        //but we can also use it in a function signature
        fn foo(_: i32, y: i32) {
            println!("only use {y}");
        }

        foo(3, 4);
        //ignoring parameters can be useful for implementing certain traits where you don't need all values in a function signature

        //ignoring part of a value with nested _
        let mut a = Some(5);
        match a {
            Some(_) => assert!(true), //<- here we don't care about the contained value, we just care about matching on the Some container
            _ => assert!(false),
        }

        //ignoring multiple places in one pattern
        let numbers = (1, 2, 3, 4, 5);
        match numbers {
            (first, _, third, _, fifth) => {
                println!("some number: {first} {third} {fifth}");
                assert!(true);
            }
            _ => assert!(false),
        }

        //ignoring an unused variable
        let _x = 5; //<- prepend variable with underscore so rust doesn't warn you about used variables
                    //this still binds 5 to x. you are only supressing the compiler warning

        //ignoring the remaining parts with ..
        let numbers = (1, 2, 3, 4, 5);
        match numbers {
            (1, ..) => assert!(true), //<- match if the first value matches, ignoring the rest of the structure with .. syntax
            //this is shorthand for typing (1, _, _, _, _)
            _ => assert!(false),
        }
        let numbers = (1, 2, 3, 4, 5);
        match numbers {
            //the .. syntax will take up as much space as it needs to
            (a, .., e) => {
                println!("values {a} & {e}");
                assert!(true);
            }
            _ => assert!(false),
        }
        //the .. sytax must be unambiguous trying to do (.., x, ..) wont compile since rust won't know which values are intended for matchin
    }
    #[test]
    fn match_guards() {
        let num = Some(4);
        match num {
            Some(x) if x % 2 == 0 => {
                //<- here we add an extra conditional ontop of the match, the if statement must also be true for the arm to match
                println!("The number {x} is even");
                assert!(true);
            }
            Some(x) => println!("The number {x} is odd"),
            None => println!("There is no number"),
        }

        let x = Some(5);
        let y = 5;

        match x {
            Some(50) => println!("Got 50"),
            Some(n) if n == y => println!("matched, n = {n}"), //<- using a match guard to bring a variable in from the outside scope to compare against
            //if we used y in our pattern it would have shadowed y, but since we used y in a match guard (which is not a pattern) we brought y in from the outer scope
            _ => assert!(false),
        }

        let x = 4;
        let y = false;

        match x {
            //in this case we are matching against 4, 5 or 6 and the if applies to all patterns, so
            //it could be though of as (4 | 5 | 6) if y =>
            //since y is false none will match
            4 | 5 | 6 if y => println!("yes"),
            _ => println!("no"),
        }
        assert!(true);
    }
    #[test]
    fn bindings() {
        enum Message {
            Hello { id: i32 },
        }

        let msg = Message::Hello { id: 5 };

        match msg {
            Message::Hello {
                id: id_variable @ 3..=7, //<- here we use the @ binding to match on a range but also capture the value we got from that range so
            } => println!("Found an id in range: {}", id_variable), //we can then use that value within the arm
            Message::Hello { id: 10..=12 } => {
                println!("Found an id in another range")
            }
            Message::Hello { id } => println!("Found some other id: {}", id),
        }
        assert!(true);
    }
}
//     _       _                               _   _____          _
//    / \   __| |_   ____ _ _ __   ___ ___  __| | |  ___|__  __ _| |_ _   _ _ __ ___  ___
//   / _ \ / _` \ \ / / _` | '_ \ / __/ _ \/ _` | | |_ / _ \/ _` | __| | | | '__/ _ \/ __|
//  / ___ \ (_| |\ V / (_| | | | | (_|  __/ (_| | |  _|  __/ (_| | |_| |_| | | |  __/\__ \
// /_/   \_\__,_| \_/ \__,_|_| |_|\___\___|\__,_| |_|  \___|\__,_|\__|\__,_|_|  \___||___/
mod ch19 {
    //-----------
    //Unsafe Rust
    //-----------
    #[test]
    fn unsafe_rust() {
        //Five things you can do in unsafe rust
        /*
            -Dereference a raw pointer
            -Call an unsafe function or method
            -Access or modify a mutable static variable
            -Implement an unsafe trait
            -Access fields of unions
        */

        //Dereferencing raw pointers
        //Raw pointers can be immutable or mutable and are written as
        let mut num = 5;

        let r1 = &num as *const i32; //<- you can create raw pointers outside of unsafe blocks, you just can't dereference them
        let r2 = &mut num as *mut i32; //<- we use as to cast references into their corresponding raw pointer types

        unsafe {
            println!("r1 is: {}", *r1); //<- since we are inside an unsafe block now, we can dereference our raw pointers
            println!("r2 is: {}", *r2);
            assert_eq!(*r1, *r2);
        }

        //writing to an arbitrary memory location
        let address = 0x012345usize;
        let r = address as *const i32;

        assert!(true);
    }
    #[test]
    fn calling_unsafe_functions() {
        //declare an unsafe function
        unsafe fn dangerous() {
            assert!(true);
        }

        //to call dangerous we need to invoke it within an unsafe block
        unsafe {
            dangerous();
        }
    }
    #[test]
    fn creating_safe_abstractions() {
        use std::slice;
        //implementing custom split_at_mut function
        fn my_split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
            let len = values.len();
            let ptr = values.as_mut_ptr(); //<- here we grab a pointer to the original slice we are gonna work with

            assert!(mid <= len); //<- assert our mid point doesn't exceed the length of the slice, thus accessing beyond the bounds of the slice

            //we cant mutably borrow the same slice twice
            //(&mut values[..mid], &mut values[mid..])

            //so we use unsafe and raw pointers to grab two non-overlapping parts of the same slice
            unsafe {
                (
                    slice::from_raw_parts_mut(ptr, mid),
                    slice::from_raw_parts_mut(ptr.add(mid), len - mid),
                )
            }
        }

        let mut v = vec![1, 2, 3, 4, 5, 6];
        let r = &mut v[..];

        let (a, b) = my_split_at_mut(r, 3); //<- note we don't have to call my_split_at_mut from an unsafe block, we've wrapped our unsafe code in a safe abstraction

        assert_eq!(a, &mut [1, 2, 3]);
        assert_eq!(b, &mut [4, 5, 6]);
    }
    #[test]
    fn calling_external_code() {
        extern "C" {
            //define which ABI we want to use, in this case C's
            fn abs(input: i32) -> i32; //<- list the name and signature of the external functions we want to call
        }

        unsafe {
            //invoke the external function from within an unsafe block since other languages don't make the same guarentees rust does
            println!("Absolute value of -3 according to C: {}", abs(-3));
        }
    }
    #[test]
    fn creating_external_code() {
        //we can also use the extern function to define our own external rust code for other langauges to call
        //use use the extern keyword along with #[no_mangle] annotation to tell rust to maintain the exact function name so other languages can use it
        #[no_mangle]
        pub extern "C" fn call_from_c() {
            println!("You can call this function from C!");
        }
        assert!(true);
    }
    #[test]
    fn mutable_static_variables() {
        static HELLO_WORLD: &str = "Hello, World!"; //<- common convention is to declare static variables in "Scream Snake Case"

        //Accessing and modifying static variables is unsafe
        static mut COUNTER: u32 = 0;

        fn add_to_count(inc: u32) {
            unsafe {
                COUNTER += inc;
            }
        }

        add_to_count(3); //<- the unsafe call to modify COUNTER is wrapped in a safe abstraction, so we can call this function without an unsafe block

        //accessing the COUNTER does require unsafe
        unsafe {
            println!("COUNTER: {}", COUNTER);
            assert_eq!(COUNTER, 3);
        }
        //This example is generally safe since its a single threaded use, but in multi threaded applications, accessing a single static variable with multiple threads could be unsafe
    }
    #[test]
    fn unsafe_trait() {
        unsafe trait Foo {
            //methods go here
        }

        unsafe impl Foo for i32 {
            //method implementation goes here
        }

        //Sync and Send are two examples of unsafe traits
        assert!(true);
    }
    //---------------
    //Advanced Traits
    //---------------
    #[test]
    fn placeholder_types_in_traits() {
        //iterator example with placeholder type
        pub trait MyIterator {
            type Item; //<- this is our placeholder type that the use has to specify in the impl

            fn next(&mut self) -> Option<Self::Item>; //<- next also returns our placeholder
        }

        struct Counter {
            count: u32,
        }

        impl Counter {
            fn new() -> Counter {
                Counter { count: 0 }
            }
        }

        impl MyIterator for Counter {
            type Item = u32; //<- in our impl of MyIterator we now define a concrete type that it will handle

            fn next(&mut self) -> Option<Self::Item> {
                Some(self.count)
            }
        }
        //Now for Counter we don't need to annotate a type everywhere like if we were using generic parameters
    }
    #[test]
    fn default_generic_params_and_operator_overloading() {
        //The default generic type within the add trait
        /*
            trait Add<Rhs=Self> { //<- here we use a default type parameter
                type Output; //<- along with a place holder

                fn add(self, rhs: Rhs) -> Self::Output;
            }
        */

        use std::ops::Add;

        #[derive(Debug, Copy, Clone, PartialEq)]
        struct Point {
            x: i32,
            y: i32,
        }

        impl Add for Point {
            type Output = Point;

            //since we used the default type parameter above, our other will default to the type of self
            fn add(self, other: Point) -> Point {
                Point {
                    x: self.x + other.x,
                    y: self.y + other.y,
                }
            }
        }

        assert_eq!(
            Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
            Point { x: 3, y: 3 }
        );

        //newtype pattern
        struct Millimeters(u32);
        struct Meters(u32);

        //rather than using the default parameter (self) as in the above example we specify a RHS parameter of Meters
        //so we can then handle an add between two different types (millimeters and meters)
        impl Add<Meters> for Millimeters {
            type Output = Millimeters;

            fn add(self, other: Meters) -> Millimeters {
                Millimeters(self.0 + (other.0 * 1000))
            }
        }
    }
    #[test]
    fn methods_w_same_name() {
        trait Pilot {
            fn fly(&self);
        }

        trait Wizard {
            fn fly(&self);
        }

        struct Human;

        //A fly func for Humans that impl Pilot Trait
        impl Pilot for Human {
            fn fly(&self) {
                println!("This is your captain speaking.");
            }
        }

        //A fly func for Humans that impl Wizard trait
        impl Wizard for Human {
            fn fly(&self) {
                println!("Up!");
            }
        }

        //A general fly function for all Human types
        impl Human {
            fn fly(&self) {
                println!("*waving arms furiously*");
            }
        }

        let person = Human;
        person.fly(); //<- calls Human::fly()
                      //We explicitly pass in the self part of the func for these fully qualified and disambiguous calls
        Pilot::fly(&person); //<- calls the Pilot impl of fly
        Wizard::fly(&person); //<- calls the Wizard impl of fly

        //if these impl functions didn't have a self parameter we could disambiguate using the following sytax
        <Human as Pilot>::fly(&person); //<- ignore the self being passed in this case
    }
    #[test]
    fn supertraits() {
        use std::fmt;

        //here we specify out OutlinePrint trait which relies on the fmt::Display trait (its supertrait)
        trait OutlinePrint: fmt::Display {
            fn outline_print(&self) {
                let output = self.to_string(); //<- requiring the Display trait allows us to use to_string in our new trait
                let len = output.len();
                println!("{}", "*".repeat(len + 4));
                println!("*{}*", " ".repeat(len + 2));
                println!("* {} *", output);
                println!("*{}*", " ".repeat(len + 2));
                println!("{}", "*".repeat(len + 4));
            }
        }

        struct Point {
            x: i32,
            y: i32,
        }

        //If we simply try to implement OutlinePrint for Point we will get an error since Point doesn't implement Display
        impl OutlinePrint for Point {}

        //To fix the issues we implement Display on Point
        impl fmt::Display for Point {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "({}, {})", self.x, self.y)
            }
        }

        let p = Point { x: 1, y: 2 };
        p.outline_print(); //<- no outline_print will work with our Point type
    }
    #[test]
    fn external_traits_on_external_types() {
        use std::fmt;

        struct Wrapper(Vec<String>); //<-simple tuple struct wrapper for Vec<String>

        //using the Wrapper type to implement Display on the underlying Vec<String>
        impl fmt::Display for Wrapper {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "[{}]", self.0.join(", "))
            }
        }

        //now we can use the display trait on our wrapped Vec<String>
        let w = Wrapper(vec![String::from("Hello, "), String::from("World!")]);
        println!("w = {w}");
    }
    //--------------
    //Advanced Types
    //--------------
    #[test]
    fn type_aliases() {
        type Kilometers = i32;

        //type aliases are synonyms for types, not a seperate new type
        let x: i32 = 5;
        let y: Kilometers = 5;
        //because of this we can use Kilometers in place of i32 and even pass Kilometers in functions that take i32 parameters
        //this feature is mainly used for cutting down on lenghty type declarations, for example:
        type BoxThing = Box<dyn Fn() + Send + 'static>;
    }
    #[test]
    fn never_type() {
        //rust has a special type named '!', often called the 'never type'
        //it represents a function that will never return
        fn bar() -> ! {
            loop { /* do something here that never returns */ }
        }
    }
    //-------------------------------
    //Advanced Functions and Closures
    //-------------------------------
    #[test]
    fn function_pointers() {
        //define our function
        //the signature is fn(i32) -> i32
        fn add_one(x: i32) -> i32 {
            x + 1
        }

        //in our do_twice function we will take a parameter that matches our add_one signature
        //then we invoke the function within this function and add those two results together
        fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
            f(arg) + f(arg)
        }

        let answer = do_twice(add_one, 5);
        println!("the answer is {answer}");
        assert_eq!(answer, 12);
    }
    #[test]
    fn returning_closures() {
        //we cant return a function directly because Rust doesn't know how much space it will need to store the closure
        // fn returns_closure() -> dyn Fn(i32) -> i32 {
        //     |x| x + 1
        // }

        //but we can make it work using Box<dyn>
        fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
            Box::new(|x| x + 1)
        }
    }
    //------
    //Macros
    //------
    #[test]
    fn simplified_vec_macro() {
        //macro_export indicates the macro should be made available anywhere this crate is brought into scope
        #[macro_export]
        macro_rules! myvec { //<- we start the definition with macro_rules! and our macro name without the exclamation mark
            ($($x:expr), *) => { //<- this is essentially a pattern we match against and the arm we execute if there is a match (more complex macros will have multiple arms)
            //pattern explaination: first parentheses encompass the whole pattern, $() captures the pattern in the first set of parentheses and passes it to the second set of parentheses
            // $x:expr matches any Rust expression and give it the name $x, the comma following $() specifies that the pattern matches zero or more times
            //so when we call myvec![1, 2, 3] the $x pattern matches 3 times with expressions 1, 2 & 3
            {
                //here is the code that is executed upon pattern match
                //we create out new vec
                let mut temp_vec = Vec::new();
                //then for each match above we generate a line of temp_vec.push($x); subbing in the capture values in place of our $x variable
                $(
                    temp_vec.push($x);
                )*
                //if we pass in [1, 2, 3] the above code will expand to
                /*
                temp_vec.push(1);
                temp_vec.push(2)
                temp_vec.push(3)
                */

                //then we return out vec
                temp_vec
            }
            };
        }

        let v = myvec![1, 2, 3];
        let mut v2 = Vec::new();
        v2.push(1);
        v2.push(2);
        v2.push(3);
        assert_eq!(v, v2);
    }
    #[test]
    fn procedural_macros() {
        //see hello_macro and macro_main crate
        assert!(true);
    }
}
// _____ _             _   ____            _           _
// |  ___(_)_ __   __ _| | |  _ \ _ __ ___ (_) ___  ___| |_
// | |_  | | '_ \ / _` | | | |_) | '__/ _ \| |/ _ \/ __| __|
// |  _| | | | | | (_| | | |  __/| | | (_) | |  __/ (__| |_
// |_|   |_|_| |_|\__,_|_| |_|   |_|  \___// |\___|\___|\__|
//                                       |__/
mod ch20 {
    //see web_server project code
    #[test]
    fn final_project() {
        assert!(true);
    }
}
