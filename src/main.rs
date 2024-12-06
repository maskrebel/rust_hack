use std::collections::HashMap;

mod fizz_buzz;
mod balanced_breaker;

fn main() {
    println!("Hello, world!");
    let map_balance = [('}', '{'), (']', '['), (')', '(')];
    let map_bracket = HashMap::from([('}', '{'), (']', '['), (')', '(')]);
    let mut map = HashMap::from([
        ('d'.to_string(), 1),
        ("abd".to_string(), 2),
    ]);

    // get or default value map
    if let Some(val) = map_bracket.get(&'}') {
        println!("Nilai untuk 'b' adalah: {}", val);
    } else {
        println!("key not found")
    }
    println!("{:?}", map_balance);
}
