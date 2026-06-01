# lau-bytecode

A minimal stack-based bytecode virtual machine for running agent programs — featuring a 28-instruction instruction set, a compiler with label support, sensor/actuator I/O channels, an emotional "vibe" state, and pre-built programs for fibonacci, thermostat control, and conservation checking.

> **69 tests** · zero dependencies · edition 2024

---

## What This Does

This crate provides a complete embedded scripting VM for game agents:

1. **28 opcodes** — stack manipulation (PUSH, DUP, POP, SWAP), arithmetic (ADD, SUB, MUL, DIV, NEG), local variables (LOAD, STORE), control flow (JMP, JZ, JNZ, CALL, RET), comparison (LT, GT, EQ), logic (AND, OR, NOT), I/O (SENSE, ACT, PRINT, HALT), and emotional state (VIBE_GET, VIBE_SET).
2. **Compiler** — Assembles text programs into bytecode with label resolution (`@label` syntax), comments (`#`, `;`), and error reporting.
3. **VM State** — Stack, 16 local variables, program counter, 8 sensor channels, 8 actuator channels, call stack, vibe (emotion float), tick counter, and output buffer.
4. **Pre-built Programs** — Fibonacci, thermostat (sensor-driven heater control), conservation checker.
5. **Step-by-step or batch execution** — `step()` for one instruction, `run(max_steps)` for batch, with halt/error/max-steps-exceeded detection.

---

## Key Idea

The VM is **stack-based**: operands are pushed onto a stack, operators pop their arguments and push results. This eliminates register allocation and makes compilation trivial. The VM is designed for agent scripting in game simulations:

- **Sensors** (`SENSE ch`) read from 8 input channels — feed them environment data (temperature, distance, energy).
- **Actuators** (`ACT ch`) write to 8 output channels — read them after execution to drive agent behavior.
- **Vibe** (`VIBE_GET` / `VIBE_SET`) is a single float representing the agent's emotional state — agents can read and modify their own mood.
- **Call stack** supports subroutines via `CALL addr` / `RET`.
- **Max steps** prevents infinite loops — the VM halts with an error if the budget is exceeded.

---

## Install

```toml
[dependencies]
lau-bytecode = { git = "https://github.com/SuperInstance/lau-bytecode" }
```

Zero dependencies. Edition 2024.

---

## Quick Start

### Programmatic Bytecode

```rust
use lau_bytecode::*;

let mut bc = Bytecode::new();
bc.push(Opcode::Push(3.0));
bc.push(Opcode::Push(4.0));
bc.push(Opcode::Add);
bc.push(Opcode::Print);
bc.push(Opcode::Halt);

let mut vm = VmState::new();
let result = vm.run(&bc, 100);
assert!(result.halted);
assert_eq!(result.output, vec!["7"]);
assert_eq!(result.stack_top, Some(7.0));
```

### Compile from Text

```rust
use lau_bytecode::*;

let source = "\
    PUSH 10
    PUSH 20
    ADD
    PRINT
    HALT
";
let bc = Compiler::compile(source).unwrap();

let mut vm = VmState::new();
let result = vm.run(&bc, 100);
assert_eq!(result.output, vec!["30"]);
```

### Labels and Control Flow

```rust
let source = "\
    PUSH 0
    STORE 0       ; accumulator
    PUSH 5        ; counter
loop:
    DUP
    PUSH 0
    EQ
    JNZ @done
    LOAD 0
    ADD
    STORE 0
    PUSH 1
    SUB
    JMP @loop
done:
    LOAD 0
    PRINT          ; prints 15 (5+4+3+2+1)
    HALT
";
let bc = Compiler::compile(source).unwrap();
let mut vm = VmState::new();
let result = vm.run(&bc, 1000);
assert_eq!(result.output, vec!["15"]);
```

### Sensors & Actuators (Thermostat)

```rust
let source = thermostat_program();
let bc = Compiler::compile(source).unwrap();

let mut vm = VmState::new();
vm.sensors[0] = 15.0;  // cold!
vm.run(&bc, 100);
assert_eq!(vm.actuators[0], 1.0);  // heater on

let mut vm2 = VmState::new();
vm2.sensors[0] = 25.0;  // warm
vm2.run(&bc, 100);
assert_eq!(vm2.actuators[0], 0.0);  // heater off
```

### Subroutines (CALL/RET)

```rust
let mut bc = Bytecode::new();
bc.push(Opcode::Push(10.0));  // 0
bc.push(Opcode::Call(4));     // 1: call sub at index 4
bc.push(Opcode::Print);       // 2: print result
bc.push(Opcode::Halt);        // 3
bc.push(Opcode::Push(2.0));   // 4: subroutine
bc.push(Opcode::Mul);         // 5: 10 * 2
bc.push(Opcode::Ret);         // 6: return to 2

let mut vm = VmState::new();
let result = vm.run(&bc, 10);
assert_eq!(result.output, vec!["20"]);
```

---

## API Reference

### Opcode (28 instructions)

| Category | Opcode | Operand | Description |
|---|---|---|---|
| **Stack** | `Push(f64)` | value | Push constant onto stack |
| | `Dup` | — | Duplicate top of stack |
| | `Pop` | — | Discard top of stack |
| | `Swap` | — | Swap top two elements |
| **Arithmetic** | `Add` | — | a + b |
| | `Sub` | — | a - b |
| | `Mul` | — | a × b |
| | `Div` | — | a ÷ b (error on ÷0) |
| | `Neg` | — | -a |
| **Locals** | `Load(u8)` | 0–15 | Push local[i] |
| | `Store(u8)` | 0–15 | Pop → local[i] |
| **Control** | `Jmp(i16)` | offset | Unconditional relative jump |
| | `Jz(i16)` | offset | Jump if top == 0 |
| | `Jnz(i16)` | offset | Jump if top != 0 |
| | `Call(u8)` | target | Push return addr, jump to absolute target |
| | `Ret` | — | Pop return addr (halt if empty) |
| **Comparison** | `Lt` | — | a < b → 1.0 or 0.0 |
| | `Gt` | — | a > b → 1.0 or 0.0 |
| | `Eq` | — | a ≈ b → 1.0 or 0.0 |
| **Logic** | `And` | — | a ≠ 0 ∧ b ≠ 0 → 1.0 or 0.0 |
| | `Or` | — | a ≠ 0 ∨ b ≠ 0 → 1.0 or 0.0 |
| | `Not` | — | a == 0 → 1.0 or 0.0 |
| **I/O** | `Print` | — | Pop and append to output |
| | `Halt` | — | Stop execution |
| **Agent** | `VibeGet` | — | Push vibe value |
| | `VibeSet` | — | Pop → vibe |
| | `Sense(u8)` | 0–7 | Push sensor[ch] |
| | `Act(u8)` | 0–7 | Pop → actuator[ch] |

### Bytecode

| Method | Description |
|---|---|
| `new()` | Create empty bytecode |
| `push(opcode)` | Append instruction |
| `disassemble()` | Pretty-print listing |
| `len()` / `is_empty()` | Instruction count |

### VmState

| Field | Type | Description |
|---|---|---|
| `stack` | `Vec<f64>` | Operand stack |
| `locals` | `[f64; 16]` | Local variables |
| `pc` | `usize` | Program counter |
| `vibe` | `f64` | Emotional state |
| `sensors` | `[f64; 8]` | Input channels |
| `actuators` | `[f64; 8]` | Output channels |
| `call_stack` | `Vec<usize>` | Return addresses |
| `halted` | `bool` | Halted flag |
| `tick` | `u64` | Instructions executed |
| `output` | `Vec<String>` | Print output |

| Method | Description |
|---|---|
| `new()` | Create fresh state |
| `reset()` | Clear all state |
| `step(bytecode)` | Execute one instruction → `StepResult` |
| `run(bytecode, max_steps)` | Run up to N steps → `VmResult` |

### VmResult

| Field | Description |
|---|---|
| `steps` | Instructions executed |
| `halted` | Did it reach HALT? |
| `stack_top` | Top of stack (if any) |
| `output` | All PRINT output |
| `error` | Error message (if any) |

### StepResult

| Variant | Meaning |
|---|---|
| `Ok` | Instruction succeeded |
| `Halted` | HALT reached or past end |
| `Error(msg)` | Stack underflow, div by zero, bounds, etc. |

### Compiler

| Method | Description |
|---|---|
| `compile(source)` | Parse text → `Result<Bytecode, String>` |

Supports labels (`name:`), label references (`@name`), comments (`#`, `;`), and all opcodes. Errors include line numbers.

### Pre-built Programs

| Function | Description |
|---|---|
| `fibonacci_program()` | Computes fib(n) where n is on the stack |
| `thermostat_program()` | Reads sensor[0] (temp), sets actuator[0] (heater) based on threshold 20.0 |
| `conservation_program()` | Sums sensors[0..2], prints deviation from 100 |

---

## How It Works

### Stack Architecture

The VM uses a **data stack** for all computation:

```
PUSH 3   →  stack: [3]
PUSH 4   →  stack: [3, 4]
ADD      →  pops 4, 3 → pushes 7 → stack: [7]
PRINT    →  pops 7 → output: ["7"] → stack: []
```

All arithmetic pops `b` first, then `a` (stack order), and pushes the result: `a op b`.

### Control Flow

- **JMP/JZ/JNZ**: Use **relative offsets** from the current instruction. Positive = forward, negative = backward. `JZ`/`JNZ` pop one value and conditionally jump.
- **CALL/RET**: `CALL target` is **absolute** (index into instruction array). It pushes `pc + 1` onto the call stack and jumps. `RET` pops the return address. If the call stack is empty, `RET` halts (return from main).

### Compiler Label Resolution

1. **First pass**: Scan for labels (`name:`), record their instruction indices.
2. **Second pass** (inline): For each `JMP @label`, `JZ @label`, `JNZ @label`, compute the offset: `target_index - instruction_index`. For `CALL @label`, use the absolute index.

Labels are case-insensitive.

### Sensor/Actuator Model

Before running an agent program, set `vm.sensors[i]` to environment values. After execution, read `vm.actuators[i]` to get the agent's actions. This is a pull-model: the agent reads sensors synchronously and writes to actuators synchronously.

### Vibe State

`VIBE_GET` pushes the current vibe onto the stack. `VIBE_SET` pops a value and sets vibe. This is a single float that agents can use to modulate behavior — e.g., high vibe could make an agent more exploratory, low vibe more conservative.

### Error Handling

The VM detects and reports:
- **Stack underflow**: any pop from an empty stack.
- **Division by zero**: `DIV` with b = 0.
- **Local out of bounds**: `LOAD`/`STORE` index ≥ 16.
- **Sensor/actuator out of bounds**: channel ≥ 8.
- **Jump out of bounds**: target ≥ program length.
- **Call target out of bounds**: target ≥ program length.
- **Max steps exceeded**: safety limit for infinite loop prevention.

All errors include the program counter position for debugging.

---

## The Math

### Stack Semantics

For a binary operator ⊙:

$$\text{stack} = [\ldots, a, b] \xrightarrow{\odot} [\ldots, a \odot b]$$

For a unary operator ⊙:

$$\text{stack} = [\ldots, a] \xrightarrow{\odot} [\ldots, \odot a]$$

### Jump Arithmetic

Relative jumps:

$$pc_{new} = pc_{current} + \text{offset}$$

Label resolution at compile time:

$$\text{offset} = \text{label\_index} - \text{instruction\_index}$$

### Fibonacci (Pre-built Program)

Computes fib(n) iteratively:

$$F(0) = 0, \quad F(n) = F(n-1) + F(n-2)$$

The program uses local[0] as accumulator and local[1] as counter, decrementing until zero.

### Thermostat (Pre-built Program)

$$\text{heater} = \begin{cases} 1 & \text{if } \text{sensor}[0] \leq 20.0 \\ 0 & \text{otherwise} \end{cases}$$

### Conservation (Pre-built Program)

$$\text{deviation} = 100.0 - (\text{sensor}[0] + \text{sensor}[1] + \text{sensor}[2])$$

Prints the deviation from a conservation target of 100.

### Opcode Discriminants

Opcodes have numeric discriminants organized by category:

| Range | Category |
|---|---|
| 0x01–0x04 | Stack ops |
| 0x10–0x14 | Arithmetic |
| 0x20–0x21 | Locals |
| 0x30–0x34 | Control flow |
| 0x40–0x42 | Comparison |
| 0x50–0x52 | Logic |
| 0x60 | Print |
| 0x70–0x71 | Vibe |
| 0x80–0x81 | I/O |
| 0xFF | Halt |

---

## License

MIT or Apache-2.0 (at your option).
