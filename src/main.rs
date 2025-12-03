fn day01() {
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
fn main() {
    day01();
    let input: Vec<(i64, i64)> = std::fs::read_to_string("inputs/day02.txt")
        .unwrap()
        .split(',')
        .map(|range| {
            let mut ids = range.trim().split('-');
            let a = ids.next().unwrap().parse().unwrap();
            let b = ids.next().unwrap().parse().unwrap();
            (a, b)
        })
        .collect();
    let mut invalid = 0;
    for (start, end) in input {
        for num in start..end+1 {
            let curr = num.to_string();
            let n = curr.len();
            for size in 1..=n / 2 {
                if n % size != 0 {
                    continue;
                }
                let part = &curr[..size];

                let mut ok = true;
                for chunk in curr.as_bytes().chunks(size) {
                    if chunk != part.as_bytes() {
                        ok = false;
                        break;
                    }
                }

                if ok {
                    invalid += num;
                    break;
                }
            }
        }
    }

    println!("invalid total: {}", invalid);
}
