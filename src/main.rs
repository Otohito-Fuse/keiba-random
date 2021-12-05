use std::env;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::time::Instant;

use rand::{thread_rng, Rng};
use rand::distributions::Standard;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
struct Data {
    title: Option<String>,
    #[serde(rename = "numOfHeads")]
    num_of_heads: Option<usize>,
    horses: Vec<Horse>
}

#[derive(Deserialize, Debug, Clone)]
struct Horse {
    number: usize,
    name: String,
    odds: f64
}

impl PartialEq for Horse {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for Horse {}

fn read_json_file<P: AsRef<Path>>(path: P) -> Result<Data, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let data: Data = serde_json::from_reader(reader)?;

    Ok(data)
}

fn random_select(n: usize, horses: &Vec<Horse>) -> Result<Vec<Horse>, Vec<Horse>> {
    if n  >= horses.len() {
        return Ok(horses.to_vec());
    }

    let mut sum: f64 = 0f64;
    for horse in horses {
        sum += 1.0 / horse.odds;
    }
    if sum == 0f64 {
        return Ok(Vec::new());
    }

    let mut borders: Vec<f64> = Vec::new();
    let mut tmp: f64 = 0f64;
    for horse in horses {
        tmp += (1.0 / horse.odds) / sum;
        borders.push(tmp);
    }

    let mut rng = thread_rng();
    let mut random_f64: Vec<f64> = (&mut rng).sample_iter(Standard).take(n).collect();
    random_f64.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let mut random_horse: Vec<Horse> = Vec::new();
    let mut tmp: usize = 0;
    let mut dup_flag: bool = false;
    for each in random_f64 {
        let mut dup_flag_tmp = true;
        while each > borders[tmp] {
            dup_flag_tmp = false;
            tmp += 1;
        }
        if dup_flag_tmp {
            dup_flag = true;
        }
        random_horse.push(horses[tmp].clone());
    }
    if dup_flag {
        // はじめ重複をなくすためにここでOk/Errで分岐させていたが、
        // 人気の馬ほど重複しやすいため、人気の馬が出にくくなってしまう
        // Err(random_horse)
        // 一時的に両方Okとし重複を許している
        Ok(random_horse)
    } else {
        Ok(random_horse)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 2 {
        println!("引数にファイル名と頭数を指定してください");
        return Ok(())
    }
    let data: Data = read_json_file("./input/".to_string() + &args[1])?;
    let num: usize = args[2].parse()?;

    let start = Instant::now();
    let mut selected: Vec<Horse> = Vec::new();
    loop {
        if start.elapsed().as_secs() > 5 {
            break;
        }
        if let Ok(selected_tmp) = random_select(num, &data.horses) {
            selected = selected_tmp;
            break;
        }
    }
    let mut result: Vec<(usize, String)> = Vec::new();
    for horse in selected {
        result.push((horse.number, horse.name));
    }
    result.sort();
    for each in result {
        println!("{:2} {}", each.0, each.1);
    }

    Ok(())
}
