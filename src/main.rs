use std::{
    collections::HashMap,
    io::{self, Write},
};

fn main() {
    let word_list = include_str!("wordlist.csv");
    let freq_list = include_str!("freq.csv");
    let (adj, total_words) = build_adj_list(word_list);
    let freq_map = build_freq_map(freq_list);
    let parent = build_parent_map(total_words, &adj, &freq_map);

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

fn build_adj_list(word_list: &'static str) -> (Vec<Vec<&'static str>>, usize) {
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

fn build_freq_map(freq_list: &'static str) -> HashMap<&'static str, u32> {
    let mut freq_map: HashMap<&'static str, u32> = HashMap::new();
    for line in freq_list.lines() {
        if let Some((word, freq_str)) = line.split_once(',') {
            let freq: u32 = freq_str.trim().parse().unwrap();
            freq_map.insert(word.trim(), freq);
        }
    }
    freq_map
}

fn build_parent_map<'a>(
    total_words: usize,
    adj: &[Vec<&'a str>],
    freq_map: &HashMap<&'a str, u32>,
) -> HashMap<&'a str, &'a str> {
    let mut parent = HashMap::with_capacity(total_words);
    let mut buckets: HashMap<[u8; 4], (&'a str, u32)> = HashMap::with_capacity(total_words);
    for [current_words, next_words] in adj.array_windows() {
        buckets.clear();
        for &word in current_words {
            let b = word.as_bytes();
            let word_freq = freq_map.get(word).copied().unwrap_or(0);
            for i in 0..4 {
                let mut mask = [b[0], b[1], b[2], b[3]];
                mask[i] = b'*';
                let prev = buckets.entry(mask).or_insert((word, word_freq));
                if word_freq > prev.1 {
                    *prev = (word, word_freq);
                }
            }
        }

        for &neighbor in next_words {
            let b = neighbor.as_bytes();
            let mut best_word = "";
            let mut best_freq = 0;

            for i in 0..4 {
                let mut mask = [b[0], b[1], b[2], b[3]];
                mask[i] = b'*';
                if let Some(&(p, freq)) = buckets.get(&mask)
                    && (best_word.is_empty() || freq > best_freq)
                {
                    best_word = p;
                    best_freq = freq;
                }
            }
            if !best_word.is_empty() {
                parent.insert(neighbor, best_word);
            }
        }
    }
    parent
}
