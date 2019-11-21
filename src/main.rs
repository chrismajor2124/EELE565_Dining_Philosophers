//--------------------------------------------------------------------------------------------
//
//      EELE 565 - Parallel Processing
//      Fall 2019
//      Taught by Dr. Ross Snider       
//
//      Developers: James Eaton, Kaveen Liyanage, Chris Major, Trey Scofield
//
//      Objective: simulate the Dining Philosopher's Problem using the concurrency features of
//      Rust.     
//
//--------------------------------------------------------------------------------------------
use std::{mem, thread};
use std::sync::{Arc, Mutex};

// STRUCTS
//--------------------------------------------------------------------------------------------

// Chopstick struct
struct Chopstick {
     count: usize,                           // counting semaphore for individual chopstick
}
// Implementation of Chopstick
impl Chopstick {
    //Create a chopstick with a default count of 1
    fn init() -> Chopstick {
        let mut count = 1;
        Chopstick{
            count: count,                       // create Chopstick with count of 1
        }
    }
    // read count
    fn read(&self) -> usize {
        let mut temp = self.count.clone();
        temp
    }
    // increment count
    fn inc(&mut self) {
        self.count = self.count.wrapping_add(1);
    }
}
// implement copy trait
impl Copy for Chopstick {}
// implement clone trait
impl Clone for Chopstick {
    fn clone(&self) -> Self {
        *self
    }
}

// Chopstick array
struct CSarray {
    chopsticks: Vec<Arc<Mutex<Chopstick>>>, //vector of chopstick semaphores
}
// Implementation of Chopstick array
impl CSarray {
    //creates an array of chopsticks of size (Capacity)
    fn init(capacity: usize) -> CSarray {
        // create chopstick array
        let mut chopsticks: Vec<Arc<Mutex<Chopstick>>> = Vec::with_capacity(capacity);
        // fill chopstick array
        let mut cs = Chopstick::init();
        for _ in 0..capacity {
            let temp = Arc::new(Mutex::new(cs.clone()));
            chopsticks.push(temp);
        }
        // create CSarray struct to return
        CSarray {
            chopsticks: chopsticks,
        }
    }
}

// Create a struct for a Philosopher
struct Philosopher {
    name: String,                           // String to contain the Philosopher's name
    number: usize,                          // Number of philosopher
}

// Implementation of a Philosopher
impl Philosopher {

    // Function to define the name for each philosopher
    fn new(name: &str, number: usize) -> Philosopher {

        Philosopher {
            name: name.to_string(),         // Assign the name
            number: number,                 // Assign number
        }

    }

}
// SUB FUNCTIONS
//--------------------------------------------------------------------------------------------
fn reader(cs_lk: &Arc<Mutex<Chopstick>>) -> usize {
    // pass in protected chopstick, return count
    let mut cs = cs_lk.lock().unwrap();
    let count = cs.read();
    count

}

// MAIN FUNCTION
//--------------------------------------------------------------------------------------------
fn main() {
    
    // Create five philosophers, per the original problem (change to threads)
    let ph1 = Philosopher::new("Socrates",0);
    let ph2 = Philosopher::new("Plato",1);
    let ph3 = Philosopher::new("Kant",2);
    let ph4 = Philosopher::new("Locke",3);
    let ph5 = Philosopher::new("Descartes",4);

    // create semaphore array
    let CS = CSarray::init(5);
    // print the counts of the chopsticks
    let CSc0 = reader(&CS.chopsticks[0]);
    let CSc1 = reader(&CS.chopsticks[1]);
    let CSc2 = reader(&CS.chopsticks[2]);
    let CSc3 = reader(&CS.chopsticks[3]);
    let CSc4 = reader(&CS.chopsticks[4]);
    println! ( " Counts: {} {} {} {} {}", CSc0,CSc1,CSc2,CSc3,CSc4);

}


//--------------------------------------------------------------------------------------------
// END OF CODE