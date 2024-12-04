use std::io;




fn lfsr(mut state: u32, polynomial: u32, num_bits: usize) -> (u32, u32) {
    let mut output_bit = 0;
    
    
    output_bit = (state & 1) as u32;

    
    let mut feedback = 0;
    for i in 0..num_bits {
        if (polynomial >> i) & 1 == 1 {
            feedback ^= (state >> i) & 1;
        }
    }

    print!("XOR-Bit: {}, ", feedback);
    // shift everythin to right and add cor_bit at the begining
    state = (state >> 1) | (feedback << (num_bits - 1));
    
    
    (state, output_bit)
}

fn main() {
    


    let polynomial: u32 = 0b010110110110010101001001; 

    let mut state: u32 = 0b100001111100001001100100; 

    let num_bits = 24;


    for i in 0..11 {
        let (new_state, output_bit) = lfsr(state, polynomial, num_bits);
        
        let decimal_value = new_state;
        

        println!("State {:<2}, {:<24}, {:<9}", i , format!("{:024b}", new_state), decimal_value, );
        state = new_state;
    }
}
