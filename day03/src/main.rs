use std::io::*;
use std::collections::{HashSet, HashMap};

#[derive(Copy,Clone,Debug,PartialEq,Eq,Hash)]
struct Point {
    x: i64,
    y: i64
}

fn main() {
    let s = stdin();
    let lines = s.lock().lines().filter_map(|line| line.ok());

    let mut point_sets = Vec::new();
    let mut point_vecs = Vec::new();
    for line in lines {
        let mut point_set = HashSet::new();
        let mut point_vec = Vec::new();
        let mut p = Point{x: 0, y: 0};
        for cmd in line.split(',') {
            let (d, n) = cmd.split_at(1);
            let n: i64 = n.parse().unwrap();
            let func: fn(Point) -> Point = match d { 
                "R" => |p| Point{x: p.x + 1, y: p.y    },
                "U" => |p| Point{x: p.x,     y: p.y + 1},
                "L" => |p| Point{x: p.x - 1, y: p.y    },
                "D" => |p| Point{x: p.x,     y: p.y - 1},
                _ => panic!()
            };
            for _ in 0..n {
                p = func(p);
                point_set.insert(p);
                point_vec.push(p);
            }
        }
        point_sets.push(point_set);
        point_vecs.push(point_vec);
    }

    let crossover_points: Vec<Point> = point_sets[0]
        .intersection(&point_sets[1])
        .cloned()
        .collect();

    let closest_point = crossover_points.iter()
        .map(|p| p.x.abs() + p.y.abs())
        .min();

    println!("closest = {:?}", closest_point);

    let h1: HashMap<_, _> = point_vecs[0].iter().enumerate().map(|(i, p)| (p, i+1)).collect();
    let h2: HashMap<_, _> = point_vecs[1].iter().enumerate().map(|(i, p)| (p, i+1)).collect();

    let shortest_path = crossover_points.iter().map(|p| h1[p] + h2[p]).min();

    println!("shortest = {:?}", shortest_path);
}
