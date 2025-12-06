use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use anyhow::Result;
use std::fs::File;
use std::io::{self, BufRead};
use std::time::Instant;

// --------------------------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct UnorderedPairList {
    items: Vec<[char; 2]>,
}

impl UnorderedPairList {
    pub fn new(mut items: Vec<[char; 2]>) -> Self {
        items.sort(); // ensure consistent order for equality and hashing
        UnorderedPairList { items }
    }
}

impl PartialEq for UnorderedPairList {
    fn eq(&self, other: &Self) -> bool {
        self.items == other.items
    }
}

impl Eq for UnorderedPairList {}

impl Hash for UnorderedPairList {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.items.hash(state)
    }
}

// --------------------------------------------------------------------------------------------

#[derive(Debug, Clone)]
struct UnorderedPair([char; 2], [char; 2]);

impl PartialEq for UnorderedPair {
    fn eq(&self, other: &Self) -> bool {
        (self.0 == other.0 && self.1 == other.1) ||
        (self.0 == other.1 && self.1 == other.0)
    }
}

impl Eq for UnorderedPair {}

impl Hash for UnorderedPair {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Sort the pair to ensure consistent hash regardless of order
        let (a, b) = if self.0 <= self.1 {
            (&self.0, &self.1)
        } else {
            (&self.1, &self.0)
        };
        a.hash(state);
        b.hash(state);
    }
}

// --------------------------------------------------------------------------------------------


#[derive(Debug)]
struct NetworkLinks {
    links: HashSet<UnorderedPairList>
}

impl NetworkLinks
{
    fn new() -> Self {
        NetworkLinks { links: HashSet::new() }
    }
}

#[derive(Debug)]
struct TSets {
    sets: HashMap<[char; 2], Vec<[char; 2]>>,
}

impl TSets {
    fn new(nl: &NetworkLinks) -> Self {
        let mut hm = HashMap::new();

        for link in nl.links.iter() {
            if link.items[0][0] == 't' {
                hm.entry(link.items[0]).or_insert(Vec::new()).push(link.items[1]);
            }
            if link.items[1][0] == 't' {
                hm.entry(link.items[1]).or_insert(Vec::new()).push(link.items[0]);
            }
        }

        TSets { sets: hm }
    }

    fn find_all_clusters(&self, nl: &NetworkLinks) -> HashSet<UnorderedPairList> {
        let mut clusters = HashSet::new();
        for (key, val) in self.sets.iter() {
            for i in 0 .. val.len() {
                for j in i+1 ..val.len(){
                    if nl.links.contains(&UnorderedPairList::new(vec![val[i], val[j]])) {

                        clusters.insert(UnorderedPairList::new(vec![*key, val[i], val[j]]));
                    }
                }
            }
        }
        clusters
    }
}


fn read_input(path: &str) -> Result<NetworkLinks>
{
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let mut nl = NetworkLinks::new();

    for line in reader.lines() {
        let l = line?;
        let bytes = l.as_bytes();
        if bytes.len() != 5 {
            continue;
        }
        let a = [bytes[0] as char, bytes[1] as char];
        let b = [bytes[3] as char, bytes[4] as char];

        nl.links.insert(UnorderedPairList::new(vec![a, b]));
    }

    Ok(nl)

}

fn main() -> Result<()> {
    let nl = read_input("day23.txt")?;
    let tset = TSets::new(&nl);
    let result = tset.find_all_clusters(&nl);

    println!("TSET SIZE {:?}", result.len());

    Ok(())
}
