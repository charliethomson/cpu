



// Instructions
LDA addr -> Load the value at addr into the accumulator register
STA addr -> Store the value in the accumulator register at addr
INC      -> Increment the value in the accumulator register
DEC      -> Decrement the value in the accumulator register
SETV value -> Set the value of the user register to value
SETA addr  -> Set the value of the user register to the value at addr
STR addr -> Store the value in the user register at addr
LOAD addr -> Load the value at addr into the user register
ADD -> Add the value in the user register and the accumulator register and store it in the accumulator register, this keeps the user register unchanged
SUB -> Subtract the value of the user register from the accumulator and store it in the accumulator. This keeps the user register unchanged
JMP addr -> Unconditionally jump to addr
JC addr -> Conditionally jump if any of the flags are set
JZ addr -> Conditionally jump if the zero flag is set
JO addr -> Conditionally jump if the overflow flag is set
OUT -> Push the contents of the accumulator to stdout
EXIT code -> Exit the program with `code`
CLN -> Clone user into accumulator


// BYTE TABLE
*  0    1    2    3    4    5    6    7    8    9    A    B    C    D    E    F
0  NOP  LDA  STA  INC  DEC  SETV SETA STR  LOAD ADD  SUB  JMP  JC        JZ   JO
1  OUT  EXIT CLN  