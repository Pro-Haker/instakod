PTX "Enter a seed: "
LTV A
SET B 878878148 # some random bs to hash with
SET C 194857920 # some random bs to hash with
SET D 100000
SUB A B
ADD C B
SUB B C
SUB C B
ADD B A
SUB A B
SUB B A
SUB D 1
IFJ D = 0 NXT 6
PVR C
