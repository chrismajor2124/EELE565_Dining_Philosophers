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
use std::{thread, time, time::Duration};        // Library for mem, threads, and time
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
     used_count: usize,                              // Counting semaphore for individual chopstick
}

// Implementation of Chopstick
impl Chopstick {
    
    // Create a chopstick with a default count of 1
    fn init() -> Chopstick {
        let mut count = 0;                          // Set chopsticks to 0 uses
        Chopstick{
            used_count: count,                      // Create Chopstick with count of 1
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

        // Assign chopsticks to philosopher
        let mut _left_chopstick = CS.chopsticks[self.left_chopstick].lock().unwrap();
        let mut _right_chopstick = CS.chopsticks[self.right_chopstick].lock().unwrap();

        // Print update messages
        println! ( "\n{} (#{}) has {}.", self.name.to_string().blue(), self.number, "started eating".green() );
        println! ( " > {}  {} ({})", "Left Chopstick acquired:".white(), self.left_chopstick.to_string().magenta(), self.number.to_string().blue());
        println! ( " > {} {} ({})", "Right Chopstick acquired:".white(), self.right_chopstick.to_string().magenta(), self.number.to_string().blue());

        // Sleep the thread
        thread::sleep(sleep_time);

        // Print update message
        println! ( "{} (#{}) has {}.\n", self.name.to_string().blue(), self.number, "finished eating".green() );

        // Update thread count
    
    }

}


// SUB FUNCTIONS
//--------------------------------------------------------------------------------------------

// Function for a Philosopher to use a Chopstick
fn update_eatC(counts_lk: &Arc<Mutex<Vec<i32>>>, index: usize)  {
    
    // Pass in a protected counts
    let mut counts = counts_lk.lock().unwrap();

    // Read and return count value
    counts[index] = counts[index]+1;
}

// Function to print runtime status messages
fn print_status(option: usize) {

    // Print simulation message
    println! ("\n--------------------------------------------------------");

    match option {
        0 => println! ("BEGIN: DINING PHILOSOPHER'S PROBLEM"),
        1 => println! ("END: DINING PHILOSOPHER'S PROBLEM"),
        2 => println! ("THREADS START"),
        3 => println! ("THREADS REPORT"),
        _ => println! ("{}", "ERROR!".red()),
    };

    println! ("--------------------------------------------------------\n");
}

// Displays if the philosophers ate or not
fn philosopher_upate(count_vec: &Arc<Mutex<Vec<i32>>>) {

    // Print update
    println! ("");

    // Pass in a protected counts
    let mut counts = count_vec.lock().unwrap();

    // If the values of counts are non-zero, all philosophers have eaten
    for pos in 0..counts.len() {
        
        match counts[pos] {
            0 => println!("{} Philosopher #{} failed to eat ({} times)", "ERROR:".red(), pos, counts[pos]),
            _ => println!("{} Philosopher #{} ate successfully ({} times)", "SUCCESS:".green(), pos, counts[pos])

        }
    }
}

// MAIN FUNCTION
//--------------------------------------------------------------------------------------------
fn main() {

    // Create vectors
    let mut vec = vec![0;5];
    let mut counts = Arc::new(Mutex::new(vec));

    // Print simulation begin message (BEGIN)
    print_status(0);

    // Create a semaphore array
    let CS = Arc::new(CSarray::init(5));

    // Create five philosophers, per the original problem (change to threads)
    // ph5 is left-handed as a deadlock mitigation strategy
    let ph1 = Philosopher::init("Socrates", 0, 0, 1);
    let ph2 = Philosopher::init("Plato", 1, 1, 2);
    let ph3 = Philosopher::init("Kant", 2, 2, 3);
    let ph4 = Philosopher::init("Locke", 3, 3, 4);
    let ph5 = Philosopher::init("Descartes", 4, 0, 4);

    // Print the counts of the chopsticks
    let CS1 =Arc::clone(&CS);
    let CS2 =Arc::clone(&CS);
    let CS3 =Arc::clone(&CS);
    let CS4 =Arc::clone(&CS);
    let CS5 =Arc::clone(&CS);

    // Print simulation begin message (THREADS)
    print_status(2);

    // Socrates (ph1) starts his thread
    thread::spawn(move || {
        for i in 0..10 {
            ph1.is_thinking();
            ph1.is_eating(&CS1);
            ph1.is_thinking();
        }
    });

    // Plato (ph2) starts his thread
    thread::spawn(move || {
        for i in 0..10 {
            ph2.is_thinking();
            ph2.is_eating(&CS2);
            ph2.is_thinking();
        }
    });

    // Kant (ph3) starts his thread
    thread::spawn(move || {
        for i in 0..10 {
            ph3.is_thinking();
            ph3.is_eating(&CS3);
            ph3.is_thinking();
        }
    });

    // Locke (ph4) starts his thread
    thread::spawn(move || {
        for i in 0..10 {
            ph4.is_thinking();
            ph4.is_eating(&CS4);
            ph4.is_thinking();
        }
    });

    // Descares (ph5) starts his thread
    thread::spawn(move || {
        for i in 0..10 {
            ph5.is_thinking();
            ph5.is_eating(&CS5);
            ph5.is_thinking();
        }
    });

    // Sleep the main function
    thread::sleep(20 * sleep_time);

    // Print simulation begin message (END)
    print_status(3);

    // Report on philosopher activity
    philosopher_upate(&counts);

    // Print simulation begin message (END)
    print_status(1);
}

//--------------------------------------------------------------------------------------------
// END OF CODE