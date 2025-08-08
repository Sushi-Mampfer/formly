# Syntax
A quick reference to the syntax of tha available login rules.

## Basics
The pattern can have any length and is read and applied from left to right to the string.
If a pattern tries to access a character after the end of the string nothing happens.
If no specific char or substring is selected.

## Controll flow

#### block `{x}`
x wil be executed depending on the condition before the curly brackets.
x can be one or multiple instructions.

#### loop [x]
The next instruction or block will be executed x times

#### slice `(x..y)`
The next instruction or block will only be applied on the substring from position x(inclusive) to position y(exclusive).
If x or y isn't present the substring will start at the beginning/end of the string.

#### char `(x)`
The next instruction or block will only be applied to the character at position x.


## Operations

#### shift up `>`
Shifts the character up by one position, this operations wraps.
If the character is not alphanumeric nothing happens.

###### Examples
a -> b
Z -> A
1 -> 2
9 -> 0

#### shift down `<`
Shifts the character down by one position, this operations wraps.
If the character is not alphanumeric nothing happens.

###### Examples
b -> a
A -> Z
2 -> 1
0 -> 9

#### replace `x=y`
Replaces all occurences of character x with character y.
x and y are only allowed to be alphanumerical

#### uppercase `^`
Converts all alphabetical characters to uppercase.

#### lowercase `_`
Converts all alphabetical characters to lowercase