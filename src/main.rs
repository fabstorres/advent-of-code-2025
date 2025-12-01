fn main() {
    let input = std::fs::read_to_string("inputs/day01-part01.txt").unwrap();
    let arr: Vec<i32> = input
        .split('\n')
        .map(|s| {
            if s.starts_with('L') {
                -s[1..].parse::<i32>().unwrap()
            } else if s.starts_with('R') {
                s[1..].parse::<i32>().unwrap()
            } else {
                0
            }
        })
        .collect();

    let mut total = vec![50];
    let mut crosses = 0;

    for num in &arr {
        let prev = *total.last().unwrap();
        let delta = *num;

        crosses += delta.abs() / 100;

        let leftover = delta % 100;
        if leftover != 0 {
            if leftover > 0 {
                if prev + leftover >= 100 {
                    crosses += 1;
                }
            } else {
                if prev + leftover < 0 {
                    crosses += 1;
                }
            }
        }

        let next = (prev + delta).rem_euclid(100);
        total.push(next);
    }

    println!(
        "Password zeros: {}",
        total.iter().filter(|n| **n == 0).count()
    );
    println!("Crosses: {}", crosses);
}
