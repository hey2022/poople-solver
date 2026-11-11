use std::{
    collections::HashMap,
    io::{self, Write},
};

fn main() {
    let word_list = include_str!("wordlist.csv");
    let (adj, total_words) = load_words(word_list);
    let parent = build_parent_map(&adj, total_words);

    let mut input = String::new();
    loop {
        print!("Enter word: ");
        io::stdout().flush().unwrap();
        input.clear();
        if io::stdin().read_line(&mut input).unwrap() == 0 {
            break;
        }
        let word = input.trim().to_uppercase();
        if word.is_empty() {
            continue;
        }

        let mut curr = word.as_str();
        if word == "POOP" || parent.contains_key(curr) {
            println!("{}", word);
        } else {
            println!("No possible way to reach POOP.");
            continue;
        }

        let mut steps = 0;
        while let Some(next) = parent.get(curr) {
            println!("{}", next);
            curr = next;
            steps += 1;
        }
        println!("Reached POOP in {} steps!", steps);
    }
}

fn load_words(word_list: &'static str) -> (Vec<Vec<&'static str>>, usize) {
    let mut adj: Vec<Vec<&'static str>> = Vec::new();
    let mut total_words = 0;
    for line in word_list.lines() {
        if let Some((word, dist_str)) = line.split_once(',') {
            let distance: usize = dist_str.trim().parse().unwrap();
            while adj.len() <= distance {
                adj.push(Vec::new());
            }
            adj[distance].push(word.trim());
            total_words += 1;
        }
    }
    (adj, total_words)
}

fn build_parent_map<'a>(adj: &Vec<Vec<&'a str>>, total_words: usize) -> HashMap<&'a str, &'a str> {
    let mut parent = HashMap::new();
    let mut buckets: HashMap<[u8; 4], &'a str> = HashMap::with_capacity(total_words);
    for [current_words, next_words] in adj.array_windows() {
        buckets.clear();
        for &word in current_words {
            let b = word.as_bytes();
            for i in 0..4 {
                let mut mask = [b[0], b[1], b[2], b[3]];
                mask[i] = b'*';
                buckets.entry(mask).or_insert(word);
            }
        }

        for &neighbor in next_words {
            let b = neighbor.as_bytes();
            for i in 0..4 {
                let mut mask = [b[0], b[1], b[2], b[3]];
                mask[i] = b'*';
                if let Some(&p) = buckets.get(&mask) {
                    parent.insert(neighbor, p);
                    break;
                }
            }
        }
    }
    parent
}
