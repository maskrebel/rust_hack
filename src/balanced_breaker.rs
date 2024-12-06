fn is_balanced(s: &str) -> String {
    let map_balance = [('}', '{'), (']', '['), (')', '(')];
    let mut tmp_bracket = Vec::new();

    for c in s.chars() {
        if let Some(spouse) = map_balance.iter().find_map(|&(closing, opening)| {
            if c == closing { Some(opening) } else { None }
        }) {
            if tmp_bracket.is_empty() || *tmp_bracket.last().unwrap() != spouse {
                return "NO".to_string();
            } else {
                tmp_bracket.pop();
            }
        } else {
            tmp_bracket.push(c);
        }
    }

    if !tmp_bracket.is_empty() {
        return "NO".to_string();
    }

    "YES".to_string()
}

#[test]
fn main() {
    let s = "[()][{}()][](){}([{}(())([[{}]])][])[]([][])(){}{{}{[](){}}}()[]({})[{}{{}([{}][])}}";
    println!("{}", is_balanced(&s));
}