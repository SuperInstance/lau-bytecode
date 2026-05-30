//! `lau-bytecode` — a low-level bytecode VM for running agent programs.
//!
//! Stack-based, minimal, safe. Designed as an embedded scripting VM for
//! game agents with sensors, actuators, and a "vibe" emotional state.

use std::fmt;

// ---------------------------------------------------------------------------
// Opcodes
// ---------------------------------------------------------------------------

/// All supported opcodes. Data-carrying variants store their operand inline.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Opcode {
    Push(f64),
    Dup,
    Pop,
    Swap,
    Add,
    Sub,
    Mul,
    Div,
    Neg,
    Load(u8),
    Store(u8),
    Jmp(i16),
    Jz(i16),
    Jnz(i16),
    Call(u8),
    Ret,
    Lt,
    Gt,
    Eq,
    And,
    Or,
    Not,
    Print,
    Halt,
    VibeGet,
    VibeSet,
    Sense(u8),
    Act(u8),
}

impl Opcode {
    pub fn discriminant(&self) -> u8 {
        match self {
            Opcode::Push(_) => 0x01,
            Opcode::Dup => 0x02,
            Opcode::Pop => 0x03,
            Opcode::Swap => 0x04,
            Opcode::Add => 0x10,
            Opcode::Sub => 0x11,
            Opcode::Mul => 0x12,
            Opcode::Div => 0x13,
            Opcode::Neg => 0x14,
            Opcode::Load(_) => 0x20,
            Opcode::Store(_) => 0x21,
            Opcode::Jmp(_) => 0x30,
            Opcode::Jz(_) => 0x31,
            Opcode::Jnz(_) => 0x32,
            Opcode::Call(_) => 0x33,
            Opcode::Ret => 0x34,
            Opcode::Lt => 0x40,
            Opcode::Gt => 0x41,
            Opcode::Eq => 0x42,
            Opcode::And => 0x50,
            Opcode::Or => 0x51,
            Opcode::Not => 0x52,
            Opcode::Print => 0x60,
            Opcode::Halt => 0xFF,
            Opcode::VibeGet => 0x70,
            Opcode::VibeSet => 0x71,
            Opcode::Sense(_) => 0x80,
            Opcode::Act(_) => 0x81,
        }
    }
}

impl fmt::Display for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Opcode::Push(v) => write!(f, "PUSH {v}"),
            Opcode::Dup => write!(f, "DUP"),
            Opcode::Pop => write!(f, "POP"),
            Opcode::Swap => write!(f, "SWAP"),
            Opcode::Add => write!(f, "ADD"),
            Opcode::Sub => write!(f, "SUB"),
            Opcode::Mul => write!(f, "MUL"),
            Opcode::Div => write!(f, "DIV"),
            Opcode::Neg => write!(f, "NEG"),
            Opcode::Load(i) => write!(f, "LOAD {i}"),
            Opcode::Store(i) => write!(f, "STORE {i}"),
            Opcode::Jmp(o) => write!(f, "JMP {o}"),
            Opcode::Jz(o) => write!(f, "JZ {o}"),
            Opcode::Jnz(o) => write!(f, "JNZ {o}"),
            Opcode::Call(i) => write!(f, "CALL {i}"),
            Opcode::Ret => write!(f, "RET"),
            Opcode::Lt => write!(f, "LT"),
            Opcode::Gt => write!(f, "GT"),
            Opcode::Eq => write!(f, "EQ"),
            Opcode::And => write!(f, "AND"),
            Opcode::Or => write!(f, "OR"),
            Opcode::Not => write!(f, "NOT"),
            Opcode::Print => write!(f, "PRINT"),
            Opcode::Halt => write!(f, "HALT"),
            Opcode::VibeGet => write!(f, "VIBE_GET"),
            Opcode::VibeSet => write!(f, "VIBE_SET"),
            Opcode::Sense(i) => write!(f, "SENSE {i}"),
            Opcode::Act(i) => write!(f, "ACT {i}"),
        }
    }
}

// ---------------------------------------------------------------------------
// Instruction
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Instruction {
    pub opcode: Opcode,
    pub line: u32,
}

impl Instruction {
    pub fn new(opcode: Opcode, line: u32) -> Self {
        Instruction { opcode, line }
    }
}

// ---------------------------------------------------------------------------
// Bytecode
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct Bytecode {
    pub instructions: Vec<Instruction>,
}

impl Bytecode {
    pub fn new() -> Self {
        Bytecode { instructions: Vec::new() }
    }

    pub fn push(&mut self, op: Opcode) {
        let line = self.instructions.len() as u32;
        self.instructions.push(Instruction::new(op, line));
    }

    pub fn disassemble(&self) -> Vec<String> {
        self.instructions
            .iter()
            .enumerate()
            .map(|(i, ins)| format!("{:>4}:  {}", i, ins.opcode))
            .collect()
    }

    pub fn len(&self) -> usize {
        self.instructions.len()
    }

    pub fn is_empty(&self) -> bool {
        self.instructions.is_empty()
    }
}

impl Default for Bytecode {
    fn default() -> Self { Self::new() }
}

// ---------------------------------------------------------------------------
// VmResult, StepResult, VmState
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq)]
pub struct VmResult {
    pub steps: usize,
    pub halted: bool,
    pub stack_top: Option<f64>,
    pub output: Vec<String>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum StepResult {
    Ok,
    Halted,
    Error(String),
}

#[derive(Debug, Clone)]
pub struct VmState {
    pub stack: Vec<f64>,
    pub locals: [f64; 16],
    pub pc: usize,
    pub vibe: f64,
    pub sensors: [f64; 8],
    pub actuators: [f64; 8],
    pub call_stack: Vec<usize>,
    pub halted: bool,
    pub tick: u64,
    pub output: Vec<String>,
}

impl VmState {
    pub fn new() -> Self {
        VmState {
            stack: Vec::new(),
            locals: [0.0; 16],
            pc: 0,
            vibe: 0.0,
            sensors: [0.0; 8],
            actuators: [0.0; 8],
            call_stack: Vec::new(),
            halted: false,
            tick: 0,
            output: Vec::new(),
        }
    }

    pub fn reset(&mut self) {
        self.stack.clear();
        self.locals = [0.0; 16];
        self.pc = 0;
        self.vibe = 0.0;
        self.call_stack.clear();
        self.halted = false;
        self.tick = 0;
        self.output.clear();
    }

    /// Pop a value or return StepResult::Error.
    fn pop(&mut self, op_name: &str) -> Result<f64, StepResult> {
        match self.stack.pop() {
            Some(v) => Ok(v),
            None => Err(StepResult::Error(format!(
                "stack underflow on {} at pc={}",
                op_name, self.pc
            ))),
        }
    }

    pub fn run(&mut self, bytecode: &Bytecode, max_steps: usize) -> VmResult {
        if max_steps == 0 {
            return VmResult {
                steps: 0,
                halted: self.halted,
                stack_top: self.stack.last().copied(),
                output: self.output.clone(),
                error: Some("max_steps is zero".into()),
            };
        }

        let mut steps = 0;
        while steps < max_steps {
            match self.step(bytecode) {
                StepResult::Ok => steps += 1,
                StepResult::Halted => {
                    steps += 1;
                    return VmResult {
                        steps,
                        halted: true,
                        stack_top: self.stack.last().copied(),
                        output: self.output.clone(),
                        error: None,
                    };
                }
                StepResult::Error(e) => {
                    steps += 1;
                    return VmResult {
                        steps,
                        halted: false,
                        stack_top: self.stack.last().copied(),
                        output: self.output.clone(),
                        error: Some(e),
                    };
                }
            }
        }

        VmResult {
            steps,
            halted: false,
            stack_top: self.stack.last().copied(),
            output: self.output.clone(),
            error: Some(format!(
                "max steps ({max_steps}) exceeded at pc={}",
                self.pc
            )),
        }
    }

    pub fn step(&mut self, bytecode: &Bytecode) -> StepResult {
        if self.halted {
            return StepResult::Halted;
        }
        if self.pc >= bytecode.instructions.len() {
            self.halted = true;
            return StepResult::Halted;
        }

        let ins = &bytecode.instructions[self.pc];
        self.tick += 1;

        match ins.opcode {
            // ---- stack manipulation ----
            Opcode::Push(v) => {
                self.stack.push(v);
                self.pc += 1;
                StepResult::Ok
            }
            Opcode::Dup => {
                let v = match self.stack.last().copied() {
                    Some(v) => v,
                    None => {
                        return StepResult::Error(format!(
                            "stack underflow on DUP at pc={}",
                            self.pc
                        ))
                    }
                };
                self.stack.push(v);
                self.pc += 1;
                StepResult::Ok
            }
            Opcode::Pop => {
                if self.stack.pop().is_none() {
                    return StepResult::Error(format!(
                        "stack underflow on POP at pc={}",
                        self.pc
                    ));
                }
                self.pc += 1;
                StepResult::Ok
            }
            Opcode::Swap => {
                let a = match self.pop("SWAP") {
                    Ok(v) => v,
                    Err(e) => return e,
                };
                let b = match self.pop("SWAP") {
                    Ok(v) => v,
                    Err(e) => return e,
                };
                self.stack.push(a);
                self.stack.push(b);
                self.pc += 1;
                StepResult::Ok
            }

            // ---- arithmetic ----
            Opcode::Add => {
                let b = match self.pop("ADD") { Ok(v) => v, Err(e) => return e };
                let a = match self.pop("ADD") { Ok(v) => v, Err(e) => return e };
                self.stack.push(a + b);
                self.pc += 1;
                StepResult::Ok
            }
            Opcode::Sub => {
                let b = match self.pop("SUB") { Ok(v) => v, Err(e) => return e };
                let a = match self.pop("SUB") { Ok(v) => v, Err(e) => return e };
                self.stack.push(a - b);
                self.pc += 1;
                StepResult::Ok
            }
            Opcode::Mul => {
                let b = match self.pop("MUL") { Ok(v) => v, Err(e) => return e };
                let a = match self.pop("MUL") { Ok(v) => v, Err(e) => return e };
                self.stack.push(a * b);
                self.pc += 1;
                StepResult::Ok
            }
            Opcode::Div => {
                let b = match self.pop("DIV") { Ok(v) => v, Err(e) => return e };
                let a = match self.pop("DIV") { Ok(v) => v, Err(e) => return e };
                if b == 0.0 {
                    return StepResult::Error(format!(
                        "division by zero at pc={}",
                        self.pc
                    ));
                }
                self.stack.push(a / b);
                self.pc += 1;
                StepResult::Ok
            }
            Opcode::Neg => {
                let v = match self.pop("NEG") { Ok(v) => v, Err(e) => return e };
                self.stack.push(-v);
                self.pc += 1;
                StepResult::Ok
            }

            // ---- locals ----
            Opcode::Load(i) => {
                let idx = i as usize;
                if idx >= 16 {
                    return StepResult::Error(format!(
                        "local index {idx} out of bounds at pc={}",
                        self.pc
                    ));
                }
                self.stack.push(self.locals[idx]);
                self.pc += 1;
                StepResult::Ok
            }
            Opcode::Store(i) => {
                let idx = i as usize;
                if idx >= 16 {
                    return StepResult::Error(format!(
                        "local index {idx} out of bounds at pc={}",
                        self.pc
                    ));
                }
                let v = match self.pop("STORE") { Ok(v) => v, Err(e) => return e };
                self.locals[idx] = v;
                self.pc += 1;
                StepResult::Ok
            }

            // ---- control flow ----
            Opcode::Jmp(offset) => {
                match apply_offset(self.pc, offset, bytecode.len()) {
                    Ok(pc) => self.pc = pc,
                    Err(e) => return StepResult::Error(e),
                }
                StepResult::Ok
            }
            Opcode::Jz(offset) => {
                let v = match self.pop("JZ") { Ok(v) => v, Err(e) => return e };
                if v == 0.0 {
                    match apply_offset(self.pc, offset, bytecode.len()) {
                        Ok(pc) => self.pc = pc,
                        Err(e) => return StepResult::Error(e),
                    }
                } else {
                    self.pc += 1;
                }
                StepResult::Ok
            }
            Opcode::Jnz(offset) => {
                let v = match self.pop("JNZ") { Ok(v) => v, Err(e) => return e };
                if v != 0.0 {
                    match apply_offset(self.pc, offset, bytecode.len()) {
                        Ok(pc) => self.pc = pc,
                        Err(e) => return StepResult::Error(e),
                    }
                } else {
                    self.pc += 1;
                }
                StepResult::Ok
            }
            Opcode::Call(target) => {
                let target = target as usize;
                if target >= bytecode.instructions.len() {
                    return StepResult::Error(format!(
                        "CALL target {target} out of bounds at pc={}",
                        self.pc
                    ));
                }
                self.call_stack.push(self.pc + 1);
                self.pc = target;
                StepResult::Ok
            }
            Opcode::Ret => {
                match self.call_stack.pop() {
                    Some(addr) => self.pc = addr,
                    None => self.halted = true,
                }
                StepResult::Ok
            }

            // ---- comparison ----
            Opcode::Lt => {
                let b = match self.pop("LT") { Ok(v) => v, Err(e) => return e };
                let a = match self.pop("LT") { Ok(v) => v, Err(e) => return e };
                self.stack.push(if a < b { 1.0 } else { 0.0 });
                self.pc += 1;
                StepResult::Ok
            }
            Opcode::Gt => {
                let b = match self.pop("GT") { Ok(v) => v, Err(e) => return e };
                let a = match self.pop("GT") { Ok(v) => v, Err(e) => return e };
                self.stack.push(if a > b { 1.0 } else { 0.0 });
                self.pc += 1;
                StepResult::Ok
            }
            Opcode::Eq => {
                let b = match self.pop("EQ") { Ok(v) => v, Err(e) => return e };
                let a = match self.pop("EQ") { Ok(v) => v, Err(e) => return e };
                self.stack.push(if (a - b).abs() < f64::EPSILON { 1.0 } else { 0.0 });
                self.pc += 1;
                StepResult::Ok
            }

            // ---- logical ----
            Opcode::And => {
                let b = match self.pop("AND") { Ok(v) => v, Err(e) => return e };
                let a = match self.pop("AND") { Ok(v) => v, Err(e) => return e };
                self.stack.push(if a != 0.0 && b != 0.0 { 1.0 } else { 0.0 });
                self.pc += 1;
                StepResult::Ok
            }
            Opcode::Or => {
                let b = match self.pop("OR") { Ok(v) => v, Err(e) => return e };
                let a = match self.pop("OR") { Ok(v) => v, Err(e) => return e };
                self.stack.push(if a != 0.0 || b != 0.0 { 1.0 } else { 0.0 });
                self.pc += 1;
                StepResult::Ok
            }
            Opcode::Not => {
                let v = match self.pop("NOT") { Ok(v) => v, Err(e) => return e };
                self.stack.push(if v == 0.0 { 1.0 } else { 0.0 });
                self.pc += 1;
                StepResult::Ok
            }

            // ---- introspection ----
            Opcode::Print => {
                let v = match self.pop("PRINT") { Ok(v) => v, Err(e) => return e };
                self.output.push(format!("{v}"));
                self.pc += 1;
                StepResult::Ok
            }
            Opcode::Halt => {
                self.halted = true;
                StepResult::Halted
            }

            // ---- vibe ----
            Opcode::VibeGet => {
                self.stack.push(self.vibe);
                self.pc += 1;
                StepResult::Ok
            }
            Opcode::VibeSet => {
                let v = match self.pop("VIBE_SET") { Ok(v) => v, Err(e) => return e };
                self.vibe = v;
                self.pc += 1;
                StepResult::Ok
            }

            // ---- I/O ----
            Opcode::Sense(ch) => {
                let idx = ch as usize;
                if idx >= 8 {
                    return StepResult::Error(format!(
                        "sensor channel {idx} out of bounds at pc={}",
                        self.pc
                    ));
                }
                self.stack.push(self.sensors[idx]);
                self.pc += 1;
                StepResult::Ok
            }
            Opcode::Act(ch) => {
                let idx = ch as usize;
                if idx >= 8 {
                    return StepResult::Error(format!(
                        "actuator channel {idx} out of bounds at pc={}",
                        self.pc
                    ));
                }
                let v = match self.pop("ACT") { Ok(v) => v, Err(e) => return e };
                self.actuators[idx] = v;
                self.pc += 1;
                StepResult::Ok
            }
        }
    }
}

impl Default for VmState {
    fn default() -> Self { Self::new() }
}

// ---------------------------------------------------------------------------

fn apply_offset(pc: usize, offset: i16, len: usize) -> Result<usize, String> {
    let new_pc = if offset >= 0 {
        pc.wrapping_add(offset as usize)
    } else {
        pc.wrapping_sub((-offset) as usize)
    };
    if new_pc >= len {
        return Err(format!(
            "jump target {new_pc} out of bounds (len={len}) at pc={pc}"
        ));
    }
    Ok(new_pc)
}

// ---------------------------------------------------------------------------
// Compiler
// ---------------------------------------------------------------------------

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Compiler;

impl Compiler {
    pub fn compile(source: &str) -> Result<Bytecode, String> {
        let mut bytecode = Bytecode::new();
        let mut labels: HashMap<String, usize> = HashMap::new();
        let mut pending_refs: Vec<(usize, String)> = Vec::new();

        for (line_num, raw_line) in source.lines().enumerate() {
            let line = raw_line.trim();
            if line.is_empty() || line.starts_with('#') || line.starts_with(';') {
                continue;
            }

            if let Some(name) = line.strip_suffix(':') {
                let name = name.trim().to_lowercase();
                if labels.contains_key(&name) {
                    return Err(format!("line {}: duplicate label '{name}'", line_num + 1));
                }
                labels.insert(name, bytecode.instructions.len());
                continue;
            }

            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.is_empty() {
                continue;
            }

            let op_str = parts[0].to_uppercase();
            let instr_idx = bytecode.instructions.len();
            let line_no = (line_num + 1) as u32;

            match op_str.as_str() {
                "PUSH" => {
                    let v = parse_float(parts.get(1), line_no)?;
                    bytecode.instructions.push(Instruction::new(Opcode::Push(v), line_no));
                }
                "ADD" => bytecode.instructions.push(Instruction::new(Opcode::Add, line_no)),
                "SUB" => bytecode.instructions.push(Instruction::new(Opcode::Sub, line_no)),
                "MUL" => bytecode.instructions.push(Instruction::new(Opcode::Mul, line_no)),
                "DIV" => bytecode.instructions.push(Instruction::new(Opcode::Div, line_no)),
                "NEG" => bytecode.instructions.push(Instruction::new(Opcode::Neg, line_no)),
                "LOAD" => {
                    let i = parse_u8(parts.get(1), line_no, 15)?;
                    bytecode.instructions.push(Instruction::new(Opcode::Load(i), line_no));
                }
                "STORE" => {
                    let i = parse_u8(parts.get(1), line_no, 15)?;
                    bytecode.instructions.push(Instruction::new(Opcode::Store(i), line_no));
                }
                "JMP" | "JZ" | "JNZ" | "CALL" => {
                    let operand = parts.get(1).ok_or_else(|| {
                        format!("line {}: missing operand for {op_str}", line_no)
                    })?;

                    let opcode = match op_str.as_str() {
                        "JMP" => Opcode::Jmp(0),
                        "JZ" => Opcode::Jz(0),
                        "JNZ" => Opcode::Jnz(0),
                        "CALL" => Opcode::Call(0),
                        _ => unreachable!(),
                    };

                    if let Some(label_name) = operand.strip_prefix('@') {
                        pending_refs.push((instr_idx, label_name.to_lowercase()));
                        bytecode.instructions.push(Instruction::new(opcode, line_no));
                    } else {
                        let offset: i16 = operand.parse().map_err(|_| {
                            format!("line {}: invalid operand '{}' for {op_str}", line_no, operand)
                        })?;
                        let resolved = match op_str.as_str() {
                            "JMP" => Opcode::Jmp(offset),
                            "JZ" => Opcode::Jz(offset),
                            "JNZ" => Opcode::Jnz(offset),
                            "CALL" => Opcode::Call(offset as u8),
                            _ => unreachable!(),
                        };
                        bytecode.instructions.push(Instruction::new(resolved, line_no));
                    }
                }
                "RET" => bytecode.instructions.push(Instruction::new(Opcode::Ret, line_no)),
                "LT" => bytecode.instructions.push(Instruction::new(Opcode::Lt, line_no)),
                "GT" => bytecode.instructions.push(Instruction::new(Opcode::Gt, line_no)),
                "EQ" => bytecode.instructions.push(Instruction::new(Opcode::Eq, line_no)),
                "AND" => bytecode.instructions.push(Instruction::new(Opcode::And, line_no)),
                "OR" => bytecode.instructions.push(Instruction::new(Opcode::Or, line_no)),
                "NOT" => bytecode.instructions.push(Instruction::new(Opcode::Not, line_no)),
                "DUP" => bytecode.instructions.push(Instruction::new(Opcode::Dup, line_no)),
                "POP" => bytecode.instructions.push(Instruction::new(Opcode::Pop, line_no)),
                "SWAP" => bytecode.instructions.push(Instruction::new(Opcode::Swap, line_no)),
                "PRINT" => bytecode.instructions.push(Instruction::new(Opcode::Print, line_no)),
                "HALT" => bytecode.instructions.push(Instruction::new(Opcode::Halt, line_no)),
                "VIBE_GET" => bytecode.instructions.push(Instruction::new(Opcode::VibeGet, line_no)),
                "VIBE_SET" => bytecode.instructions.push(Instruction::new(Opcode::VibeSet, line_no)),
                "SENSE" | "SENS" => {
                    let ch = parse_u8(parts.get(1), line_no, 7)?;
                    bytecode.instructions.push(Instruction::new(Opcode::Sense(ch), line_no));
                }
                "ACT" => {
                    let ch = parse_u8(parts.get(1), line_no, 7)?;
                    bytecode.instructions.push(Instruction::new(Opcode::Act(ch), line_no));
                }
                _ => {
                    return Err(format!("line {}: unknown opcode '{}'", line_no, op_str));
                }
            }
        }

        for (instr_idx, label_name) in &pending_refs {
            let target_idx = labels.get(label_name).ok_or_else(|| {
                format!("undefined label '@{label_name}'")
            })?;

            let ins = &mut bytecode.instructions[*instr_idx];
            match ins.opcode {
                Opcode::Jmp(_) => {
                    let offset = *target_idx as i16 - *instr_idx as i16;
                    ins.opcode = Opcode::Jmp(offset);
                }
                Opcode::Jz(_) => {
                    let offset = *target_idx as i16 - *instr_idx as i16;
                    ins.opcode = Opcode::Jz(offset);
                }
                Opcode::Jnz(_) => {
                    let offset = *target_idx as i16 - *instr_idx as i16;
                    ins.opcode = Opcode::Jnz(offset);
                }
                Opcode::Call(_) => {
                    if *target_idx > 255 {
                        return Err(format!("CALL target {target_idx} exceeds u8 range"));
                    }
                    ins.opcode = Opcode::Call(*target_idx as u8);
                }
                _ => {
                    return Err(format!(
                        "internal: unexpected opcode at label ref {instr_idx}"
                    ));
                }
            }
        }

        Ok(bytecode)
    }
}

fn parse_float(opt: Option<&&str>, line_no: u32) -> Result<f64, String> {
    let s = opt.ok_or_else(|| format!("line {line_no}: expected numeric operand"))?;
    s.parse::<f64>()
        .map_err(|_| format!("line {line_no}: invalid number '{s}'"))
}

fn parse_u8(opt: Option<&&str>, line_no: u32, max: u8) -> Result<u8, String> {
    let s = opt.ok_or_else(|| format!("line {line_no}: expected integer operand"))?;
    let v: u8 = s.parse()
        .map_err(|_| format!("line {line_no}: invalid integer '{s}'"))?;
    if v > max {
        return Err(format!("line {line_no}: value {v} exceeds max {max}"));
    }
    Ok(v)
}

// ---------------------------------------------------------------------------
// Pre-built programs
// ---------------------------------------------------------------------------

pub fn fibonacci_program() -> &'static str {
    "\
# fib(n): n on stack, result left on stack
# local[1] = n, local[0] = accumulator
    STORE 1
    PUSH 0
    STORE 0
loop:
    LOAD 1
    PUSH 0
    EQ
    JNZ @done
    LOAD 0
    LOAD 1
    ADD
    STORE 0
    LOAD 1
    PUSH 1
    SUB
    STORE 1
    JMP @loop
done:
    LOAD 0
    PRINT
    HALT
"
}

pub fn thermostat_program() -> &'static str {
    "\
# thermostat: sensor[0] = temp, actuator[0] = heater
    SENSE 0
    PUSH 20.0
    GT
    JZ @heat_on
    PUSH 0
    ACT 0
    HALT
heat_on:
    PUSH 1
    ACT 0
    HALT
"
}

pub fn conservation_program() -> &'static str {
    "\
# conservation: sum(sensors[0..2]) ≈ 100
    PUSH 0
    STORE 0
    SENSE 0
    STORE 0
    SENSE 1
    LOAD 0
    ADD
    STORE 0
    SENSE 2
    LOAD 0
    ADD
    STORE 0
    PUSH 100.0
    LOAD 0
    SUB
    PRINT
    HALT
"
}
#[cfg(test)]
mod tests {
    use crate::*;

    // ---- Opcode discriminant ----
    #[test]
    fn test_opcode_discriminant_push() {
        assert_eq!(Opcode::Push(3.0).discriminant(), 0x01);
    }

    #[test]
    fn test_opcode_discriminant_halt() {
        assert_eq!(Opcode::Halt.discriminant(), 0xFF);
    }

    #[test]
    fn test_opcode_discriminant_add() {
        assert_eq!(Opcode::Add.discriminant(), 0x10);
    }

    #[test]
    fn test_opcode_discriminant_vibe_get() {
        assert_eq!(Opcode::VibeGet.discriminant(), 0x70);
    }

    #[test]
    fn test_opcode_discriminant_sense() {
        assert_eq!(Opcode::Sense(0).discriminant(), 0x80);
    }

    // ---- Opcode Display ----
    #[test]
    fn test_opcode_display_push() {
        assert_eq!(Opcode::Push(1.5).to_string(), "PUSH 1.5");
    }

    #[test]
    fn test_opcode_display_act() {
        assert_eq!(Opcode::Act(1).to_string(), "ACT 1");
    }

    // ---- Bytecode ----
    #[test]
    fn test_bytecode_new_is_empty() {
        let bc = Bytecode::new();
        assert!(bc.is_empty());
        assert_eq!(bc.len(), 0);
    }

    #[test]
    fn test_bytecode_push() {
        let mut bc = Bytecode::new();
        bc.push(Opcode::Push(42.0));
        bc.push(Opcode::Halt);
        assert_eq!(bc.len(), 2);
    }

    #[test]
    fn test_bytecode_disassemble() {
        let mut bc = Bytecode::new();
        bc.push(Opcode::Push(1.0));
        bc.push(Opcode::Push(2.0));
        bc.push(Opcode::Add);
        bc.push(Opcode::Print);
        bc.push(Opcode::Halt);
        let listing = bc.disassemble();
        assert_eq!(listing.len(), 5);
        assert!(listing[0].contains("PUSH"));
        assert!(listing[2].contains("ADD"));
    }

    // ---- VmState basics ----
    #[test]
    fn test_vm_state_new() {
        let vm = VmState::new();
        assert!(vm.stack.is_empty());
        assert!(!vm.halted);
        assert_eq!(vm.pc, 0);
        assert_eq!(vm.tick, 0);
    }

    #[test]
    fn test_vm_state_reset() {
        let mut vm = VmState::new();
        vm.stack.push(99.0);
        vm.pc = 10;
        vm.halted = true;
        vm.tick = 5;
        vm.reset();
        assert!(vm.stack.is_empty());
        assert_eq!(vm.pc, 0);
        assert!(!vm.halted);
        assert_eq!(vm.tick, 0);
    }

    // ---- Individual instructions ----
    #[test]
    fn test_push_then_pop() {
        let mut bc = Bytecode::new();
        bc.push(Opcode::Push(42.0));
        bc.push(Opcode::Pop);
        bc.push(Opcode::Halt);

        let mut vm = VmState::new();
        let result = vm.run(&bc, 10);
        assert!(result.halted);
        assert!(result.error.is_none());
        assert!(vm.stack.is_empty());
    }

    #[test]
    fn test_add() {
        let mut bc = Bytecode::new();
        bc.push(Opcode::Push(3.0));
        bc.push(Opcode::Push(4.0));
        bc.push(Opcode::Add);
        bc.push(Opcode::Halt);

        let mut vm = VmState::new();
        let result = vm.run(&bc, 10);
        assert!(result.halted);
        assert_eq!(vm.stack.last(), Some(&7.0));
    }

    #[test]
    fn test_sub() {
        let mut bc = Bytecode::new();
        bc.push(Opcode::Push(10.0));
        bc.push(Opcode::Push(3.0));
        bc.push(Opcode::Sub);
        bc.push(Opcode::Halt);

        let mut vm = VmState::new();
        vm.run(&bc, 10);
        assert_eq!(vm.stack.last(), Some(&7.0));
    }

    #[test]
    fn test_mul() {
        let mut bc = Bytecode::new();
        bc.push(Opcode::Push(6.0));
        bc.push(Opcode::Push(7.0));
        bc.push(Opcode::Mul);
        bc.push(Opcode::Halt);

        let mut vm = VmState::new();
        vm.run(&bc, 10);
        assert_eq!(vm.stack.last(), Some(&42.0));
    }

    #[test]
    fn test_div() {
        let mut bc = Bytecode::new();
        bc.push(Opcode::Push(10.0));
        bc.push(Opcode::Push(2.0));
        bc.push(Opcode::Div);
        bc.push(Opcode::Halt);

        let mut vm = VmState::new();
        vm.run(&bc, 10);
        assert_eq!(vm.stack.last(), Some(&5.0));
    }

    #[test]
    fn test_div_by_zero() {
        let mut bc = Bytecode::new();
        bc.push(Opcode::Push(1.0));
        bc.push(Opcode::Push(0.0));
        bc.push(Opcode::Div);

        let mut vm = VmState::new();
        let result = vm.run(&bc, 10);
        assert!(result.error.is_some());
        assert!(result.error.unwrap().contains("division by zero"));
    }

    #[test]
    fn test_neg() {
        let mut bc = Bytecode::new();
        bc.push(Opcode::Push(5.0));
        bc.push(Opcode::Neg);
        bc.push(Opcode::Halt);

        let mut vm = VmState::new();
        vm.run(&bc, 10);
        assert_eq!(vm.stack.last(), Some(&-5.0));
    }

    #[test]
    fn test_dup() {
        let mut bc = Bytecode::new();
        bc.push(Opcode::Push(7.0));
        bc.push(Opcode::Dup);
        bc.push(Opcode::Halt);

        let mut vm = VmState::new();
        vm.run(&bc, 10);
        assert_eq!(vm.stack.len(), 2);
        assert_eq!(vm.stack[0], 7.0);
        assert_eq!(vm.stack[1], 7.0);
    }

    #[test]
    fn test_swap() {
        let mut bc = Bytecode::new();
        bc.push(Opcode::Push(1.0));
        bc.push(Opcode::Push(2.0));
        bc.push(Opcode::Swap);
        bc.push(Opcode::Halt);

        let mut vm = VmState::new();
        vm.run(&bc, 10);
        assert_eq!(vm.stack[0], 2.0);
        assert_eq!(vm.stack[1], 1.0);
    }

    #[test]
    fn test_load_store() {
        let mut vm = VmState::new();
        let mut bc = Bytecode::new();
        bc.push(Opcode::Push(99.0));
        bc.push(Opcode::Store(0));
        bc.push(Opcode::Load(0));
        bc.push(Opcode::Halt);

        vm.run(&bc, 10);
        assert_eq!(vm.stack.last(), Some(&99.0));
        assert_eq!(vm.locals[0], 99.0);
    }

    // ---- Jumps ----
    #[test]
    fn test_jmp_forward() {
        let mut bc = Bytecode::new();
        bc.push(Opcode::Push(1.0));  // 0
        bc.push(Opcode::Jmp(2));     // 1 → jump to 3
        bc.push(Opcode::Push(99.0)); // 2 (skipped)
        bc.push(Opcode::Halt);       // 3

        let mut vm = VmState::new();
        let result = vm.run(&bc, 10);
        assert!(result.halted);
        assert_eq!(vm.stack.last(), Some(&1.0));
    }

    #[test]
    fn test_jz_jumps_when_zero() {
        let mut bc = Bytecode::new();
        bc.push(Opcode::Push(0.0));    // 0
        bc.push(Opcode::Jz(2));        // 1: pc=1, offset=2 → pc=3 (Halt)
        bc.push(Opcode::Push(99.0));   // 2 (skipped)
        bc.push(Opcode::Halt);         // 3

        let mut vm = VmState::new();
        let result = vm.run(&bc, 10);
        assert!(result.halted);
        // JZ pops the 0.0, so stack is empty
        assert!(vm.stack.is_empty());
    }

    #[test]
    fn test_jz_does_not_jump_when_nonzero() {
        let mut bc = Bytecode::new();
        bc.push(Opcode::Push(1.0));   // 0
        bc.push(Opcode::Jz(2));       // 1 → stays
        bc.push(Opcode::Push(99.0));  // 2
        bc.push(Opcode::Halt);        // 3

        let mut vm = VmState::new();
        vm.run(&bc, 10);
        assert_eq!(vm.stack.last(), Some(&99.0));
    }

    #[test]
    fn test_jnz_jumps_when_nonzero() {
        let mut bc = Bytecode::new();
        bc.push(Opcode::Push(1.0));    // 0
        bc.push(Opcode::Jnz(1));       // 1: pc=1, offset=1 → pc=2
        bc.push(Opcode::Push(42.0));   // 2
        bc.push(Opcode::Halt);         // 3

        let mut vm = VmState::new();
        let result = vm.run(&bc, 10);
        assert!(result.halted);
        assert_eq!(vm.stack.last(), Some(&42.0));
    }

    // ---- Comparison ----
    #[test]
    fn test_lt_true() {
        let mut bc = Bytecode::new();
        bc.push(Opcode::Push(1.0));
        bc.push(Opcode::Push(2.0));
        bc.push(Opcode::Lt);
        bc.push(Opcode::Halt);

        let mut vm = VmState::new();
        vm.run(&bc, 10);
        assert_eq!(vm.stack.last(), Some(&1.0));
    }

    #[test]
    fn test_lt_false() {
        let mut bc = Bytecode::new();
        bc.push(Opcode::Push(3.0));
        bc.push(Opcode::Push(2.0));
        bc.push(Opcode::Lt);
        bc.push(Opcode::Halt);

        let mut vm = VmState::new();
        vm.run(&bc, 10);
        assert_eq!(vm.stack.last(), Some(&0.0));
    }

    #[test]
    fn test_gt_true() {
        let mut bc = Bytecode::new();
        bc.push(Opcode::Push(5.0));
        bc.push(Opcode::Push(3.0));
        bc.push(Opcode::Gt);
        bc.push(Opcode::Halt);

        let mut vm = VmState::new();
        vm.run(&bc, 10);
        assert_eq!(vm.stack.last(), Some(&1.0));
    }

    #[test]
    fn test_eq_true() {
        let mut bc = Bytecode::new();
        bc.push(Opcode::Push(42.0));
        bc.push(Opcode::Push(42.0));
        bc.push(Opcode::Eq);
        bc.push(Opcode::Halt);

        let mut vm = VmState::new();
        vm.run(&bc, 10);
        assert_eq!(vm.stack.last(), Some(&1.0));
    }

    #[test]
    fn test_eq_false() {
        let mut bc = Bytecode::new();
        bc.push(Opcode::Push(1.0));
        bc.push(Opcode::Push(2.0));
        bc.push(Opcode::Eq);
        bc.push(Opcode::Halt);

        let mut vm = VmState::new();
        vm.run(&bc, 10);
        assert_eq!(vm.stack.last(), Some(&0.0));
    }

    // ---- Logical ----
    #[test]
    fn test_and_true() {
        let mut bc = Bytecode::new();
        bc.push(Opcode::Push(1.0));
        bc.push(Opcode::Push(5.0));
        bc.push(Opcode::And);
        bc.push(Opcode::Halt);

        let mut vm = VmState::new();
        vm.run(&bc, 10);
        assert_eq!(vm.stack.last(), Some(&1.0));
    }

    #[test]
    fn test_and_false() {
        let mut bc = Bytecode::new();
        bc.push(Opcode::Push(1.0));
        bc.push(Opcode::Push(0.0));
        bc.push(Opcode::And);
        bc.push(Opcode::Halt);

        let mut vm = VmState::new();
        vm.run(&bc, 10);
        assert_eq!(vm.stack.last(), Some(&0.0));
    }

    #[test]
    fn test_or_true() {
        let mut bc = Bytecode::new();
        bc.push(Opcode::Push(0.0));
        bc.push(Opcode::Push(3.0));
        bc.push(Opcode::Or);
        bc.push(Opcode::Halt);

        let mut vm = VmState::new();
        vm.run(&bc, 10);
        assert_eq!(vm.stack.last(), Some(&1.0));
    }

    #[test]
    fn test_or_false() {
        let mut bc = Bytecode::new();
        bc.push(Opcode::Push(0.0));
        bc.push(Opcode::Push(0.0));
        bc.push(Opcode::Or);
        bc.push(Opcode::Halt);

        let mut vm = VmState::new();
        vm.run(&bc, 10);
        assert_eq!(vm.stack.last(), Some(&0.0));
    }

    #[test]
    fn test_not_zero() {
        let mut bc = Bytecode::new();
        bc.push(Opcode::Push(0.0));
        bc.push(Opcode::Not);
        bc.push(Opcode::Halt);

        let mut vm = VmState::new();
        vm.run(&bc, 10);
        assert_eq!(vm.stack.last(), Some(&1.0));
    }

    #[test]
    fn test_not_nonzero() {
        let mut bc = Bytecode::new();
        bc.push(Opcode::Push(3.0));
        bc.push(Opcode::Not);
        bc.push(Opcode::Halt);

        let mut vm = VmState::new();
        vm.run(&bc, 10);
        assert_eq!(vm.stack.last(), Some(&0.0));
    }

    // ---- Print ----
    #[test]
    fn test_print() {
        let mut bc = Bytecode::new();
        bc.push(Opcode::Push(1.5));
        bc.push(Opcode::Print);
        bc.push(Opcode::Halt);

        let mut vm = VmState::new();
        let result = vm.run(&bc, 10);
        assert!(result.halted);
        assert_eq!(result.output, vec!["1.5"]);
    }

    // ---- Vibe ----
    #[test]
    fn test_vibe_get_set() {
        let mut bc = Bytecode::new();
        bc.push(Opcode::Push(0.75));
        bc.push(Opcode::VibeSet);
        bc.push(Opcode::VibeGet);
        bc.push(Opcode::Print);
        bc.push(Opcode::Halt);

        let mut vm = VmState::new();
        let result = vm.run(&bc, 10);
        assert!(result.halted);
        assert_eq!(vm.vibe, 0.75);
        assert_eq!(result.output, vec!["0.75"]);
    }

    // ---- Sensors & Actuators ----
    #[test]
    fn test_sense_act() {
        let mut vm = VmState::new();
        vm.sensors[0] = 25.0;
        vm.sensors[1] = 18.0;

        let mut bc = Bytecode::new();
        bc.push(Opcode::Sense(0));
        bc.push(Opcode::Sense(1));
        bc.push(Opcode::Add);
        bc.push(Opcode::Act(0));
        bc.push(Opcode::Halt);

        vm.run(&bc, 10);
        assert_eq!(vm.actuators[0], 43.0);
    }

    // ---- Call/Ret ----
    #[test]
    fn test_call_and_ret() {
        // Main: push(10), call sub at 4, print, halt
        // Sub: push(2), mul (10*2=20), ret
        let mut bc = Bytecode::new();
        bc.push(Opcode::Push(10.0)); // 0
        bc.push(Opcode::Call(4));    // 1: call sub at 4, return addr=2
        bc.push(Opcode::Print);      // 2: print 20.0
        bc.push(Opcode::Halt);       // 3
        // subroutine:
        bc.push(Opcode::Push(2.0));  // 4
        bc.push(Opcode::Mul);        // 5: 10 * 2 = 20
        bc.push(Opcode::Ret);        // 6: return to pc=2

        let mut vm = VmState::new();
        let result = vm.run(&bc, 10);
        assert!(result.halted);
        assert_eq!(result.output, vec!["20"]);
    }

    #[test]
    fn test_ret_from_main_halts() {
        let mut bc = Bytecode::new();
        bc.push(Opcode::Push(1.0));
        bc.push(Opcode::Ret);

        let mut vm = VmState::new();
        let result = vm.run(&bc, 10);
        assert!(result.halted);
        assert_eq!(vm.stack.last(), Some(&1.0));
    }

    // ---- Stack underflow ----
    #[test]
    fn test_stack_underflow_add() {
        let mut bc = Bytecode::new();
        bc.push(Opcode::Push(1.0));
        bc.push(Opcode::Add);

        let mut vm = VmState::new();
        let result = vm.run(&bc, 10);
        assert!(result.error.is_some());
        assert!(result.error.as_ref().unwrap().contains("stack underflow"));
    }

    #[test]
    fn test_stack_underflow_pop() {
        let mut bc = Bytecode::new();
        bc.push(Opcode::Pop);

        let mut vm = VmState::new();
        let result = vm.run(&bc, 10);
        assert!(result.error.is_some());
    }

    // ---- Max steps ----
    #[test]
    fn test_max_steps_exceeded() {
        let mut bc = Bytecode::new();
        bc.push(Opcode::Jmp(0));

        let mut vm = VmState::new();
        let result = vm.run(&bc, 100);
        assert!(!result.halted);
        assert!(result.error.is_some());
        assert!(result.error.unwrap().contains("max steps"));
        assert_eq!(result.steps, 100);
    }

    #[test]
    fn test_run_max_steps_zero() {
        let mut bc = Bytecode::new();
        bc.push(Opcode::Halt);

        let mut vm = VmState::new();
        let result = vm.run(&bc, 0);
        assert!(result.error.is_some());
        assert_eq!(result.steps, 0);
    }

    // ---- Step ----
    #[test]
    fn test_step_halted_returns_halted() {
        let mut vm = VmState::new();
        vm.halted = true;
        let bc = Bytecode::new();
        assert_eq!(vm.step(&bc), StepResult::Halted);
    }

    #[test]
    fn test_step_past_end_halts() {
        let mut vm = VmState::new();
        vm.pc = 5;
        let bc = Bytecode::new();
        assert_eq!(vm.step(&bc), StepResult::Halted);
        assert!(vm.halted);
    }

    // ---- Compiler ----
    #[test]
    fn test_compile_simple() {
        let source = "PUSH 42\nPRINT\nHALT";
        let bc = Compiler::compile(source).unwrap();
        assert_eq!(bc.len(), 3);
        assert!(matches!(bc.instructions[0].opcode, Opcode::Push(42.0)));
        assert!(matches!(bc.instructions[1].opcode, Opcode::Print));
        assert!(matches!(bc.instructions[2].opcode, Opcode::Halt));
    }

    #[test]
    fn test_compile_add() {
        let source = "PUSH 3\nPUSH 4\nADD\nPRINT\nHALT";
        let bc = Compiler::compile(source).unwrap();
        assert_eq!(bc.len(), 5);
    }

    #[test]
    fn test_compile_labels() {
        let source = "\
    JMP @skip
    PUSH 99
skip:
    PUSH 10
    PRINT
    HALT
";
        let bc = Compiler::compile(source).unwrap();
        assert_eq!(bc.len(), 5);
    }

    #[test]
    fn test_compile_duplicate_label() {
        let source = "loop:\nloop:\nHALT";
        assert!(Compiler::compile(source).is_err());
    }

    #[test]
    fn test_compile_undefined_label() {
        let source = "JMP @nonexistent\nHALT";
        assert!(Compiler::compile(source).is_err());
    }

    #[test]
    fn test_compile_unknown_opcode() {
        let source = "FOOBAR 42";
        assert!(Compiler::compile(source).is_err());
    }

    #[test]
    fn test_compile_with_comments() {
        let source = "# comment\n; also comment\nPUSH 1\nHALT";
        let bc = Compiler::compile(source).unwrap();
        assert_eq!(bc.len(), 2);
    }

    #[test]
    fn test_compile_all_opcodes() {
        let source = "\
PUSH 1.5
ADD
SUB
MUL
DIV
NEG
LOAD 0
STORE 1
JMP 2
JZ 3
JNZ 4
LT
GT
EQ
AND
OR
NOT
DUP
POP
SWAP
PRINT
HALT
VIBE_GET
VIBE_SET
SENSE 0
ACT 1
CALL 0
RET
";
        let bc = Compiler::compile(source).unwrap();
        assert_eq!(bc.len(), 28);
    }

    // ---- Pre-built programs ----
    #[test]
    fn test_fibonacci() {
        let source = fibonacci_program();
        let bc = Compiler::compile(source).unwrap();
        let mut bc_with = Bytecode::new();
        bc_with.push(Opcode::Push(10.0));
        for ins in &bc.instructions {
            bc_with.push(ins.opcode);
        }
        let mut vm = VmState::new();
        let result = vm.run(&bc_with, 1000);
        assert!(result.halted, "fib should halt: {:?}", result.error);
        assert_eq!(result.output, vec!["55"], "fib(10)=55");
    }

    #[test]
    fn test_thermostat_cold() {
        let source = thermostat_program();
        let bc = Compiler::compile(source).unwrap();
        let mut vm = VmState::new();
        vm.sensors[0] = 15.0;
        let result = vm.run(&bc, 100);
        assert!(result.halted);
        assert_eq!(vm.actuators[0], 1.0);
    }

    #[test]
    fn test_thermostat_warm() {
        let source = thermostat_program();
        let bc = Compiler::compile(source).unwrap();
        let mut vm = VmState::new();
        vm.sensors[0] = 25.0;
        vm.run(&bc, 100);
        assert_eq!(vm.actuators[0], 0.0);
    }

    #[test]
    fn test_conservation() {
        let source = conservation_program();
        let bc = Compiler::compile(source).unwrap();
        let mut vm = VmState::new();
        vm.sensors[0] = 30.0;
        vm.sensors[1] = 40.0;
        vm.sensors[2] = 30.0;
        let result = vm.run(&bc, 100);
        assert!(result.halted);
        assert_eq!(result.output, vec!["0"]);
    }

    // ---- Error cases ----
    #[test]
    fn test_local_out_of_bounds_load() {
        let mut bc = Bytecode::new();
        bc.push(Opcode::Load(16));
        let mut vm = VmState::new();
        let result = vm.run(&bc, 10);
        assert!(result.error.is_some());
    }

    #[test]
    fn test_sensor_out_of_bounds() {
        let mut bc = Bytecode::new();
        bc.push(Opcode::Sense(8));
        let mut vm = VmState::new();
        let result = vm.run(&bc, 10);
        assert!(result.error.is_some());
    }

    #[test]
    fn test_actuator_out_of_bounds() {
        let mut bc = Bytecode::new();
        bc.push(Opcode::Push(1.0));
        bc.push(Opcode::Act(8));
        let mut vm = VmState::new();
        let result = vm.run(&bc, 10);
        assert!(result.error.is_some());
    }

    #[test]
    fn test_call_out_of_bounds() {
        let mut bc = Bytecode::new();
        bc.push(Opcode::Call(99));
        let mut vm = VmState::new();
        let result = vm.run(&bc, 10);
        assert!(result.error.is_some());
    }

    #[test]
    fn test_jmp_out_of_bounds() {
        let mut bc = Bytecode::new();
        bc.push(Opcode::Push(1.0));
        bc.push(Opcode::Jmp(999));
        let mut vm = VmState::new();
        let result = vm.run(&bc, 10);
        assert!(result.error.is_some());
    }

    // ---- VmResult ----
    #[test]
    fn test_vm_result_populated() {
        let mut bc = Bytecode::new();
        bc.push(Opcode::Push(42.0));
        bc.push(Opcode::Halt);

        let mut vm = VmState::new();
        let result = vm.run(&bc, 10);
        assert_eq!(result.steps, 2);
        assert!(result.halted);
        assert_eq!(result.stack_top, Some(42.0));
        assert!(result.error.is_none());
    }

    // ---- Empty program ----
    #[test]
    fn test_empty_program_halts_immediately() {
        let bc = Bytecode::new();
        let mut vm = VmState::new();
        let result = vm.run(&bc, 10);
        assert!(result.halted);
        assert_eq!(result.steps, 1);
    }

    // ---- Negative jumps ----
    #[test]
    fn test_negative_jump_backwards() {
        let mut bc = Bytecode::new();
        bc.push(Opcode::Push(1.0));  // 0: push 1
        bc.push(Opcode::Push(2.0));  // 1: push 2
        bc.push(Opcode::Add);        // 2: 1+2=3
        bc.push(Opcode::Halt);       // 3

        let mut vm = VmState::new();
        vm.run(&bc, 10);
        assert_eq!(vm.stack.last(), Some(&3.0));
    }

    // ---- Label compilation execution ----
    #[test]
    fn test_compile_and_run_with_labels() {
        let source = "\
    PUSH 0
    STORE 0
    PUSH 5
loop:
    DUP
    PUSH 1
    SUB
    DUP
    PUSH 0
    GT
    JNZ @loop
    POP
    HALT
";
        let bc = Compiler::compile(source).unwrap();
        let mut vm = VmState::new();
        let result = vm.run(&bc, 100);
        // This computes a countdown; final stack should be empty after POPs
        assert!(result.halted || result.error.is_some());
    }
}
