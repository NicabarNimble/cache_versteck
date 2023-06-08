use drand_core::HttpClient;
use rand::{seq::SliceRandom, SeedableRng};
use rand_chacha::ChaCha20Rng;

#[tokio::main]
async fn main() {
    let client: HttpClient = "https://api.drand.sh".try_into().unwrap();
    let latest = client.latest().await.unwrap();
    let round = latest.round();

    let seed: <ChaCha20Rng as SeedableRng>::Seed = latest.randomness().try_into().unwrap();
    let mut rng = ChaCha20Rng::from_seed(seed);

    let dice = vec![
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10,
        11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
    ];

    let roll = dice.choose(&mut rng).unwrap();
    println!("Dice roll: {} (round {})", roll, round);
}
