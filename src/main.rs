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
//      With reference to https://doc.rust-lang.org/1.1.0/book/dining-philosophers.html
//
//--------------------------------------------------------------------------------------------


// LIBRARIES
//--------------------------------------------------------------------------------------------
use std::{mem, thread, time, time::Duration};   // Library for mem, threads, and time
use std::sync::{Arc, Mutex};                    // Library for mutexes
use colored::*;                                 // Library for terminal color printing


// GLOBAL VARIABLES
//--------------------------------------------------------------------------------------------

// Define a segment of time for sleep()
static sleep_time : Duration = time::Duration::from_millis(500);


// STRUCTS
//--------------------------------------------------------------------------------------------

// Create a struct for a Chopstick
struct Chopstick {
     used_count: usize,                              // counting semaphore for individual chopstick
}

// Implementation of Chopstick
impl Chopstick {
    
    // Create a chopstick with a default count of 1
    fn init() -> Chopstick {
        let mut count = 0;                     // set chopsticks to 0 uses
        Chopstick{
            used_count: count,                       // create Chopstick with count of 1
        }
    }

    // Read count
    fn read(&self) -> usize {
        let mut temp = self.used_count.clone();
        temp
    }

    // Increment count
    fn inc(&mut self) {
        self.used_count = self.used_count.wrapping_add(1);
    }
}

// Implement copy trait
impl Copy for Chopstick {}

// Implement clone trait
impl Clone for Chopstick {
    fn clone(&self) -> Self {
        *self
    }
}


// Create a struct for a Chopstick array
struct CSarray {
    chopsticks: Vec<Mutex<Chopstick>>,     //vector of chopstick semaphores
}

// Implementation of Chopstick array
impl CSarray {

    // Creates an array of chopsticks of size (Capacity)
    fn init(capacity: usize) -> CSarray {

        // Print update message
        println! ( "{} {} have been added to the table.\n", capacity.to_string().magenta(), "chopsticks".magenta());

        // Create chopstick array
        let mut chopsticks: Vec<Mutex<Chopstick>> = Vec::with_capacity(capacity);

        // fill chopstick array
        let cs = Chopstick::init();
        for _ in 0..capacity {
            let temp = Mutex::new(cs.clone());
            chopsticks.push(temp);
        }

        // Create CSarray struct to return
        CSarray {
            chopsticks: chopsticks,
        }
    }
}


// Create a struct for a Philosopher
struct Philosopher {
    name: String,                                   // String to contain the Philosopher's name
    number: usize,                                  // Number of philosopher

    left_chopstick: usize,                          // Index for Left chopstick
    right_chopstick: usize,                         // Index for Right chopstick
}

// Implementation of a Philosopher
impl Philosopher {

    // Function to define the name for each philosopher
    fn init(name: &str, number: usize, chop_left: usize, chop_right: usize) -> Philosopher {

        // Print update message
        println! ( "{} (#{}) has joined the table.", name.to_string().blue(), number);

        Philosopher {
            name: name.to_string(),                 // Assign the name
            number: number,                         // Assign number

            left_chopstick: chop_left,              // Assign left chopstick
            right_chopstick: chop_right,            // Assign right chopstick
        }
    }

    // Function to represent thinking
    fn is_thinking(&self) {

        // Print update message
        println! ( "{} (#{}) is {}.", self.name.to_string().blue(), self.number, "thinking".yellow() );

    }

    // Function to represent thinking
    fn is_eating(&self, CS: &CSarray) {

        //let _left_chopstick = CSarray.chopsticks[self.left_chopstick].lock().unwrap();
        let mut _left_chopstick = CS.chopsticks[self.left_chopstick].lock().unwrap();
        let mut _right_chopstick = CS.chopsticks[self.right_chopstick].lock().unwrap();

        // Print update message
        println! ( "\n{} (#{}) has {}.", self.name.to_string().blue(), self.number, "started eating".green() );

        println! ( " > {}  {}", "Left Chopstick:".white(), self.left_chopstick.to_string().magenta());
        println! ( " > {} {}", "Right Chopstick:".white(), self.right_chopstick.to_string().magenta());

        // Sleep the thread
        thread::sleep(sleep_time);

        // Print update message
        println! ( "{} (#{}) has {}.\n", self.name.to_string().blue(), self.number, "finished eating".green() );
    
    }

//    fn is_living(&self, CS: &'static CSarray) {
//        thread::spawn(move || {
//            self.is_thinking();
//            self.is_eating(CS);
//            self.is_thinking();
//        });
//    }
}


// SUB FUNCTIONS
//--------------------------------------------------------------------------------------------
fn use_cs(cs_lk: &Mutex<Chopstick>) -> usize {
    
    // Pass in protected chopstick
    let mut cs = cs_lk.lock().unwrap();
    // inc count
    cs.inc();
    //return count
    let count = cs.read();
    count

}

fn print_status(option: usize) {

    // Print simulation message
    println! ("\n--------------------------------------------------------");

    match option {
        0 => println! ("BEGIN: DINING PHILOSOPHER'S PROBLEM"),
        1 => println! ("END: DINING PHILOSOPHER'S PROBLEM"),
        _ => println! ("{}", "ERROR!".red()),
    };

    println! ("--------------------------------------------------------\n");
}

// MAIN FUNCTION
//--------------------------------------------------------------------------------------------
fn main() {
    
    // Print simulation begin message (BEGIN)
    print_status(0);

    // Create a semaphore array
    let CS = Arc::new(CSarray::init(5));

    // Print the counts of the chopsticks
    let CSc0 = use_cs(&CS.chopsticks[0]);
    let CSc1 = use_cs(&CS.chopsticks[1]);
    let CSc2 = use_cs(&CS.chopsticks[2]);
    let CSc3 = use_cs(&CS.chopsticks[3]);
    let CSc4 = use_cs(&CS.chopsticks[4]);
    println! ( "Counts: {} {} {} {} {}\n", CSc0, CSc1, CSc2, CSc3, CSc4);

    // Create five philosophers, per the original problem (change to threads)
    let ph1 = Philosopher::init("Socrates", 0, 0, 1);
    let ph2 = Philosopher::init("Plato", 1, 1, 2);
    let ph3 = Philosopher::init("Kant", 2, 2, 3);
    let ph4 = Philosopher::init("Locke", 3, 3, 4);
    let ph5 = Philosopher::init("Descartes", 4, 0, 4);

    // [TODO] Make Descartes left-handed to avoid deadlock?


    let CS1 =Arc::clone(&CS);
    let CS2 =Arc::clone(&CS);
    let CS3 =Arc::clone(&CS);
    let CS4 =Arc::clone(&CS);
    let CS5 =Arc::clone(&CS);
    // Test eating/thinking (remove when threads are implemented)
    ph4.is_thinking();
    ph4.is_eating(&CS);
    ph4.is_thinking();

    thread::spawn(move || {
        ph1.is_thinking();
        ph1.is_eating(&CS1);
        ph1.is_thinking();
    });
    thread::spawn(move || {
        ph2.is_thinking();
        ph2.is_eating(&CS2);
        ph2.is_thinking();
    });
    thread::spawn(move || {
        ph3.is_thinking();
        ph3.is_eating(&CS3);
        ph3.is_thinking();
    });
    thread::spawn(move || {
        ph4.is_thinking();
        ph4.is_eating(&CS4);
        ph4.is_thinking();
    });
    thread::spawn(move || {
        ph5.is_thinking();
        ph5.is_eating(&CS5);
        ph5.is_thinking();
    });

    // Sleep the main function
    thread::sleep(sleep_time+sleep_time+sleep_time);
    //ph4.join().unwrap();
    // Print simulation begin message (END)
    print_status(1);
}

//--------------------------------------------------------------------------------------------
// END OF CODE