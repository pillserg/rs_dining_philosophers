struct Philosopher {
    name: String,
}

impl Philosopher {
    fn new(name: &str) -> Philosopher {
        Philosopher {
            name: name.to_string()
        }
    }

    fn eat(&self) {
        println!("{:?} is done eating", self.name );
    }
}


fn main() {
    
    let philosophers: Vec<Philosopher> = vec![
        "Judith Butler", 
        "Cilles Deleuze", 
        "Karl Marx", 
        "Emma Goldman", 
        "Michel Foucault"
    ].iter().map(|n|Philosopher::new(n)).collect();
    
    for p in &philosophers {
        p.eat();
    }
    
}
