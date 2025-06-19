PTX "Enter the first number: "
LTV A
PTX "Enter the second number: "
LTV B
IFJ A = B 11 NEXT
IFJ A > B 9 NEXT
SUB B A
JMP 5
SUB A B
JMP 5
PTX "The greatest common denominator is "
PVR A
