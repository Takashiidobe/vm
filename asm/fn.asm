fn add
add R1 R0
retfn
putreg 10 R0
putreg 20 R1
call add
printreg R0
putreg 0 R0
ret
