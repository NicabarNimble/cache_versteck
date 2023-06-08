use drand_core::HttpClient;
use rand::{seq::SliceRandom, SeedableRng};
use rand_chacha::ChaCha20Rng;
use std::io::{self, BufRead};

#[tokio::main]
async fn main() {
    let client: HttpClient = "https://api.drand.sh".try_into().unwrap();
    let latest = client.latest().await.unwrap();
    let round = latest.round();
    let seed: <ChaCha20Rng as SeedableRng>::Seed = latest.randomness().try_into().unwrap();
    let mut rng = ChaCha20Rng::from_seed(seed);

    println!("Enter the size of the dice:");
    let size: i32 = match io::stdin().lock().lines().next().unwrap().expect("Failed to read input").trim().parse() {
        Ok(value) if value <= 100 => value,
        _ => {
            println!("Nice try but that's not going to cut it!");
            return;
        }
    };

    let dice: Vec<i32> = (1..=size).collect();
    let roll = dice.choose(&mut rng).unwrap();
    println!("Dice roll: {} (round {})", roll, round);
}
