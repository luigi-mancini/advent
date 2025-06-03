mod coordinates;
mod keypad;

use anyhow::Result;
use keypad::Keypad;

fn main() -> Result<()> {
    let nk_vec = vec![
        vec![Some('7'), Some('8'), Some('9')],
        vec![Some('4'), Some('5'), Some('6')],
        vec![Some('1'), Some('2'), Some('3')],
        vec![None, Some('0'), Some('A')],
    ];

    let nk = Keypad::new(nk_vec);

    println!("{:?}", nk);
    Ok(())
}
