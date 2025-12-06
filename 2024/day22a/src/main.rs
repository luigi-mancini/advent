use anyhow::Result;
use std::fs::File;
use std::io::{self, BufRead};

type SeedList = Vec<usize>;

struct SecretNumber {
    secret: usize,
}

impl SecretNumber {
    fn new(secret: usize) -> Self {
       SecretNumber{ secret }
    }

    fn next_secret(&mut self) -> usize {
       let tmp = self.secret * 64;
       self.mix_and_prune(tmp);

       let tmp = self.secret / 32;
       self.mix_and_prune(tmp);

       let tmp = self.secret * 2048;
       self.mix_and_prune(tmp);

       self.secret
    }

    fn mix_and_prune(&mut self, val: usize) {
       self.secret ^= val;
       self.secret %= 16777216;
    }


}

fn read_input(path: &str) -> Result<SeedList> {
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let sl = reader
        .lines()
        .map(|x| {
            let line = x?;
	    let number = line.parse::<usize>()?;
	    Ok(number)
        })
        .collect::<Result<SeedList>>()?;

    Ok(sl)
}

fn main() -> Result<()> {
   let mut total = 0;
   let seeds = read_input("day22.txt")?;
   for s in seeds {
      let mut sn = SecretNumber::new(s);
      for _ in 0..2000 {
          sn.next_secret();
      }
      total += sn.secret;
      println!("{}", sn.secret);

   }
   println!("Total {}", total);

   Ok(())
}
