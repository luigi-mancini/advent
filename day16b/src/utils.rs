use crossterm::{cursor, terminal, ExecutableCommand};
use std::io::{stdout, Write};

pub fn print_grid(vec: &[Vec<char>]) {
    let mut stdout = stdout();
    stdout
        .execute(terminal::Clear(terminal::ClearType::All))
        .unwrap();

    stdout.execute(cursor::MoveTo(0, 0)).unwrap();

    for row in vec.iter() {
        let s: String = row.iter().collect();
        //println!("{}", s);
        writeln!(stdout, "{}", s).unwrap();
    }

    stdout.flush().unwrap();
}

pub fn print_grid_opt(vec: &[Vec<Option<usize>>]) {
    let mut stdout = stdout();
    stdout
        .execute(terminal::Clear(terminal::ClearType::All))
        .unwrap();

    stdout.execute(cursor::MoveTo(0, 0)).unwrap();

    for row in vec.iter() {
        for entry in row {
            if let Some(val) = entry {
                write!(stdout, "{:05} ", val).unwrap();
            } else {
                write!(stdout, "##### ").unwrap();
            }
        }

        writeln!(stdout).unwrap();
    }

    stdout.flush().unwrap();
}
