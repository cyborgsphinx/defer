use defer::{defer, Defer};

fn main() {
    let _later = Defer::new(|| println!("later"));
    let _named = defer!(println!("deferred"));
    let mut value = 0;
    defer!(value += 1);
    let _other = defer!(value += 1; println!("{}", value));
    defer!(println!("early"));
    println!("immediate");
}
