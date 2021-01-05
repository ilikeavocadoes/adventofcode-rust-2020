use std::fs;

pub fn run() {
    let input = fs::read_to_string("input/day13.input").unwrap();
    print!("{}", solve_crt(parse_bus_times(&input)));
}

fn ex() -> &'static str {
    "53\n17,x,13,19"
}

fn parse_bus_times(times: &str) -> Vec<BusTime> {
    let mut v = Vec::new();
    let numbers = times.lines().nth(1).unwrap().split(',');
    for (s, i) in numbers.zip(0..) {
        let number = match s.trim().parse::<i64>() {
            Ok(n) => n,
            Err(_) => continue,
        };
        let bus_time = BusTime {
            time_offset: i,
            bus_number: number,
        };
        v.push(bus_time);
    }
    v
}

fn solve_crt(times: Vec<BusTime>) -> i64 {
    let n = times.iter().fold(1, |acc, b| acc * b.bus_number);
    let ys: Vec<i64> = times.iter().map(|b| n / b.bus_number).collect();
    let zs = times
        .iter()
        .zip(ys.iter())
        .map(|(b, y)| euclidian_inverse(*y, b.bus_number));
    let x = times
        .iter()
        .zip(ys.iter())
        .zip(zs)
        .map(|((b, y), z)| (b.bus_number - b.time_offset) * y * z)
        .fold(0, |acc, m| acc + m);
    x % n
}

fn euclidian_inverse(a: i64, n: i64) -> i64 {
    let mut t = 0;
    let mut newt = 1;
    let mut r = n;
    let mut newr = a;

    while newr != 0 {
        let quotient = r / newr;
        let prevt = t;
        t = newt;
        newt = prevt - quotient * t;
        let prevr = r;
        r = newr;
        newr = prevr - quotient * r;
    }
    if r > 1 {
        panic!(format!("{} is not invertible", a));
    }
    if t < 0 {
        t = t + n;
    }
    t
}

#[derive(Debug)]
struct BusTime {
    time_offset: i64,
    bus_number: i64,
}
