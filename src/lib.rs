use std::slice;

pub fn unsafe_test1() {
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
        *r2 = 10;
        println!("new r1 is: {}", *r1);
        println!("new r2 is: {}", *r2);
    }

    // let address = 0x012345usize;
    // let r = address as *const i32;

    // unsafe { // 访问非法内存地址，会报错
    //     println!("r is: {}", *r);
    //     println!("r is: {}", *r as i32);
    // }
}

fn split_at_mut<T>(slc: &mut [T], mid: usize) -> (&mut [T], &mut [T]) {
    let len = slc.len();
    let ptr = slc.as_mut_ptr();
    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

pub fn unsafe_test2() {
    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    let (a, b) = split_at_mut(r, 3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    a.reverse();

    assert_eq!(a, &mut [3, 2, 1]);
    assert_eq!(b, &mut [4, 5, 6]);
    println!("end");
}

pub fn unsafe_test3() {
    let s: &str = "123";
    let ptr: *const u8 = s.as_ptr();

    unsafe {
        println!("{}", *ptr as char);
        println!("{}", *ptr.add(1) as char);
        println!("{}", *ptr.add(2) as char);
        println!("{}", *ptr.add(3) as char);
        println!("{}", *ptr.add(4) as char);
        println!("{}", *ptr.add(5) as char);
        println!("{}", *ptr.add(6) as char);
        println!("{}", *ptr.add(7) as char);
        println!("{}", *ptr.add(8) as char);
        println!("{}", *ptr.add(9) as char);
    }
}

extern "C" {
    fn abs(input: i32) -> i32;
}

pub fn unsafe_test4() {
    unsafe {
        println!("abs(-3) = {}", abs(-3));
    }
}

static mut COUNTER: i32 = 0;

fn add_to_count(inc: i32) {
    unsafe {
        COUNTER += inc;
    }
}

pub fn unsafe_test5() {
    add_to_count(10);
    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}

unsafe trait Foo {
    fn plus_plus(&mut self);
}

unsafe impl Foo for i32 {
    fn plus_plus(&mut self) {
        *self += 1;
    }
}

pub fn unsafe_test6() {
    let mut x = 5;
    let y = &mut x as *mut i32 as *mut dyn Foo;
    unsafe {
        (*y).plus_plus();
        println!("x is now: {}", x);
    }
}
