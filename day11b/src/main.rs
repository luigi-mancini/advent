use std::collections::HashMap;
use std::fs;
use std::time::Instant;

type Key = (usize, String);

fn get_score(ttl: usize, val: &str, cache: &mut HashMap<Key, usize>) -> usize {
    let key = (ttl - 1, val.to_string());

    if let Some(score) = cache.get(&key) {
        *score
    } else {
        let temp = blink2(ttl - 1, val, cache);
        cache.insert(key, temp);
        temp
    }
}

fn blink2(ttl: usize, val: &str, cache: &mut HashMap<Key, usize>) -> usize {
    if ttl == 0 {
        return 1;
    }

    if val == "0" {
        get_score(ttl, "1", cache)
    } else if val.len() % 2 == 0 {
        let mut a = val[0..(val.len() / 2)].trim_start_matches('0');
        let mut b = val[(val.len() / 2)..].trim_start_matches('0');

        a = if a.is_empty() { "0" } else { a };
        b = if b.is_empty() { "0" } else { b };

        get_score(ttl, a, cache) + get_score(ttl, b, cache)
    } else {
        let temp = val.parse::<usize>().unwrap() * 2024;
        get_score(ttl, &(temp.to_string()), cache)
    }
}

fn main() {
    let input = fs::read_to_string("day11.txt").expect("File nout found");

    let mut cache: HashMap<(usize, String), usize> = HashMap::new();

    let mut vec: Vec<(usize, String)> = input
        .trim()
        .split(' ')
        .map(|x| (76, x.to_string()))
        .collect();
    println!("{:?}", vec);

    let start = Instant::now();

    let mut count: usize = 0;
    while let Some((ttl, val)) = vec.pop() {
        count += blink2(ttl - 1, &val, &mut cache);
    }

    let end = start.elapsed();

    println!("MY COUNT IS {:?} in  {:?}", count, end);
}
