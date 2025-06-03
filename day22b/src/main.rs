use anyhow::Result;
use std::fs::File;
use std::io::{self, BufRead};
use std::collections::{HashSet, HashMap};
use std::time::Instant;

type SeedList = Vec<usize>;

struct SecretNumber {
    secret: usize,
}

impl SecretNumber {
    fn new(secret: usize) -> Self {
       SecretNumber{ secret }
    }

    fn next_secret(&mut self, pl: &mut PriceList) -> usize {
       let current = self.secret;
    
       let tmp = self.secret * 64;
       self.mix_and_prune(tmp);

       let tmp = self.secret / 32;
       self.mix_and_prune(tmp);

       let tmp = self.secret * 2048;
       self.mix_and_prune(tmp);

       pl.add_price(current, self.secret);

       self.secret
    }

    fn mix_and_prune(&mut self, val: usize) {
       self.secret ^= val;
       self.secret %= 16777216;
    }


}

struct PriceList {
    price: Vec<(usize, isize)>,
}

impl PriceList {
    fn new() -> Self {
       PriceList {
           price: Vec::new(),
        }
    }

    fn add_price(&mut self, curr_price: usize, new_price: usize) {
        let cp = curr_price % 10;
        let np = new_price % 10;	

        let diff = np as isize - cp as isize;
	
	self.price.push((np, diff));
    }
}

#[derive(Debug)]
struct PriceTable {
       price_table: HashMap<[isize; 4], usize>
}

impl PriceTable {
    fn new() -> Self {
       PriceTable {
           price_table: HashMap::new()
        }
    }

    fn update_price_table(&mut self, pl: &PriceList) {
       let mut hs : HashSet<[isize; 4]> = HashSet::new();
       
       for window in pl.price.windows(4) {
       	   let key = [window[0].1, window[1].1, window[2].1, window[3].1];
	   let val = window[3].0;

	   if hs.get(&key).is_none() {
	      *self.price_table.entry(key).or_insert(0) += val;
	      hs.insert(key);
	   }
       }
    }

    fn get_max(&self) -> usize {
        let max_val = self.price_table.values().max();
	if let Some(val) = max_val {
	    *val
	} else {
	    0
	}
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
   let seeds = read_input("day22.txt")?;
   let mut pt = PriceTable::new();

   let start = Instant::now();

   for s in seeds {
      let mut sn = SecretNumber::new(s);
      let mut pl = PriceList::new();
	    
      for _ in 0..2000 {
          sn.next_secret(&mut pl);
      }
      
      pt.update_price_table(&pl);
   }
   
   let max = pt.get_max();
   let end = start.elapsed();


   println!("PT {:?} {:?}", max, end); 

   Ok(())
}
