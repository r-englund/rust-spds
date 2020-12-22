use tracy_rs;
use tracy_rs::*;

fn asdf(i: usize) -> usize {
    profile_scope!("asdf");
    i * i
}

fn main() {
    unsafe {
        tracy_rs::load("c:/temp/tracy/bin/TracyProfiler.dll");
    }
    tracy_begin_frame!("frame");
    let mut sum: usize = 0;
    for i in 0..usize::MAX {        
        sum = sum.overflowing_add(asdf(i)).0;
    }
    tracy_end_frame!("frame");

    println!("{}", sum)
}
