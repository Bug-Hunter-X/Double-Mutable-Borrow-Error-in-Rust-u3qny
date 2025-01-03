fn main() {
    let mut x = 5;
    { // Create a new scope to limit the lifetime of the mutable borrow 
        let y = &mut x; // y is a mutable reference to x
        *y = 10;
        println!("x after y = {}", x);
    }

    { // Another scope for the second mutable reference
        let z = &mut x; // z is a mutable reference to x
        *z = 15;
        println!("x after z = {}", x); 
    }
    println!("x = {}", x);
}