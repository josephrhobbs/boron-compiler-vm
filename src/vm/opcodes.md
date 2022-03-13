OPCODES
=======

The Boron Virtual Machine currently has 17 opcodes.  See documentation for each below.



`0x00`: NOP (no operation)

No arguments.



`0x01`: STO (store a value in a given register)

Arguments:
- Register (1 byte): register
- Value (8 bytes): value to store in that register

`0x02`: ADD (add to a given register a given value)

Arguments:
- Register (1 byte): register
- Value (8 bytes): value to add to that register

`0x03`: SUB (subtract from a given register a given value)

Arguments:
- Register (1 byte): register
- Value (8 bytes): value to subtract from that register



`0x11`: LD (loads into a given register a value at a given address)

Arguments:
- Register (1 byte): register
- Pointer (4 bytes): location in memory to get data from

`0x12`: STO (stores from a given register a value into memory at a given address)

Arguments:
- Register (1 byte): register
- Pointer (4 bytes): location in memory to store to



`0x21`: LDR (loads into a given register a value in a given register)

Arguments:
- Destination register (1 byte): register that will store the final value
- Pointer register (1 bytes): register that stores the pointer to memory where the value currently is

`0x22`: STR (stores from a given register a value into memory at an address provided by a given register)

Arguments:
- Source register (1 byte): register that stores the value
- Pointer register (1 bytes): register that stores the pointer to memory where the value will end up



`0x31`: LSL (logical shift left a value in a given register by a given number of places)

Arguments:
- Register (1 byte): register
- Places (1 byte): number of digits to shift left by

`0x32`: LSR (logical shift right a value in a given register by a given number of places)

Arguments:
- Register (1 byte): register
- Places (1 byte): number of digits to shift right by



`0x41`: JLT (jump less than)

Arguments:
- Pointer (4 bytes): location in memory to jump to
- Register 1 (1 byte): value in register 1 should be less than value in register 2
- Register 2 (1 byte)

`0x42`: JLE (jump less than or equal)

Arguments:
- Pointer (4 bytes): location in memory to jump to
- Register 1 (1 byte): value in register 1 should be less than or equal to value in register 2
- Register 2 (1 byte)

`0x43`: JGT (jump greater than)

Arguments:
- Pointer (4 bytes): location in memory to jump to
- Register 1 (1 byte): value in register 1 should be greater than value in register 2
- Register 2 (1 byte)

`0x44`: JGE (jump greater than or equal)

Arguments:
- Pointer (4 bytes): location in memory to jump to
- Register 1 (1 byte): value in register 1 should be greater than or equal to value in register 2
- Register 2 (1 byte)

`0x45`: JEQ (jump equal)

Arguments:
- Pointer (4 bytes): location in memory to jump to
- Register 1 (1 byte): value in register 1 should be equal to value in register 2
- Register 2 (1 byte)

`0x46`: JNE (jump not equal)

Arguments:
- Pointer (4 bytes): location in memory to jump to
- Register 1 (1 byte): value in register 1 should be not equal to value in register 2
- Register 2 (1 byte)



`0x51`: XSR (execute subroutine)

Arguments:
- Pointer (4 bytes): location in memory to jump to



`0xA1`: TX (print the first byte from the output buffer to the console)

No arguments

`0xA2`: RX (load the first byte from the input buffer to the given register)

Arguments:
- Register (1 byte): location to store byte



`0xFF`: HLT (halt program execution)