use std::io;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {

    println!("Enter the length of the string to generate");

    let mut pass = false;

    while pass == false
    {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        
        let length = match input.trim().parse::<u32>() {
            Ok(i) => {
                pass = true; // break us out
                i
            },
            Err(..) => { 
                0 // Returning 0 so we don't crash on failed parse
            }
        };
    }

    


    // Read an int N to represent the length of the string

    // Generate a random char for each place in the string
     // Create an array of the alphabet
     // Generate random number 1-26
     // Select index randomNum - 1 


    // Append the random char to a string builder

    //  RETURN
    Ok(())
}
