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


// STRUCTS
//--------------------------------------------------------------------------------------------

// Create a struct for a Philosopher
struct Philosopher {
    name: String,                           // String to contain the Philosopher's name
}

// Implementation of a Philosopher
impl Philosopher {

    // Function to define the name for each philosopher
    fn new(name: &str) -> Philosopher {

        Philosopher {
            name: name.to_string(),         // Assign the name 
        }

    }

}


// MAIN FUNCTION
//--------------------------------------------------------------------------------------------
fn main() {
    
    // Create five philosophers, per the original problem
    let ph1 = Philosopher::new("Socrates");
    let ph2 = Philosopher::new("Plato");
    let ph3 = Philosopher::new("Kant");
    let ph4 = Philosopher::new("Locke");
    let ph5 = Philosopher::new("Descartes");        

}


//--------------------------------------------------------------------------------------------
// END OF CODE