extern crate libc;


/// Use this to load the standard library interface to threading.
use std::thread;


#[no_mangle]
pub extern fn double(x: i32) -> i32{
    x * 2
}

#[no_mangle]
pub extern fn process(threads: i32) {
    let handles: Vec<_> = (0..threads).map(|_| {
        thread::spawn(|| {
            let mut x = 0;
            for _ in 0..2_000_000_000 {
                x += 1;
            }
            x
        })
    }).collect();

    for h in handles {
        println!("Thread finished with count={}",
        h.join().map_err(|_| "Could not join a thread!").unwrap());
    }
}

#[test]
fn it_works(){
    assert!(double(3) == 6);
}