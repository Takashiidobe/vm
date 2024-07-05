# VM

This project implements a small VM language that supports encoding to
binary and decoding from binary to run a program.

## Usage

Instructions can be written in an assembly form, encoded to binary, and
run from the binary form.

Take this file, which counts from 0 to 4:

```sh
$ cat asm/while-loop.asm
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
```

We can encode this into binary:

```sh
$ cargo r -q -- -e asm/while-loop.asm out.bin
```

And `xxd` it to see its binary form:

```sh
$ xxd out.bin
00000000: 0100 0000 0101 0001 0105 0002 1300 0211  ................
00000010: 0300 0900 0501 0010 fbff 0100 0000 00    ...............
```

We can then run this binary in the VM:

```sh
$ cargo r -q -- -d out.bin
0
1
2
3
4
```

It's also possible to run the assembly directly on the VM:

```sh
$ cargo r -q -- -r asm/while-loop.asm
0
1
2
3
4
```

## VM Internals

This project implements a VM with 16 registers (`R0..R16`), and a stack
of 65536 two-byte words. There is also a condition flag, which is used
for jumping back and forth, and an instruction pointer to keep track of
which instruction is currently used.

The language currently supports a few instructions, which take either
Immediates (currently, a `u16`), a stack pos (a `u16`), which indexes
into the stack, an offset (an `i16`), to jump back and forth in the
instructions, or a Register, a `u8` that indicates which register to
use.

```
Ret,                    // Return R0
PutReg(Immediate, Reg), // Put u16 -> Reg
CopySR(StackPos, Reg),  // Load Stack -> Reg
CopyRR(Reg, Reg),       // Copy Reg -> Reg
CopyRS(Reg, StackPos),  // Copy Reg -> Stack
Add(Reg, Reg),          // Add R1, R2 -> R2
Sub(Reg, Reg),          // Sub R1, R2 -> R2
Mul(Reg, Reg),          // Mul R1, R2 -> R2
Div(Reg, Reg),          // Div R1, R2 -> R2
PrintReg(Reg),          // Print Reg
Jump(Offset),           // Jump Forward or backward
JumpTrue(Offset),       // Jump Forward or backwards if status is true
JumpFalse(Offset),      // Jump Forward or backwards if status is false
Cmp(Reg, Reg),          // Compare R1 to R2, setting the condition flag.
```

There's currently no way to load immediates onto the stack, so a load to
the stack first requires loading an immediate to a register and then
moving that to the stack:

Take for example, this program:

```rust
let x = [1, 2, 3];
print(x);
```

That would be converted into this code: (note loading each immediate
into a register before moving that register into the stack):

```
PutReg(1, R0)
CopyRS(R0, 0)
PutReg(2, R0)
CopyRS(R0, 1)
PutReg(3, R0)
CopyRS(R0, 2)
CopySR(0, R0)
PrintReg(R0)
CopySR(1, R0)
PrintReg(R0)
CopySR(2, R0)
PrintReg(R0)
PutReg(0, R0)
Ret
```

## Encoding and Decoding

Each instruction is encoded into bytes:

Take these instructions:

```
PutReg(20, R0),
PutReg(20, R1),
Cmp(R0, R1),
JumpFalse(3),
PutReg(0, R0),
PrintReg(R0),
Jump(2),
PutReg(1, R0),
PrintReg(R0),
PutReg(0, R0),
Ret
```

That would be encoded into these bytes:

```
PutReg(20, R0): [0x01,0x14,0x00,0x00]
PutReg(20, R1): [0x01,0x14,0x00,0x01]
Cmp(R0, R1): [0x13,0x00,0x01]
JumpFalse(3): [0x12,0x03,0x00]
PutReg(0, R0): [0x01,0x00,0x00,0x00]
PrintReg(R0): [0x09,0x00]
Jump(2): [0x10,0x02,0x00]
PutReg(1, R0): [0x01,0x01,0x00,0x00]
PrintReg(R0): [0x09,0x00]
PutReg(0, R0): [0x01,0x00,0x00,0x00]
Ret: [0x00]
```

Each instruction gets a unique byte: `Ret`, for example, is given the
value `0x0`, and `PutReg` is given `0x01`. This is the first byte in
every encoded instruction. Each instruction then either takes zero, one,
or two extra arguments, where the arguments can be one or two bytes
long, and be signed or unsigned.

For example, `PrintReg`, instruction `0x09` takes one argument, the
register to print. Since there are only 16 registers, this is a one byte
instruction. In the example, this prints `R0`, which in bytes is `0x00`.

Another instruction, `Jump`, `0x10`, will take one argument, but it
takes an offset, which is signed and 2-bytes. In this example, it is
denoted by the bytes `0x02, 0x00`, which will be converted to 2 when run
by the VM.

These instructions can thus be serialized in a compact form on disc and
turned into instructions, which can then be run by the VM.

## Testing

Currently, there are some property tests using `quickcheck` to generate
arbitrary programs and then confirming that those instructions, when
encoded to disk and decoded, still return the same program.
