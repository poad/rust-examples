fn main() {
    use std::collections::HashMap;

    let fizz_buzz: HashMap<i32, &str> = [
        (3, "Fizz"),
        (5, "Buzz")
     ].iter().cloned().collect();

    for n in 1..=100 {
        let n_str = n.to_string();
        let result = fizz_buzz.iter()
            .filter(|(&k, _)| n % k == 0)
            .map(|(_, v)| v)
            .next()
            .map_or(n_str.as_str(), |v| v);
        println!("{}", result);
    }
}
