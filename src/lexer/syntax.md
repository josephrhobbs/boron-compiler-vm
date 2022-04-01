# Boron Syntax and Behavior

Boron is a high-level programming language, designed to be fast, memory-safe, efficient, and easy to use.

As such, its syntax must be easily readable by both humans and machines.

# Guiding Design Principles

Boron is designed to be as easy as possible for programmers to use while maximizing memory safety and minimizing run time and memory/CPU usage.

Boron should be:
- Simple.  Computer science is hard enough... it shouldn't be complicated by weird programming syntax.
- Intuitive.  Boron should be self-documenting whenever possible.
- Reliable.  Boron should run the way you expect it to.
- Robust.  Boron should protect memory safety and maximize efficiency whenever possible without sacrificing simplicity.
- Flexible.  Experienced programmers should be able to use Boron syntax in many different ways.  Programmers should have control over what happens when errors are thrown.
- Communicative.  If something happens that the programmer does not expect, the programmer should know.  Errors should never pass silently.

# Syntax

Simple syntax
```
// Comment

/*
Multline comment
*/

// Immutable assignment
// Note the use of the colon (:) as the assignment operator, not equals (=)
// The assignment keyword is "let"
// Note also that type inference is supported

let a (i32): 10;  // This is a 32-bit signed integer
let b: 32;  // This is inferred to be a 32-bit signed integer


// Mutable assignment
// This variable may be changed after definition
let mut sum: 0;
sum: a + b;

// Function definition
/* A few things:
    - Function declaration keyword is "func"
    - Anything after :: is arguments.  Arguments are optional.  Separate arguments.  Arguments need to be type-annotated.
    - The type specified after -> is the return type.  Exclude -> for functions that do not return anything.
*/

func do_nothing {

}

func print_number :: a: i32 {
    println!("{a}");
}

func add :: a: i32, b: i32 -> i32 {
    return a + b;
}

// Macro definition
// Macros are defined in a similar way to functions.  However, they never return anything.
// Arguments are specified inside the macro definition to allow a macro's arguments to be more flexible.
// See below for more information on macros
macr add {
    (a: i32) -> {
        a
    }

    (a: i32, b: i32) -> {
        a + b
    }
}

// Function call and assignment of result value
sum: add(a, b);

// Macro call
// Note the ! denoting a macro call rather than a function
sum: add!(a, b);
```

Special macros
```
// The todo! macro 
fn not_yet_implemented {
    todo!();
}

```

# Functions vs. Macros

Boron supports the use of both functions and macros.  Explaining the difference between functions and macros is best in the context of Boron bytecode.

Compilation converts Boron source code into Boron bytecode, which is ready to run on the Boron virtual machine.  Boron bytecode represents a series of low-level operations on virtual memory in order to produce a result.

The Boron virtual machine executes bytecode by scanning through, one byte at a time, and executing the instruction at each byte.

On compilation, functions are sent to a specific **subroutine**.  That is, the functions are written *one time* into the source code at a specific location as a subroutine.  Whenever a function is called, the current position (or program counter) is moved to the position of the subroutine, and the subroutine executes.  Then the subroutine jumps back to the original position in the code.  This may be best explained using a simple graphic.

```
SUBROUTINE 1
SUBROUTINE 2
ENTRY POINT         <= The VM is looking here right now
CALL SUBROUTINE 1
CALL SUBROUTINE 3
SUBROUTINE 3
```

The program counter (PC) moves down to `CALL SUBROUTINE 1`.

```
SUBROUTINE 1
SUBROUTINE 2
ENTRY POINT 
CALL SUBROUTINE 1   <=
CALL SUBROUTINE 3
SUBROUTINE 3
```

The PC moves to subroutine 1 and executes.

```
SUBROUTINE 1        <=
SUBROUTINE 2
ENTRY POINT 
CALL SUBROUTINE 1
CALL SUBROUTINE 3
SUBROUTINE 3
```

At subroutine completion, the PC returns to subroutine 1.

```
SUBROUTINE 1
SUBROUTINE 2
ENTRY POINT 
CALL SUBROUTINE 1   <=
CALL SUBROUTINE 3
SUBROUTINE 3
```

**Macros**, on the other hand, are pieces of code that write other pieces of code.  A macro call in Boron source code is replaced with raw code during compilation, and the actual definition is not stored as a subroutine.

# Memory Management

Boron does not have a garbage collector.  Instead, Boron uses what is referred to as **borrowing**.  Users of Rust will be quite familiar with the concept.  For those who are not familiar with borrowing, here is a brief explanation.

TODO: Include explanation of borrowing.

# Error Handling

TODO