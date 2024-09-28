use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();

    for i in 1..11 {
        println!("Random float: {}", rng.gen::<f64>());
    }
    
}