struct Record {
    matches_played: usize,
    wins: usize,
    draws: usize,
    losses: usize,
    points: usize,
}

impl Default for Record {
    fn default() -> Self {
        Self {
            matches_played: 0,
            wins: 0,
            draws: 0,
            losses: 0,
            points: 0,
        }
    }
}

pub fn tally(match_results: &str) -> String {
    use std::collections::HashMap;

    let mut season: HashMap<String, Record> = HashMap::new();

    for line in match_results.split('\n').filter(|s| !s.is_empty()) {
        let result: Vec<&str> = line.split(';').collect();
        match result[2] {
            "win" => {
                let mut rec = season
                    .entry(result[0].to_string())
                    .or_insert_with(Record::default);
                rec.matches_played += 1;
                rec.wins += 1;
                rec.points += 3;
                let mut rec = season
                    .entry(result[1].to_string())
                    .or_insert_with(Record::default);
                rec.matches_played += 1;
                rec.losses += 1;
            }
            "loss" => {
                let mut rec = season
                    .entry(result[0].to_string())
                    .or_insert_with(Record::default);
                rec.matches_played += 1;
                rec.losses += 1;
                let mut rec = season
                    .entry(result[1].to_string())
                    .or_insert_with(Record::default);
                rec.matches_played += 1;
                rec.wins += 1;
                rec.points += 3;
            }
            "draw" => {
                let mut rec = season
                    .entry(result[0].to_string())
                    .or_insert_with(Record::default);
                rec.matches_played += 1;
                rec.draws += 1;
                rec.points += 1;
                let mut rec = season
                    .entry(result[1].to_string())
                    .or_insert_with(Record::default);
                rec.matches_played += 1;
                rec.draws += 1;
                rec.points += 1;
            }
            _ => panic!(),
        }
    }

    let mut table = "Team                           | MP |  W |  D |  L |  P".to_string();
    let mut keys: Vec<_> = season.keys().collect();
    keys.sort_by(|a, b| {
        let val_a = season.get(*a).unwrap();
        let val_b = season.get(*b).unwrap();
        if val_a.points != val_b.points {
            val_b.points.cmp(&val_a.points)
        } else {
            a.cmp(b)
        }
    });
    println!("keys {:?}", keys);
    for key in keys {
        let val = season.get(key).unwrap();
        table = format!(
            "{}\n{: <30} | {: >2} | {: >2} | {: >2} | {: >2} | {: >2}",
            table, key, val.matches_played, val.wins, val.draws, val.losses, val.points
        )
    }
    table
}
