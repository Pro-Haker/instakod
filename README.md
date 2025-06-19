# InstaKod

Bad language from school, recreated to look like Assembly!!! :D
[Ahmed-approved!](https://ermitavulpe.github.io/ask-ahmed) (Ahmed is made by my brother)

# Installation

Go to the [Releases](https://github.com/Pro-Haker/instakod/releases) page and download the latest `instakod.exe` file.

# Interpreting/Running the code

To run your InstaKod code, move `instakod.exe` and your code file to the same folder, then run:
```sh
.\instakod.exe <your file name>
```

# Examples

This language can be pretty complex, so I've added [a folder with examples!](https://github.com/Pro-Haker/instakod/blob/master/examples)

# IMPORTANT NOTES

There are only 4 variables, labeled A, B, C and D. They can only store integers. They usually can't store negative numbers the only way to do so is to use subtraction.
A comparison is:
- < (less than)
- <= (less than or equal to)
- = (equal to)
- != (not equal to)
- \> (greater than)
- \>= (greater than or equal to).

A jump destination is:
- NXT/NEXT (Jumps to the next instruction)
- END (Ends the program)
- A number (Jumps to the line with that number)

You can't add empty lines because of how the JMP instruction works.
Comments are added at the end of lines with the `#` symbol. Due to this, the symbol cannot be put into strings.

# Docs

`PVR <variable>` - Prints the contents of a variable. Example:
```instakod
PVR C
```
Will print the contents of the variable C.

`PTX <text>` - Prints some text. (you have to use double `"` quotes!) Example:
```instakod
PTX "Text"!!!
```
Will print the string "Text!!!"

`NLN` - Goes to a new line.
(you really don't need an example...)

`LTV <variable>` - Load user input to a variable. Example:
```instakod
LTV B
```
Will load whatever the user types into B.

`SET <variable> <variable or number>` - Sets a variable to a specific integer or the contents of another variable. Example:
```instakod
SET A B
```
Will set A to the same value as B.

`ADD <variable> <variable or number>` - Increases the value of a variable by an integer or the contents of another variable. Example:
```instakod
ADD D 2
```
Will increase D by 2.

`SUB <variable> <variable or number>` - Decreases the value of a variable by an integer or the contents of another variable. Example:
```instakod
SUB A 5
```
Will decrease A by 5.

`IFJ <variable> <comparison> <variable or number> <jump destination> <jump destination>` - Compares \<variable> with \<variable or number> using \<comparison>. If the statement is true, jump to the first \<jump destination>, otherwise, jump to the second one. Examples:
```instakod
IFJ A = 0 NEXT END
```
Will jump to the next line if A is equal to 0, otherwise, ends execution of the program.
```instakod
IFJ C >= B 13 2
```
Will jump to line 13 if C is greater than or equal to B, otherwise, jumps to line 2.

`JMP <jump destination>` - Jumps to the specified jump destination. Example:
```instakod
JMP END
```
Will end execution of the program.

# License

This project is licensed under the MIT License — see the [LICENSE](https://github.com/Pro-Haker/instakod/blob/master/LICENSE) file for details

# Ahmed

Your reward for making it this far is [AHMED!!!](https://ermitavulpe.github.io/ask-ahmed) (made by my brother)