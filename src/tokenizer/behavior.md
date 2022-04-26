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

Boron syntax is designed to be highly readable and flexible.

The following introduction to Boron syntax is oriented towards individuals with experience in computer science.  If you are shaky with the following concepts then visit the `tutorials` directory for a gentler introduction.

## Comments

```
// Single-line comment
```

Boron does not support block comments.

## Data Types

### Primitive Types

The following primitive data types are available in Boron.

`int`: 32-bit integer
`float`: 32-bit floating-point number
`char`: 8-bit character
`bool`: boolean value (`True` or `False`)
`list<T>`: list of type `T` (heap-allocated, of arbitrary length)
`null`: null data type (used for functions that return nothing)

### Derived Types

The following derived types are available through Boron's standard library.

#### `str`

String (heap-allocated, of arbitrary length)

Import using `use std::string::Str`;

More information coming soon!

#### `vec<T>`

Vector of type `T` (used for linear algebra calculations)

Import using `use std::math::vec;`

More information coming soon!

#### `mx<T>`

Matrix of type `T` (used for linear algebra calculations)

Import using `use std::math::mx;`

More information coming soon!

## Variable Declaration

```
// Define a variable
x (int): 3;
pi (float): 3.1415;
mystr (str): "Hello, world!";
letter (char): 'a';
mybool (bool): True;
some_numbers (list<int>): [1, 2, 3, 4];
```

Note the following:
- Type annotations.  Type inference is not supported in Boron but will be added in a future update.
- Absence of assignment keyword (`let`, `var`, etc.)
- Presence of semicolons.  Semicolons are required and denote the end of a line.

## Function Definitions & Calls

```
// Define a function
does_nothing :: -> null {
    // Do nothing
}

// Define a function that accepts arguments
print_numbers :: a (int), b (int) -> null {
    print("{}", a);
    print("{}", b);
}

// Define a function that returns a result
five :: -> int {
    return 5;
}

// Define a function that accepts arguments *and* returns a result
add :: a (int), b (int) -> int {
    return a + b;
}

// Call does_nothing
does_nothing();

// Call print_numbers
print_numbers(1, 2);

// Call five
my_number (int): five();

// Call add
sum (int): add(3, 4);
```