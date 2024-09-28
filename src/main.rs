use rand::Rng;
use rand_distr::{Exp, Distribution};



fn main() {
    let mut rng = rand::thread_rng();

    for _i in 1..11 {
        println!("Random float: {}", rng.gen::<f64>());
    }

    let mut a = 100;
    let mut t : f64 = 0.0;

    println!("Time\tA pop");

    println!("{t}\t{a}");
    while a > 0 {
        let r = rng.gen::<f64>();
        let dt = -r.ln();
        t += dt;
        a-=1;
        println!("{t}\t{a}");
    }


    let exp = Exp::new(2.0).unwrap();
    let v = exp.sample(&mut rand::thread_rng());
    println!("{} is from a Exp(2) distribution", v);
    
}