# lau-bytecode

> Part of the PLATO/LAU mathematical agent framework

## What This Does

Part of the PLATO/LAU mathematical agent framework. Part of the PLATO/LAU ecosystem — a mathematically rigorous framework for building educational agents that learn, teach, and evolve.

## The Key Idea

This crate implements the core abstractions needed for its domain, with a focus on correctness, composability, and conservation guarantees. Every public type is serializable (serde), every algorithm is tested, and every invariant is verified.

## Install

```bash
cargo add lau-bytecode
```

## Quick Start

See the API Reference below for complete usage. Key entry points:

```rust
use lau_bytecode::*;
// See types and methods below for complete usage
```

## API Reference

```rust
pub enum Opcode 
    pub fn discriminant(&self) -> u8 
pub struct Instruction 
    pub fn new(opcode: Opcode, line: u32) -> Self 
pub struct Bytecode 
    pub fn new() -> Self 
    pub fn push(&mut self, op: Opcode) 
    pub fn disassemble(&self) -> Vec<String> 
    pub fn len(&self) -> usize 
    pub fn is_empty(&self) -> bool 
pub struct VmResult 
pub enum StepResult 
pub struct VmState 
    pub fn new() -> Self 
    pub fn reset(&mut self) 
    pub fn run(&mut self, bytecode: &Bytecode, max_steps: usize) -> VmResult 
    pub fn step(&mut self, bytecode: &Bytecode) -> StepResult 
pub struct Compiler;
    pub fn compile(source: &str) -> Result<Bytecode, String> 
pub fn fibonacci_program() -> &'static str 
pub fn thermostat_program() -> &'static str 
pub fn conservation_program() -> &'static str 
```

## How It Works

Read the source in `src/` for full implementation details. All algorithms are documented with inline comments explaining the mathematical foundations.

## The Math

This crate implements formal mathematical constructs. See the source documentation for theorem statements and proofs of correctness.

## Testing

**69 tests** covering construction, serialization, correctness properties, edge cases, and composability with other lau-* crates.

## License

MIT
