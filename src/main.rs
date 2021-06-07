enum people {
    Ram,
    Shyam,
    Hari,
    Kalu,
}

fn main() {
    greet_people(&people::Kalu)
}

fn greet_people(person: &people) {
    match person {
        people::Ram => {
            println!("Hello I am Ram Dai, I am don")
        }
        people::Hari => {
            println!("Hello I am Hari Dai, I am don")
        }
        people::Shyam => {
            println!("Hello I am Shyam Dai, I am don")
        }
        _ => {
            println!("Hello I am the default case")
        }
    }
}
