use std::fs;
use std::time::Instant;

fn main() {
    let input = fs::read_to_string("day11.txt").expect("File nout found");

    let mut vec: Vec<String> = input.trim().split(' ').map(|x| x.to_string()).collect();
    println!("{:?}", vec);

    let start = Instant::now();

    for index in 0..25 {
        println!("Iteration {}", index);
        vec = vec
            .iter()
            .flat_map(|x| {
                if *x == "0" {
                    vec!["1".to_string()]
                } else if x.len() % 2 == 0 {
                    let a = &x[0..(x.len() / 2)].trim_start_matches('0');
                    let b = &x[(x.len() / 2)..].trim_start_matches('0');
                    vec![
                        if a.is_empty() {
                            "0".to_string()
                        } else {
                            a.to_string()
                        },
                        if b.is_empty() {
                            "0".to_string()
                        } else {
                            b.to_string()
                        },
                    ]

                    //vec![x[0..(x.len() / 2)], x[x.len() / 2..]]
                } else {
                    let temp = x.parse::<usize>().unwrap() * 2024;
                    vec![temp.to_string()]
                }
            })
            .collect();

        //println!("{:?}", vec);
    }

    let end = start.elapsed();

    println!("{:?} {:?}", vec.len(), end);
}
