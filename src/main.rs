use std::{
    collections::HashMap,
    io::{self, Write},
};

fn main() {
    let word_list = include_str!("wordlist.csv");
    let adj = load_words(word_list);
    let parent = build_parent_map(&adj);

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

fn load_words(word_list: &'static str) -> HashMap<u32, Vec<&'static str>> {
    let mut adj: HashMap<u32, Vec<&'static str>> = HashMap::new();
    for line in word_list.lines() {
        if let Some((word, dist_str)) = line.split_once(',') {
            let distance: u32 = dist_str.trim().parse().unwrap();
            adj.entry(distance).or_default().push(word.trim());
        }
    }
    adj
}

fn build_parent_map<'a>(adj: &HashMap<u32, Vec<&'a str>>) -> HashMap<&'a str, &'a str> {
    let mut parent = HashMap::new();
    let mut buckets: HashMap<[u8; 4], &'a str> = HashMap::with_capacity(2048);
    let mut distance = 0;

    while let (Some(current_words), Some(next_words)) =
        (adj.get(&distance), adj.get(&(distance + 1)))
    {
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

        distance += 1;
    }

    parent
}
