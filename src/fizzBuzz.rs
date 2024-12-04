fn fizzBuzz(n: i32) {
    for i in 1..=n {
        if i % 3 == 0 && i % 5 == 0 {
            println!("FizzBuzz");
        } else if i % 3 == 0 {
            println!("Fizz");
        } else if i % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", i);
        }
    }
}

#[test]
fn main() {
    let n: i32 = 10;
    fizzBuzz(n);
}