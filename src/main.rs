mod cats; // import the cats rust file, to use the "load_arts()" function

use rand::seq::{IndexedRandom};
use rand::rng;

fn main() {
    let arts = cats::load_arts();
    let mut rng = rng();

    if let Some(art) = arts.choose(&mut rng) {
        println!("{}", art);
    }
}