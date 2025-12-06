use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use anyhow::Result;
use std::fs::File;
use std::io::{self, BufRead};
use std::fmt;
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
struct Graph {
    nodes: HashSet<[char; 2]>,
    nbrs: HashMap<[char; 2], HashSet<[char; 2]>>,
}

impl Graph {
    fn new(nl: &NetworkLinks) -> Self {
        let mut nodes = HashSet::new();
        let mut nbrs = HashMap::new();

        for link in nl.links.iter() {
            nodes.insert(link.items[0]);
            nodes.insert(link.items[1]); 
            
            nbrs.entry(link.items[0]).or_insert(HashSet::new()).insert(link.items[1]);
            nbrs.entry(link.items[1]).or_insert(HashSet::new()).insert(link.items[0]);
        }

        Graph { nodes, nbrs }
    }
}

impl fmt::Display for Graph {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Nodes: ")?;

        for n in self.nodes.iter() {
            write!(f, "{} ", n.iter().collect::<String>())?;
        }

        for (node, neighbors) in &self.nbrs {
            write!(f, "\n{}: ", node.iter().collect::<String>())?;
            for nbr in neighbors {
                write!(f, "{} ", nbr.iter().collect::<String>())?;
            }
        }

        Ok(())     
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

fn bron_kerbosch(
    graph: &Graph,
    r: &mut HashSet<[char; 2]>,
    p: &mut HashSet<[char; 2]>,
    x: &mut HashSet<[char; 2]>,
    cliques: &mut Vec<HashSet<[char; 2]>>)
{
    if p.is_empty() && x.is_empty() {
        cliques.push(r.clone());
        return;
    }

    let p_copy = p.clone();
    for v in p_copy.iter() {

        let mut r_new = r.clone();
        r_new.insert(*v);
        let mut p_new = p.intersection(graph.nbrs.get(v).expect("Node not found in neighbhor list")).cloned().collect();
        let mut x_new = x.intersection(graph.nbrs.get(v).expect("Node not found in neighbhor list")).cloned().collect();

        bron_kerbosch(graph, &mut r_new, &mut p_new, &mut x_new, cliques);

        p.remove(v);
        x.insert(*v);
    }
}

fn format_clique_info(cliques: &Vec<HashSet<[char; 2]>>) -> String {
    let mut result = String::new();

    let mut max_len = 0;
    for c in cliques.iter() {
        if c.len() > max_len {
            result.clear();

            let mut c_copy = c.iter().cloned().collect::<Vec<_>>();
            c_copy.sort();
            for node in c_copy.iter() {
                result.push_str(&format!("{},", node.iter().collect::<String>()));
            }
            max_len = c.len();
        }
    }
    result
}

fn main() -> Result<()> {
    let nl = read_input("day23.txt")?;
    let graph = Graph::new(&nl);
    //let result = tset.find_all_clusters(&nl);

    let start = Instant::now();
    let mut cliques = Vec::new();
    let mut r = HashSet::new();
    let mut p = graph.nodes.clone();
    let mut x = HashSet::new();
    bron_kerbosch(&graph, &mut r, &mut p, &mut x, &mut cliques);

    let clique_info = format_clique_info(&cliques);

    let end = start.elapsed();
    println!("Elapsed time: {:?}", end);
    println!("Clique Info: {}", clique_info);

    Ok(())
}
