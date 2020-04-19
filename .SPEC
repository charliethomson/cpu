
// ARGUMENT PREFIXES
" => String -> Everything until the corresponding " is one argument, a string
' => Character -> the character directly following the ' is interpreted as an ascii character
# => Address -> If the instruction expects a value, you can use this to get the value at the address directly following the #
% => Register -> The instruction will use the value in the register (%a / %b) as a value or address, depending on what it expects,
                 If it expects `Either`, it will use it as a value unless otherwise stated (#%a for the value of a as an address)
$ => Raw Value -> A raw byte value ($0x43 | $123)


// COMMENTS
Comments are lines that start with ', ", or `


// INSTRUCTIONS
< sta | stb > arg:addr => Store the value of < reg_a | reg_b> in *arg
< lda | ldb > arg:addr => Load the value from arg into < reg_a | reg_b >
< lt | le | eq | ge | gt > => Compare < reg_a > and < reg_b > and set < flag > accordingly. Flag is cleared after one instruction
< jmp > arg:addr => Unconditionally set < ip > to arg
< ji > arg:addr => Conditionally set < ip > to arg if < flag > is set
< push > < sz:sz arg:either > => Push < arg | *arg > to the stack
< pop > arg:addr => Pop the data from the top of the stack to < arg >
< in > arg:addr => Take input from stdin, store sizeof input in arg, store input in arg + 1 - arg + 1 + sizeof input 
< out > arg:addr size:sz => Output the data from { arg .. arg + size } to stdout
< adda | addb > arg:addr => < reg_a | reg_b > += *arg
< suba | subb > arg:addr => < reg_a | reg_b > -= *arg
< nop > => Skip this instruction

Instructions are loaded in line by line, stored in < memory > starting from 0x00,
until all instructions are stored, or there's a memory overflow

// BYTE TABLE

0x 0   1   2   3   4
0  nop sta stb lda ldb

1  lt  le  eq  ge  gt

2  jmp ji

3  in  out push pop

4  adda addb suba subb

..

8 => see INTERNAL BYTECODES

// BYTES MAP

0x00 => nop
0x01 => sta
0x02 => stb
0x03 => lda
0x04 => ldb
0x10 => lt
0x11 => le
0x12 => eq
0x13 => ge
0x14 => gt
0x20 => jmp
0x21 => ji
0x30 => in
0x31 => out
0x32 => push
0x33 => pop
0x40 => adda
0x41 => addb
0x42 => suba
0x43 => subb

// TARGS
0x00 => []
0x01 => [Addr]
0x02 => [Addr]
0x03 => [Addr]
0x04 => [Addr]
0x10 => []
0x11 => []
0x12 => []
0x13 => []
0x14 => []
0x20 => [Addr]
0x21 => [Addr]
0x30 => [Addr]
0x31 => [Addr, Val]
0x32 => [Val, Either[Addr, Val]]
0x33 => [Addr]
0x40 => [Either[Addr, Val]]
0x41 => [Either[Addr, Val]]
0x42 => [Either[Addr, Val]]
0x43 => [Either[Addr, Val]]



// INTERNAL BYTECODES
These are mainly meant for testing, exiting a program at a specific time, setting up memory
in code as #LABEL
0x80 => EXIT_OK Exits with an error code of zero
0x81 => EXIT_ERR code:err_cd
0x82 => DUMP_MEM Dump out the current hex values in memory
0x83 => DUMP_INST Dump out the instructions as strings
0x85 => ASSERT_FLAG Assert that flag is set, EXIT_OK if true, EXIT_ERR 1 if false



// ERROR CODES
1 => ASSERT_ERR