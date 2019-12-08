use std::collections::*;
use std::io::*;

fn main() {
    let orbits: HashMap<_, _> = stdin()
        .lock()
        .lines()
        .filter_map(Result::ok)
        .map(|l| { 
            let mut s = l.split(')');
            (s.next().unwrap().to_string(), s.next().unwrap().to_string())
        })
        .map(|(a, b)| (b, a))
        .collect();

    let total_orbits: u64 = orbits
        .keys()
        .map(|mut s| {
            let mut count = 0u64;
            while let Some(next) = orbits.get(s) {
                count += 1;
                s = next;
            }
            count
        })
        .sum();

    let make_vec = |s: &str| {
        let mut s2 = s;
        let mut v = Vec::new();
        while let Some(next) = orbits.get(s2) {
            v.push(next);
            s2 = next;
        }
        v
    };

    let you_orbits: HashSet<_> = make_vec("YOU").into_iter().collect();
    let san_orbits: HashSet<_> = make_vec("SAN").into_iter().collect();

    let diff = you_orbits.symmetric_difference(&san_orbits);

    println!("{:?}", total_orbits);
    println!("{:?}", diff.count());
}
