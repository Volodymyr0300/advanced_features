fn main() {
    // Unsafe rust
    // Dereferencing a Raw Pointer
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
        unsafe trait Foo {
            // methods go here
        }
        unsafe impl Foo for i32 {
            // method implementations go here
        }
    }
    // advanced traits
    
}
