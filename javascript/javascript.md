# Javascript

Javascript is a programming language that is built into most browsers. It is paired with the scripting language HTML to create websites. Javascript being built into browsers allows for front end web-dev. Apart from its syntax being awful and having inconsistencies in `if` statements people still like it (*somehow*).

## The basics

### Variables

You can create variables in javascript to store data. It is done by using the `let` keyword. The benefit of the `let` keyword is that scope is kept consistent.

```javascript
// Example
                  // v requires a semicolon like a good programming language
let a = "Hello World";    // A variable named "a" to keep other people confused storing a string type
let b = 2024;             // A variable named "b" storing an integer type
let c = 3.1415926535897;  // A variable named "c" storing a real/floating point type with the value of PI
let d = true;             // A variable named "d" storing a boolean
```

Some weird words have been mentioned haven't they? String? Integer? Real? Floating Point? Boolean? What on earth are these? These are data types. Data types are different ways of storing data. Each data type takes up a different amount of memory *(most of them)*. 

+ A boolean will take up 8 bits in unoptimised programming languages and can store `true` or `false`.
+ An Integer can take up anywhere from 8 to 64 bits. Some programming languages let you specify and others have it fixed at 64. 8 bits can store any value from -128 to 127 and 64 bits can store anything from -9,223,372,036,854,775,808 to 9,223,372,036,854,775,808 which is a *very* large range.
+ A float or real can store decimal values and can also store very large values however they lose accuracy. They can also use anything from 8 to 64 bits however it is recommended to use 64 to get the highest precision available.
+ A string can take up an infinite amount of space. It is typically an array (We'll get onto those later) of 16 bit values which direct to a character in your downloaded font. Some programs use 8 bit ascii arrays if they do not need other characters and storage is a concern.


