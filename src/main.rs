use std::io;

fn lfsr_step(input_nb: String, ){
    let mut wert:u32 = 000000000000;

    //get binary of input string
    match input_nb.parse::<u32>() {
        Ok(num) => {
            wert = num;
            let binary = format!("{:024b}", num);
            println!("The 24-bit binary representation is: {}", binary);
        }
        Err(_) => {
            println!("Invalid input. Please enter a valid positive decimal number.");
        }
    }

    let lsbs = wert & 1;

    print!("lsb:", lsbs);

    wert >>= 1;
    
    println!("Wert: {}", format!("{:024b}", wert));
}




fn main() {
    let mut input = String::new();

    println!("Put in ur Startwert!");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let trimmed_input: String = input.trim().repeat(1);

    lfsr_step(trimmed_input);


    


    //Left shifCuts last 32 bits
    //wert = wert << 1;
    //let wert_cut = wert & 0xFFFFFF;
    

}
