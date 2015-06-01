use std::thread;
use std::sync::{Mutex, Arc};


struct Philosopher {
    name: String,
    left: usize,
    right: usize
}

impl Philosopher {
    fn new(name: &str, left: usize, right: usize) -> Philosopher {
        Philosopher {
            name: name.to_string(),
            left: left,
            right: right
        }
    }

    fn eat(&self, table: &Table) {
        let _left = table.forks[self.left].lock().unwrap();
        let _right = table.forks[self.right].lock().unwrap();
        println!("{} is eating", self.name);
        thread::sleep_ms(1000);
        println!("{} is done eating", self.name );
    }
}

struct Table{
    forks: Vec<Mutex<()>>,
}


fn main() {
    let table = Arc::new(Table {
        forks: (0..5).map(|_|Mutex::new(())).collect()
    });

    let philosophers: Vec<Philosopher> = vec![
        ("Judith Butler", 0, 1),
        ("Cilles Deleuze", 1, 2),
        ("Karl Marx", 2, 3),
        ("Emma Goldman", 3, 4),
        ("Michel Foucault", 0, 4),
    ].iter().map(|&(n, l, r)| {
        Philosopher::new(n, l, r)
    }).collect();

    let handles: Vec<_> = philosophers.into_iter().map(|p| {
        let table = table.clone();
        thread::spawn(move || {
            p.eat(&table);
        })
    }).collect();

    for h in handles {
        h.join().unwrap();
    }
}
