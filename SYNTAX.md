# Syntax
A quick reference to the syntax of the available login challenge transformation rules.

## Basics
The pattern can have any length and is read and applied from left to right to the challenge.\
If a pattern tries to access a character after the end of the challenge nothing happens.\

## Controll flow

#### block `{x}`
x wil be executed depending on the condition before the curly brackets.\
x can be one or multiple instructions.

#### loop `[x]`
The next block will be executed x times

#### slice `(x.y)`
The next block will only be applied on the substring from position x(inclusive) to position y(exclusive).\
If x or y isn't present the substring will start at the beginning/end of the string.\
Fails if y is smaller than x.\
If x or y are out of bounds they'll get set to the start or end.

#### char `(x)`
The next block will only be applied to the character at position x.


## Operations

#### shift up `>`
Shifts the character up by one position, this operations wraps.

###### Examples
a -> b\
Z -> A\
1 -> 2\
9 -> 0

#### shift down `<`
Shifts the character down by one position, this operations wraps.

###### Examples
b -> a\
A -> Z\
2 -> 1\
0 -> 9

#### replace `x=y`
Replaces all occurences of character x with character y.
x and y are only allowed to be alphanumerical

#### uppercase `^`
Converts all alphabetical characters to uppercase.

#### lowercase `_`
Converts all alphabetical characters to lowercase