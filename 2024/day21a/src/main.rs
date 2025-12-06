mod coordinates;
mod keypad;

use anyhow::Result;
use keypad::Keypad;

fn get_score(nk: &Keypad, dk: &Keypad, code: &String) -> usize {
    let tmp = nk.decode('A', code);
    let tmp2 = dk.decode_vec('A', &tmp);

    let mut c = code.clone();
    let _ = c.pop();
    let val = c.parse::<usize>().unwrap_or_default();

    println!("Val {}", val);

    val * dk.decode_len('A', &tmp2, 1)
}

fn main() -> Result<()> {
    let nk_vec = vec![
        vec![Some('7'), Some('8'), Some('9')],
        vec![Some('4'), Some('5'), Some('6')],
        vec![Some('1'), Some('2'), Some('3')],
        vec![None, Some('0'), Some('A')],
    ];
    let nk = Keypad::new(nk_vec);

    let dk_vec = vec![
        vec![None, Some('^'), Some('A')],
        vec![Some('<'), Some('v'), Some('>')],
    ];
    let dk = Keypad::new(dk_vec);

    let codes = vec![
        String::from("029A"),
        String::from("980A"),
        String::from("179A"),
        String::from("456A"),
        String::from("379A"),
    ];

    let mut score = 0;
    for c in codes.iter() {
        score += get_score(&nk, &dk, c);
    }

    println!("{:?}", score);

    let codes = vec![
        String::from("208A"),
        String::from("540A"),
        String::from("685A"),
        String::from("879A"),
        String::from("826A"),
    ];

    let mut score = 0;
    for c in codes.iter() {
        score += get_score(&nk, &dk, c);
    }

    println!("{:?}", score);

    Ok(())
}
