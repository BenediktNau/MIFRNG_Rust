use std::io;




fn lfsr(mut state: u32, polynomial: u32, num_bits: usize) -> (u32, u32) {
    let mut output_bit = 0;

    
    let mut xor_bit = 0;
    for i in 0..num_bits {
        if (polynomial >> i) & 1 == 1 {
            xor_bit ^= (state >> i) & 1;
        }
    }

    // shift everythin to right and add xor_bit at the begining
    state = (state >> 1) | (xor_bit << (num_bits - 1));
    
    
    (state, xor_bit)
}

fn main() {
    let mut polynomial: u32 = 0b010110110110010101001001; 

    let mut state: u32 = 0b100001111100001001100100; 

    let mut goalrandomnumber: u32 = 576159165;


    let mut input = String::new();
    println!("Put in ur Startwert!");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    match input.trim().parse::<u32>() {
        Ok(number) => {
            state = number;
        },
        Err(_) => {
            println!("Invalid input! Please enter a valid u32 number.");
        }
    }

    let mut input = String::new();
    println!("Put in ur Goal!");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    match input.trim().parse::<u32>() {
        Ok(number) => {
            goalrandomnumber = number;
        },
        Err(_) => {
            println!("Invalid input! Please enter a valid u32 number.");
        }
    }

    let mut input = String::new();
    println!("Put in ur Polynom!");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let trimmed_input = input.trim();
    println!("{}", trimmed_input);
    match u32::from_str_radix(&trimmed_input, 2) {
        Ok(number) => {
            polynomial = number;
        },
        Err(_) => {
            println!("Invalid input! Please enter a valid u32 number.");
        }
    }

    let num_bits = 24;

    let mut random_number:u32 = 0b0; 

    let mut count: i32 = 0;

    while(random_number != goalrandomnumber){
        let (new_state, xor_bit) = lfsr(state, polynomial, num_bits);
        
        random_number = ( random_number >> 1) | (xor_bit << (32 -1));

        
        state = new_state;
        count += 1;
    }

    println!("States: {}", count)
}
