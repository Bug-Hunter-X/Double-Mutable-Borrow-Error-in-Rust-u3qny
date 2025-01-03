fn main() {
    let mut x = 5;
    let y = &mut x; // y is a mutable reference to x
    let z = &mut x; // z is also a mutable reference to x
    *y = 10;
    *z = 15;
    println!("x = {}", x); // This line will cause a compile-time error
}