fn main() {
    // ! Unsafe rust
    // dereferencing a Raw Pointer
    {
        let mut num = 5;
    
        let r1 = &raw const num;
        let r2 = &raw mut num;
    
        unsafe {
            println!("r1 is: {} and r2 is: {}", *r1, *r2);
        }
    }
    // unsafe functions and methods
    {
        unsafe fn dangerous() {
            println!("This is a dangerous function!");
        }
        unsafe {
            dangerous();
        }
    }
    {
    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    println!("a: {:?}, b: {:?}", a, b);
    }
    {
    use std::slice;

    fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
        let len = values.len();
        let ptr = values.as_mut_ptr();

        assert!(mid <= len);

        unsafe {
            (
                slice::from_raw_parts_mut(ptr, mid),
                slice::from_raw_parts_mut(ptr.add(mid), len - mid),
            )
        }
    }
    
    let mut vector = vec![1, 2, 3, 4, 5, 6];
    let (left, right) = split_at_mut(&mut vector, 3);
    println!("left: {:?}, right: {:?}", left, right);
    }
    {
    use std::slice;

    let address = 0x01234usize;
    let r = address as *mut i32;
    println!("r: {:?}", r);

    let _values: &[i32] = unsafe { slice::from_raw_parts_mut(r, 10000) };
    // println!("values: {:?}", values);
    }
    {
        unsafe extern "C" {
            safe fn abs(input: i32) -> i32;
        }
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
    // accessing or modifying a mut static variable
    {
    #[unsafe(no_mangle)]
    pub extern "C" fn call_from_c() {
        println!("Just called a Rust function from C!");
    }
    call_from_c();
    }
    {
        static HELLO_WORLD: &str = "Hello, world!";
        println!("value is: {HELLO_WORLD}");
    }
    {
        static mut COUNTER: u32 = 0;

        unsafe fn add_to_count(inc: u32) {
            unsafe {
                COUNTER += inc;
            }
        }
        unsafe {
            add_to_count(3);
            println!("COUNTER: {}", *(&raw const COUNTER));
        }
    }
    // impl unsafe trait
    {
        unsafe trait _Foo {
            // methods go here
        }
        unsafe impl _Foo for i32 {
            // method implementations go here
        }
    }
    // ! Advanced traits
    // defining traits with associated types
    {
        trait _Iterator {
            type Item;

            fn next(&mut self) -> Option<Self::Item>;
        }
    }
    {
        #[derive(Debug)]
        struct Counter {
            count: u32,
        }

        impl Counter {
            fn new() -> Counter {
                Counter { count: 0 }
            }
        }

        // pub trait Iterator<T> {
        //     fn next(&mut self) -> Option<T>;
        // }

        impl Iterator for Counter {
            type Item = u32;

            fn next(&mut self) -> Option<Self::Item> {
                // --snip--
                if self.count < 5 {
                    self.count += 1;
                    Some(self.count)
                } else {
                    None
                }
            }
        }
        println!("Counter created: {:?}", Counter::new());
    }
    // using default generic parameters and operator overloading
    {
        use std::ops::Add;

        #[derive(Debug, Copy, Clone, PartialEq)]
        struct Point {
            x: i32,
            y: i32,
        }

        impl Add for Point {
            type Output = Point;

            fn add(self, other: Point) -> Point {
                Point { 
                    x: self.x + other.x, 
                    y: self.y + other.y 
                }
            }
        }
        
        assert_eq!(
            Point {x: 1, y: 0} + Point {x: 2, y: 3},
            Point {x: 3, y: 3}
        );
        println!("Point addition: {:?}", Point {x: 1, y: 0} + Point {x: 2, y: 3});
        
    }
    {
        #![allow(unused)]
        trait Add<Rhs=Self> {
            type Output;
            fn add(self, rhs: Rhs) -> Self::Output;
        }   
    }
    {
        use std::ops::Add;

        #[derive(Debug)]
        struct Millimeters(u32);
        struct Meters(u32);

        impl Add<Meters> for Millimeters {
            type Output = Millimeters;

            fn add(self, other: Meters) -> Millimeters {
                Millimeters(self.0 + (other.0 * 1000))
            }
        }
        println!("Millimeters + Meters = {:?}", Millimeters(500) + Meters(2));
    }
    // disambiguating between identically named methods
    {
        trait Pilot {
            fn fly(&self);
        }

        trait Wizard {
            fn fly(&self);
        }

        struct Human;

        impl Pilot for Human {
            fn fly(&self) {
                println!("This is your captain speaking.");
            }
        }

        impl Wizard for Human {
            fn fly(&self) {
                println!("Up!");
            }
        }

        impl Human {
            fn fly(&self) {
                println!("*waving arms furiously*");
            }
        }

        let person = Human;
        person.fly();
        Pilot::fly(&person);
        Wizard::fly(&person);
    }
    {
        trait Animal {
            fn baby_name() -> String;
        }

        struct Dog;

        impl Dog {
            fn _baby_name() -> String {
                String::from("Spot")
            }
        }

        impl Animal for Dog {
            fn baby_name() -> String {
                String::from("puppy")
            }
        }

        println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
    }
    // using supertraits
    {
        
        trait OutlinePrint: fmt::Display {
            fn outline_print(&self) {
                let output = self.to_string();
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
        
        impl OutlinePrint for Point {}
        
        use std::fmt;

        impl fmt::Display for Point {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "({}, {})", self.x, self.y)
            }
        }

        let p = Point {x: 1, y: 3};
        p.outline_print();
    }
    // implementing external traits with the newtype patern
    {
        use std::fmt;

        struct Wrapper(Vec<String>);

        impl fmt::Display for Wrapper {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "[{}]", self.0.join(", "))
            }
        }

        let w = Wrapper(vec![String::from("hello"), String::from("world")]);
        println!("w = {w}");
    }
    {}
    // ! Advanced types
    // type synonyms and types aliases
    {
        type Kilometers = i32;

        let x: i32 = 5;
        let y: Kilometers = 5;

        println!("x + y = {}", x + y);
    }
    {
        let _f: Box<dyn Fn() + Send + 'static> = Box::new(|| println!("hi")); // type annotation is necessary
        println!("f created.");

        fn _takes_long_type(_f: Box<dyn Fn() + Send + 'static>) { // type annotation is necessary
            // --snip--
        }
        println!("calling takes_long_type...");

        fn _returns_long_type() -> Box<dyn Fn() + Send + 'static> { // type annotation is necessary
            // --snip--
            Box::new(|| ()) // placeholder implementation
        }
        println!("calling returns_long_type...");
    }
    {
        type Thunk = Box<dyn Fn() + Send + 'static>; // type alias definition

        let f: Thunk = Box::new(|| println!("hi")); // using the type alias

        fn _takes_long_type(f: Thunk) {
            // --snip--
        }

        /*
        fn returns_long_type() -> Thunk {
            // --snip--
        }
        */
    }
    {
        use std::fmt;
        
        type Result<T> = std::result::Result<T, Error>;

        pub trait Write {
            fn write(&mut self, buf: &[u8]) -> Result<usize>;
            fn flush(&mut self) -> Result<usize>;

            fn write_all(&mut self, buf: &[u8]) -> Result<()>;
            fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<()>;
        }
    }
}
