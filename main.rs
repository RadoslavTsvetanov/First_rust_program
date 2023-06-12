fn main() {
    let mut a: i32 = 0;
    let rust_better = ["like python", "fast", "the compiler is your friens"];
    for i in 0..1000 {
        a += 1;
    }
    for i in rust_better {
        println!("{}", i);
    }
    println!("Rust is {} times better than C and C++", a);
}
