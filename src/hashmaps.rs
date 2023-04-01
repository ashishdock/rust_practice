use std::collections::HashMap;

fn main() {
    let mut scores: HashMap<String, u32> = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    dbg!(&scores);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    for (k, v) in &scores {
        println!("{k} -------------------- {v}");
    }

    *scores.entry(String::from("Blue")).or_insert(20) *= 3;

    println!("{:#?}", scores);
}
