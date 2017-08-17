use arduino::serial;

pub fn read_program() -> [u8; 512] {

    /*
     * Bitmasked 4 bits per instruction
     * 512 * 2 = 1024 instructions total
     */
    let mut instructions: [u8; 512] = [0; 512];
    let mut instruction_ord: usize = 0; // Location in instruction array

    // Initialise array
    for i in 0..512 {
        instructions[i] = 0;
    }

    // Read program from serial into memory until closing "$" is received
    'input: while instruction_ord < 511  {

        // Repeat twice and increment once because 2 instructions per place in instructions array
        for _ in 0..2 {

            // Left shift 4, then add code
            instructions[instruction_ord] = match serial::receive() {
                b'+' => (instructions[instruction_ord] << 4u8) + 1,
                b'-' => (instructions[instruction_ord] << 4u8) + 2,
                b'>' => (instructions[instruction_ord] << 4u8) + 3,
                b'<' => (instructions[instruction_ord] << 4u8) + 4,
                b'[' => (instructions[instruction_ord] << 4u8) + 5,
                b']' => (instructions[instruction_ord] << 4u8) + 6,
                b',' => (instructions[instruction_ord] << 4u8) + 7,
                b'.' => (instructions[instruction_ord] << 4u8) + 8,

                b'$' => break 'input,
                _ => instructions[instruction_ord]
            };
        }

        instruction_ord += 1;
    }

    instructions

}
