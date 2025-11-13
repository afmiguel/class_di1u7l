#[derive(Clone)]
struct Counter {
    current: u32,
    max: u32,
}

impl Counter {
    fn new(max: u32) -> Counter {
        Counter { current: 0, max }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.max {
            let temp = self.current;
            self.current += 1;
            Some(temp)
        } else {
            None
        }
    }
}

fn main() {
    let counter1 = Counter::new(5);
    //let counter2 = counter1.clone();
    for i in counter1.map(|x| x*x) {
        println!("{}", i);
    }
    //println!("Sum={}", counter2.sum::<u32>());
}
