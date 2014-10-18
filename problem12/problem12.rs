mod numbers;

fn main() {
    let min = 500;
    let mut triangles = numbers::Triangles::new();

    for i in triangles {
        let factors = numbers::factors(i).len();

        if factors > min {
            println!("{} -> {} factors",i,factors);
            break;
        }
        println!("{} -> {}",i,factors);
    }
}
