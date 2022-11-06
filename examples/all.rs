use defer::{defer, Defer};

fn main() {
    let _later = Defer::new(|| println!("later"));
    defer!(println!("deferred"));
    let mut value = 0;
    defer!(value += 1; println!("{}", value));
    println!("immediate");
}
