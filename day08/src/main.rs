use std::io::{self, BufRead};

fn main() {
    // Part 1
    //if let Some(Ok(line)) = io::stdin().lock().lines().next() {
    //    let (layer, _zeros) = (0..)
    //        .take_while(|n| line.chars().count() > n*150)
    //        .map(|n| {
    //            (n, line
    //                .chars()
    //                .skip(n*150)
    //                .take(150)
    //                .filter(|x| *x == '0')
    //                .count())
    //        })
    //        .min_by_key(|&(_, b)| b)
    //        .unwrap();

    //    let ones = line.chars().skip(layer*150).take(150).filter(|x| *x == '1').count();
    //    let twos = line.chars().skip(layer*150).take(150).filter(|x| *x == '2').count();

    //    println!("{:?}", ones * twos);
    //}

    // Part 2
    if let Some(Ok(line)) = io::stdin().lock().lines().next() {
        let mut image = [' '; 150];
        let layers = line.chars().count() / 150;
        for layer in (0..layers).rev() {
            let image_iter = image.iter_mut();
            let layer_iter = line.chars().skip(layer*150).take(150);
            for (a, b) in image_iter.zip(layer_iter) {
                match b {
                    '0' => *a = ' ',
                    '1' => *a = 'X',
                    _ => {}
                }
            }
        }
        for row in image.chunks(25) {
            let s: String = row.iter().collect();
            println!("{}", s);
        }
    }
}
