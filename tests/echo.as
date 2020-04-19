' Ask the user for input, then return the same input back

' Get the input 
in 0x40
' Print the memory, instructions
#DUMP_MEM
#DUMP_INST
' Output the same input back
out 0x41 &0x40
' Exit
#EXIT_OK
