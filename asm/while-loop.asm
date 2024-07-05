# this is a while loop
# it looks like the following high level form:
# let x = 0;
# while (x < 5) {
#   print(x);
#   x += 1;
# }
putreg 0 R0
putreg 1 R1
putreg 5 R2
lte R2 R0
jumptrue 3
printreg R0
add R1 R0
jump -5
putreg 0 R0
ret
