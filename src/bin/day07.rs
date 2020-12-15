use regex::Regex;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];

    let input_vec = read_lines_to_vec(file_name).unwrap();
    println!("part1 result: {}", solve(&input_vec));
}

fn read_lines_to_vec(file_name: &str) -> Result<Vec<String>, Error> {
    let input = File::open(file_name).unwrap();
    let buffered = BufReader::new(input);

    let mut vec = Vec::new();
    for line in buffered.lines() {
        vec.push(line?.trim().to_owned());
    }

    Ok(vec)
}

fn solve(input_vec: &[String]) -> usize {
    let graph = build_graph(input_vec);

    traverse_graph_to_count_bags(&graph, "shiny gold")
}

fn traverse_graph_to_count_bags(graph: &HashMap<String, Vec<String>>, bag: &str) -> usize {
    let mut bag_num = 0;

    // traverse the graph to count bags
    for outer in graph.keys() {
        if has_bag(&graph, outer, bag) {
            bag_num += 1;
        }
    }
    bag_num
}

fn has_bag(graph: &HashMap<String, Vec<String>>, outer: &String, bag: &str) -> bool {
    let inners = graph.get(outer).unwrap();

    for inner in inners {
        if inner == bag {
            return true;
        } else if has_bag(graph, inner, bag) {
            return true;
        }
        // don't write else {
        //     return has_bag(graph, inner, bag)
        // }
    }
    false
}

fn build_graph(input_vec: &[String]) -> HashMap<String, Vec<String>> {
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();
    let re = Regex::new(r"(\w+ \w+) bag[s]*").unwrap();

    for line in input_vec {
        add_node(line, &mut graph, &re);
    }
    graph
}

fn add_node<'a>(line: &String, graph: &mut HashMap<String, Vec<String>>, re: &Regex) {
    let mut first = true;
    let mut big_bag = "".to_owned();
    for caps in re.captures_iter(line) {
        if first {
            // current mother bag
            graph
                .entry(caps[1].to_owned())
                .or_insert(Vec::<String>::new());
            big_bag = caps[1].to_owned();
            first = false;
        } else {
            // current child bags
            if &caps[1] != "no other" {
                // add child bags if they exist
                graph.get_mut(&big_bag).unwrap().push(caps[1].to_owned());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test_solve() {
        let vec: Vec<String> = vec![
            "light red bags contain 1 bright white bag, 2 muted yellow bags.",
            "dark orange bags contain 3 bright white bags, 4 muted yellow bags.",
            "bright white bags contain 1 shiny gold bag.",
            "muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.",
            "shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.",
            "dark olive bags contain 3 faded blue bags, 4 dotted black bags.",
            "vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.",
            "faded blue bags contain no other bags.",
            "dotted black bags contain no other bags.",
        ]
        .into_iter()
        .map(|s| s.to_owned())
        .collect();

        assert_eq!(solve(&vec), 4);
    }
}
