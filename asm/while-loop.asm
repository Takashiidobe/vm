putreg 0 R0
putreg 1 R1
putreg 5 R2
cmp R0 R2
jumptrue 3
printreg R0
add R1 R0
jump -5
putreg 0 R0
ret
