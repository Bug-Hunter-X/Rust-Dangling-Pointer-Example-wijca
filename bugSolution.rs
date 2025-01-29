fn main() {
    let mut v = vec![1, 2, 3];
    let mut ptr = v.as_mut_ptr();
    let len = v.len();
    unsafe {
        for i in 0..len {
            *ptr.add(i) = 100 * (i + 1);
        }
    }
    println!("{:?}", v);
}