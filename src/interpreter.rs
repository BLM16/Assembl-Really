pub mod error;

use std::process::ExitCode;

use error::AsmrRuntimeError;
use crate::parser::{line::{Line, MemType}, token::Token, instruction::Instruction};
use crate::core::{register::{RegisterData, RegisterName}, builtins::call_builtin_fn, executing_context::ExecutingContext, flags::Flag};

/// Executes parsed asmr code.
/// 
/// # Examples
/// 
/// ```
/// # use assembl_really::{parse_lines, execute};
/// let v = vec!["mov eax, 5", "push eax", "call asmr::io::print"];
/// let lines = parse_lines(v.iter()).unwrap();
/// let exit_code = execute(lines).unwrap();
/// ```
pub fn execute(lines: Vec<Line>) -> Result<ExitCode, AsmrRuntimeError>
{
    // Manage the current context for execution
    // Stores registers values, variable buffers, label pointers, instruction pointers
    let mut ctx = ExecutingContext::new();
    ctx.stack.push(RegisterData::Pointer(lines.len() as i32)); // Push final ret pointer (EOF)
    ctx.stack.push(RegisterData::Pointer(0)); // Push initial ebp value
    *ctx.registers.get(&RegisterName::Esp).unwrap().borrow_mut() += 1; // Point esp to ebp

    // Get list of all label addresses
    for i in 0..lines.len() {
        if let Some(Line::Label(label)) = lines.get(i) {
            ctx.labels.insert(label.to_string(), i);
        }
    }

    loop {
        match lines.get(ctx.ptr) {
            Some(Line::Instruction { instruction, params }) => handle_instruction(instruction, params, &mut ctx)?,
            Some(Line::Variable { identifier, mem_type, params }) => handle_variable(identifier, mem_type, params, &mut ctx)?,
            Some(_) => {}, // Labels already handled, ignore blank lines
            None => return Ok(ExitCode::from(ctx.registers.get(&RegisterName::Eax).unwrap().borrow().get_raw() as u8)), // EOF -> Return exit code from eax
        };

        // Set the current instruction pointer to the next line to execute
        // Increment the next instruction pointer
        ctx.ptr = ctx.next;
        ctx.registers.get(&RegisterName::Eip).unwrap().borrow_mut().data = RegisterData::Pointer(ctx.ptr as i32);
        ctx.next += 1;
    }
}

/// Handles an instruction line
fn handle_instruction(instruction: &Instruction, params: &Vec<Token>, ctx: &mut ExecutingContext) -> Result<(), AsmrRuntimeError> {
    match instruction {
        Instruction::Nop => Ok(()),

    // Stack
        Instruction::Push => {
            if params.len() == 0 { return Err(AsmrRuntimeError::from(ctx.ptr, "`push` takes parameters of type <...Register>")) }

            for param in params {
                if let Token::Register(r) = param {
                    ctx.stack.push(ctx.registers.get(r).unwrap().borrow_mut().data);
                    *ctx.registers.get(&RegisterName::Esp).unwrap().borrow_mut() += 1
                } else {
                    return Err(AsmrRuntimeError::from(ctx.ptr, "`push` takes parameters of type <...Register>"))
                }
            }

            Ok(())
        },
        Instruction::Pop => {
            if params.len() != 1 { return Err(AsmrRuntimeError::from(ctx.ptr, "`pop` takes one parameter of type <Register>")) }

            let last = ctx.stack.remove(ctx.registers.get(&RegisterName::Esp).unwrap().borrow().get_raw() as usize);
            *ctx.registers.get(&RegisterName::Esp).unwrap().borrow_mut() -= 1;

            if let Some(Token::Register(reg)) = params.get(0) {
                let mut reg = ctx.registers.get(reg).unwrap().borrow_mut();
                reg.data = last;
                Ok(())
            }
            else {
                Err(AsmrRuntimeError::from(ctx.ptr, "`pop` takes one parameter of type <Register>"))
            }
        },

    // Move
        Instruction::Mov => {
            if params.len() != 2 { return Err(AsmrRuntimeError::from(ctx.ptr, "`mov` takes parameters of type <Register, [Register | Identifier | Numeric]>")) }

            if let Some(Token::Register(lhs)) = params.get(0) {
                let mut lhs = ctx.registers.get(lhs).unwrap().borrow_mut();
                match params.get(1) {
                    Some(Token::Numeric(i)) => Ok(lhs.data = RegisterData::Value(*i)),
                    Some(Token::Identifier(s)) => Ok(lhs.data = RegisterData::Pointer(*ctx.symtab.get(s).expect(format!("unknown identifier `{s}`").as_str()))),
                    Some(Token::Register(r)) => Ok(lhs.data = ctx.registers.get(r).unwrap().borrow().data),
                    Some(_) => Err(AsmrRuntimeError::from(ctx.ptr, "`mov` takes parameters of type <Register, [Register | Identifier | Numeric]>")),
                    None => unreachable!("params.len() == 2"),
                }
            }
            else {
                Err(AsmrRuntimeError::from(ctx.ptr, "`mov` takes parameters of type <Register, [Register | Identifier | Numeric]>"))
            }
        },
        Instruction::Xchg => {
            if params.len() != 2 { return Err(AsmrRuntimeError::from(ctx.ptr, "`xchg` takes parameters of type <Register, Register>")) }

            if let Some(Token::Register(lhs)) = params.get(0) {
                if let Some(Token::Register(rhs)) = params.get(1) {
                    let mut lhs = ctx.registers.get(lhs).unwrap().borrow_mut();
                    let mut rhs = ctx.registers.get(rhs).unwrap().borrow_mut();

                    let tmp = lhs.data;
                    lhs.data = rhs.data;
                    rhs.data = tmp;

                    Ok(())
                }
                else {
                    Err(AsmrRuntimeError::from(ctx.ptr, "`xchg` takes parameters of type <Register, Register>"))
                }
            }
            else {
                Err(AsmrRuntimeError::from(ctx.ptr, "`xchg` takes parameters of type <Register, Register>"))
            }
        },

    // Arithmetic
        Instruction::Add => {
            if params.len() != 2 { return Err(AsmrRuntimeError::from(ctx.ptr, "`add` takes parameters of type <Register, [Register | Numeric]>")); }

            if let Some(Token::Register(lhs)) = params.get(0) {
                match params.get(1) {
                    Some(Token::Numeric(rhs)) => Ok(*ctx.registers.get(lhs).unwrap().borrow_mut() += *rhs),
                    Some(Token::Register(rhs)) => {
                        let mut lhs = ctx.registers.get(lhs).unwrap().borrow_mut();
                        let rhs = ctx.registers.get(rhs).unwrap().borrow();
                        *lhs += *rhs;
                        Ok(())
                    },
                    Some(_) => Err(AsmrRuntimeError::from(ctx.ptr, "`add` takes parameters of type <Register, [Register | Numeric]>")),
                    None => unreachable!("params.len() == 2"),
                }
            }
            else {
                Err(AsmrRuntimeError::from(ctx.ptr, "`add` takes parameters of type <Register, [Register | Numeric]>"))
            }
        },
        Instruction::Sub => {
            if params.len() != 2 { return Err(AsmrRuntimeError::from(ctx.ptr, "`sub` takes parameters of type <Register, [Register | Numeric]>")); }

            if let Some(Token::Register(lhs)) = params.get(0) {
                match params.get(1) {
                    Some(Token::Numeric(rhs)) => Ok(*ctx.registers.get(lhs).unwrap().borrow_mut() -= *rhs),
                    Some(Token::Register(rhs)) => {
                        let mut lhs = ctx.registers.get(lhs).unwrap().borrow_mut();
                        let rhs = ctx.registers.get(rhs).unwrap().borrow();
                        *lhs -= *rhs;
                        Ok(())
                    },
                    Some(_) => Err(AsmrRuntimeError::from(ctx.ptr, "`sub` takes parameters of type <Register, [Register | Numeric]>")),
                    None => unreachable!("params.len() == 2"),
                }
            }
            else {
                Err(AsmrRuntimeError::from(ctx.ptr, "`sub` takes parameters of type <Register, [Register | Numeric]>"))
            }
        },
        Instruction::Mul => {
            if params.len() != 2 { return Err(AsmrRuntimeError::from(ctx.ptr, "`mul` takes parameters of type <Register, [Register | Numeric]>")) }

            if let Some(Token::Register(lhs)) = params.get(0) {
                match params.get(1) {
                    Some(Token::Numeric(rhs)) => Ok(*ctx.registers.get(lhs).unwrap().borrow_mut() *= *rhs),
                    Some(Token::Register(rhs)) => {
                        let mut lhs = ctx.registers.get(lhs).unwrap().borrow_mut();
                        let rhs = ctx.registers.get(rhs).unwrap().borrow();
                        *lhs *= *rhs;
                        Ok(())
                    },
                    Some(_) => Err(AsmrRuntimeError::from(ctx.ptr, "`mul` takes parameters of type <Register, [Register | Numeric]>")),
                    None => unreachable!("params.len() == 2"),
                }
            }
            else {
                Err(AsmrRuntimeError::from(ctx.ptr, "`mul` takes parameters of type <Register, [Register | Numeric]>"))
            }
        },
        Instruction::Div => {
            if params.len() != 2 { return Err(AsmrRuntimeError::from(ctx.ptr, "`div` takes parameters of type <Register, [Register | Numeric]>")) }

            if let Some(Token::Register(lhs)) = params.get(0) {
                match params.get(1) {
                    Some(Token::Numeric(rhs)) => Ok(*ctx.registers.get(lhs).unwrap().borrow_mut() /= *rhs),
                    Some(Token::Register(rhs)) => {
                        let mut lhs = ctx.registers.get(lhs).unwrap().borrow_mut();
                        let rhs = ctx.registers.get(rhs).unwrap().borrow();
                        *lhs /= *rhs;
                        Ok(())
                    },
                    Some(_) => Err(AsmrRuntimeError::from(ctx.ptr, "`div` takes parameters of type <Register, [Register | Numeric]>")),
                    None => unreachable!("params.len() == 2"),
                }
            }
            else {
                Err(AsmrRuntimeError::from(ctx.ptr, "`div` takes parameters of type <Register, [Register | Numeric]>"))
            }
        },
        Instruction::Inc => {
            if params.len() != 1 { return Err(AsmrRuntimeError::from(ctx.ptr, "`inc` takes one parameter of type <Register>")) }

            if let Some(Token::Register(reg)) = params.get(0) {
                *ctx.registers.get(reg).unwrap().borrow_mut() += 1;
                Ok(())
            }
            else {
                Err(AsmrRuntimeError::from(ctx.ptr, "`inc` takes one parameter of type <Register>"))
            }
        },
        Instruction::Dec => {
            if params.len() != 1 { return Err(AsmrRuntimeError::from(ctx.ptr, "`dec` takes one parameter of type <Register>")) }
    
            if let Some(Token::Register(reg)) = params.get(0) {
                *ctx.registers.get(reg).unwrap().borrow_mut() -= 1;
                Ok(())
            }
            else {
                Err(AsmrRuntimeError::from(ctx.ptr, "`dec` takes one parameter of type <Register>"))
            }
        },
        Instruction::Shl => {
            if params.len() != 2 { return Err(AsmrRuntimeError::from(ctx.ptr, "`shl` takes parameters of type <Register, Numeric>")) }

            if let Some(Token::Register(lhs)) = params.get(0) {
                if let Some(Token::Numeric(rhs)) = params.get(1) {
                    if *rhs < 0 { return Err(AsmrRuntimeError::from(ctx.ptr, "`shl` requires the parameter <Numeric> to be greater than or equal to 0")) }
                    *ctx.registers.get(lhs).unwrap().borrow_mut() <<= *rhs;
                    Ok(())
                }
                else {
                    Err(AsmrRuntimeError::from(ctx.ptr, "`shl` takes parameters of type <Register, Numeric>"))
                }
            }
            else {
                Err(AsmrRuntimeError::from(ctx.ptr, "`shl` takes parameters of type <Register, Numeric>"))
            }
        },
        Instruction::Shr => {
            if params.len() != 2 { return Err(AsmrRuntimeError::from(ctx.ptr, "`shr` takes parameters of type <Register, Numeric>")) }

            if let Some(Token::Register(lhs)) = params.get(0) {
                if let Some(Token::Numeric(rhs)) = params.get(1) {
                    if *rhs < 0 { return Err(AsmrRuntimeError::from(ctx.ptr, "`shr` requires the parameter <Numeric> to be greater than or equal to 0")) }
                    *ctx.registers.get(lhs).unwrap().borrow_mut() >>= *rhs;
                    Ok(())
                }
                else {
                    Err(AsmrRuntimeError::from(ctx.ptr, "`shr` takes parameters of type <Register, Numeric>"))
                }
            }
            else {
                Err(AsmrRuntimeError::from(ctx.ptr, "`shr` takes parameters of type <Register, Numeric>"))
            }
        },

    // Comparisons
        Instruction::Cmp => {
            if params.len() != 2 { return Err(AsmrRuntimeError::from(ctx.ptr, "`cmp` takes parameters of type <Register, [Register | Numeric]>")) }
            
            let mut cmp = |lhs: i32, rhs: i32| {
                if lhs == rhs {
                    ctx.flags.set(Flag::ZF);
                    ctx.flags.unset(Flag::CF);
                }
                else if lhs > rhs {
                    ctx.flags.unset(Flag::ZF);
                    ctx.flags.unset(Flag::CF);
                }
                else {
                    ctx.flags.unset(Flag::ZF);
                    ctx.flags.set(Flag::CF);
                }
            };

            if let Some(Token::Register(lhs)) = params.get(0) {
                match params.get(1) {
                    Some(Token::Numeric(rhs)) => {
                        let lhs = ctx.registers.get(lhs).unwrap().borrow().get_raw();
                        cmp(lhs, *rhs);
                        Ok(())
                    },
                    Some(Token::Register(rhs)) => {
                        let lhs = ctx.registers.get(lhs).unwrap().borrow().get_raw();
                        let rhs = ctx.registers.get(rhs).unwrap().borrow().get_raw();
                        cmp(lhs, rhs);
                        Ok(())
                    },
                    Some(_) => Err(AsmrRuntimeError::from(ctx.ptr, "`cmp` takes parameters of type <Register, [Register | Numeric]>")),
                    None => unreachable!("params.len() == 2"),
                }
            }
            else {
                Err(AsmrRuntimeError::from(ctx.ptr, "`cmp` takes parameters of type <Register, Register>"))
            }
        },
        Instruction::And => {
            if params.len() != 2 { return Err(AsmrRuntimeError::from(ctx.ptr, "`and` takes parameters of type <Register, [Register | Numeric]>")) }

            if let Some(Token::Register(lhs)) = params.get(0) {
                match params.get(1) {
                    Some(Token::Numeric(rhs)) => Ok(*ctx.registers.get(lhs).unwrap().borrow_mut() &= *rhs),
                    Some(Token::Register(rhs)) => {
                        let mut lhs = ctx.registers.get(lhs).unwrap().borrow_mut();
                        let rhs = ctx.registers.get(rhs).unwrap().borrow();
                        *lhs &= *rhs;
                        Ok(())
                    },
                    Some(_) => Err(AsmrRuntimeError::from(ctx.ptr, "`and` takes parameters of type <Register, [Register | Numeric]>")),
                    None => unreachable!("params.len() == 2"),
                }
            }
            else {
                Err(AsmrRuntimeError::from(ctx.ptr, "`and` takes parameters of type <Register, [Register | Numeric]>"))
            }
        },
        Instruction::Or => {
            if params.len() != 2 { return Err(AsmrRuntimeError::from(ctx.ptr, "`or` takes parameters of type <Register, [Register | Numeric]>")) }

            if let Some(Token::Register(lhs)) = params.get(0) {
                match params.get(1) {
                    Some(Token::Numeric(rhs)) => Ok(*ctx.registers.get(lhs).unwrap().borrow_mut() |= *rhs),
                    Some(Token::Register(rhs)) => {
                        let mut lhs = ctx.registers.get(lhs).unwrap().borrow_mut();
                        let rhs = ctx.registers.get(rhs).unwrap().borrow();
                        *lhs |= *rhs;
                        Ok(())
                    },
                    Some(_) => Err(AsmrRuntimeError::from(ctx.ptr, "`or` takes parameters of type <Register, [Register | Numeric]>")),
                    None => unreachable!("params.len() == 2"),
                }
            }
            else {
                Err(AsmrRuntimeError::from(ctx.ptr, "`or` takes parameters of type <Register, [Register | Numeric]>"))
            }
        },
        Instruction::Not => {
            if params.len() != 1 { return Err(AsmrRuntimeError::from(ctx.ptr, "`not` takes one parameter of type <Register>")) }

            if let Some(Token::Register(reg)) = params.get(0) {
                ctx.registers.get(reg).unwrap().borrow_mut().bitnot_assign_self();
                Ok(())
            }
            else {
                Err(AsmrRuntimeError::from(ctx.ptr, "`not` takes one parameter of type <Register>"))
            }
        },
        Instruction::Xor => {
            if params.len() != 2 { return Err(AsmrRuntimeError::from(ctx.ptr, "`xor` takes parameters of type <Register, [Register | Numeric]>")) }

            if let Some(Token::Register(lhs)) = params.get(0) {
                match params.get(1) {
                    Some(Token::Numeric(rhs)) => Ok(*ctx.registers.get(lhs).unwrap().borrow_mut() ^= *rhs),
                    Some(Token::Register(rhs)) => {
                        let mut lhs = ctx.registers.get(lhs).unwrap().borrow_mut();
                        let rhs = ctx.registers.get(rhs).unwrap().borrow();
                        *lhs ^= *rhs;
                        Ok(())
                    },
                    Some(_) => Err(AsmrRuntimeError::from(ctx.ptr, "`xor` takes parameters of type <Register, [Register | Numeric]>")),
                    None => unreachable!("params.len() == 2"),
                }
            }
            else {
                Err(AsmrRuntimeError::from(ctx.ptr, "`xor` takes parameters of type <Register, [Register | Numeric]>"))
            }
        },
        Instruction::Test => {
            if params.len() != 2 { return Err(AsmrRuntimeError::from(ctx.ptr, "`test` takes parameters of type <Register, [Register | Numeric]>")) }
            
            let mut test = |lhs: i32, rhs: i32| {
                ctx.flags.unset(Flag::CF);
                ctx.flags.unset(Flag::ZF);

                let and = lhs & rhs;
                if and & (1 << 31) == 0 { ctx.flags.unset(Flag::SF) } else { ctx.flags.set(Flag::SF) }

                if and == 0 {
                    ctx.flags.set(Flag::ZF);
                }
            };

            if let Some(Token::Register(lhs)) = params.get(0) {
                match params.get(1) {
                    Some(Token::Numeric(rhs)) => {
                        let lhs = ctx.registers.get(lhs).unwrap().borrow().get_raw();
                        test(lhs, *rhs);
                        Ok(())
                    },
                    Some(Token::Register(rhs)) => {
                        let lhs = ctx.registers.get(lhs).unwrap().borrow().get_raw();
                        let rhs = ctx.registers.get(rhs).unwrap().borrow().get_raw();
                        test(lhs, rhs);
                        Ok(())
                    },
                    Some(_) => Err(AsmrRuntimeError::from(ctx.ptr, "`test` takes parameters of type <Register, [Register | Numeric]>")),
                    None => unreachable!("params.len() == 2"),
                }
            }
            else {
                Err(AsmrRuntimeError::from(ctx.ptr, "`test` takes parameters of type <Register, Register>"))
            }
        },

    // Jumps
        Instruction::Jmp => {
            if params.len() != 1 { return Err(AsmrRuntimeError::from(ctx.ptr, "`jmp` takes one parameter of type <Identifier>")) }

            if let Some(Token::Identifier(s)) = params.get(0) {
                let addr = ctx.labels.get(s);
                match addr {
                    Some(addr) => Ok(ctx.next = *addr),
                    None => panic!("no address associated with identifier `{s}`"),
                }
            }
            else {
                Err(AsmrRuntimeError::from(ctx.ptr, "`jmp` takes one parameter of type <Identifier>"))
            }
        },
        Instruction::Jz => {
            if params.len() != 1 { return Err(AsmrRuntimeError::from(ctx.ptr, "`jz` takes one parameter of type <Identifier>")) }

            if let Some(Token::Identifier(s)) = params.get(0) {
                let addr = ctx.labels.get(s);
                let jz_flags_set = ctx.flags.get(Flag::ZF);
                match addr {
                    Some(addr) if jz_flags_set => Ok(ctx.next = *addr),
                    Some(_) => Ok(()),
                    None => Err(AsmrRuntimeError::from(ctx.ptr, format!("no address associated with identifier `{s}`"))),
                }
            }
            else {
                Err(AsmrRuntimeError::from(ctx.ptr, "`jz` takes one parameter of type <Identifier>"))
            }
        },
        Instruction::Jnz => {
            if params.len() != 1 { return Err(AsmrRuntimeError::from(ctx.ptr, "`jnz` takes one parameter of type <Identifier>")) }

            if let Some(Token::Identifier(s)) = params.get(0) {
                let addr = ctx.labels.get(s);
                let jnz_flags_set = !ctx.flags.get(Flag::ZF);
                match addr {
                    Some(addr) if jnz_flags_set => Ok(ctx.next = *addr),
                    Some(_) => Ok(()),
                    None => Err(AsmrRuntimeError::from(ctx.ptr, format!("no address associated with identifier `{s}`"))),
                }
            }
            else {
                Err(AsmrRuntimeError::from(ctx.ptr, "`jnz` takes one parameter of type <Identifier>"))
            }
        },
        Instruction::Jg => {
            if params.len() != 1 { return Err(AsmrRuntimeError::from(ctx.ptr, "`jg` takes one parameter of type <Identifier>")) }

            if let Some(Token::Identifier(s)) = params.get(0) {
                let addr = ctx.labels.get(s);
                let jg_flags_set = !ctx.flags.get(Flag::ZF) && (ctx.flags.get(Flag::SF) == ctx.flags.get(Flag::OF));
                match addr {
                    Some(addr) if jg_flags_set => Ok(ctx.next = *addr),
                    Some(_) => Ok(()),
                    None => Err(AsmrRuntimeError::from(ctx.ptr, format!("no address associated with identifier `{s}`"))),
                }
            }
            else {
                Err(AsmrRuntimeError::from(ctx.ptr, "`jg` takes one parameter of type <Identifier>"))
            }
        },
        Instruction::Jl => {
            if params.len() != 1 { return Err(AsmrRuntimeError::from(ctx.ptr, "`jl` takes one parameter of type <Identifier>")) }

            if let Some(Token::Identifier(s)) = params.get(0) {
                let addr = ctx.labels.get(s);
                let jl_flags_set = ctx.flags.get(Flag::SF) != ctx.flags.get(Flag::OF);
                match addr {
                    Some(addr) if jl_flags_set => Ok(ctx.next = *addr),
                    Some(_) => Ok(()),
                    None => Err(AsmrRuntimeError::from(ctx.ptr, format!("no address associated with identifier `{s}`"))),
                }
            }
            else {
                Err(AsmrRuntimeError::from(ctx.ptr, "`jl` takes one parameter of type <Identifier>"))
            }
        },
        Instruction::Jge => {
            if params.len() != 1 { return Err(AsmrRuntimeError::from(ctx.ptr, "`jge` takes one parameter of type <Identifier>")) }

            if let Some(Token::Identifier(s)) = params.get(0) {
                let addr = ctx.labels.get(s);
                let jge_flags_set = ctx.flags.get(Flag::SF) == ctx.flags.get(Flag::OF);
                match addr {
                    Some(addr) if jge_flags_set => Ok(ctx.next = *addr),
                    Some(_) => Ok(()),
                    None => Err(AsmrRuntimeError::from(ctx.ptr, format!("no address associated with identifier `{s}`"))),
                }
            }
            else {
                Err(AsmrRuntimeError::from(ctx.ptr, "`jge` takes one parameter of type <Identifier>"))
            }
        },
        Instruction::Jle => {
            if params.len() != 1 { return Err(AsmrRuntimeError::from(ctx.ptr, "`jle` takes one parameter of type <Identifier>")) }

            if let Some(Token::Identifier(s)) = params.get(0) {
                let addr = ctx.labels.get(s);
                let jle_flags_set = ctx.flags.get(Flag::ZF) || (ctx.flags.get(Flag::SF) != ctx.flags.get(Flag::OF));
                match addr {
                    Some(addr) if jle_flags_set => Ok(ctx.next = *addr),
                    Some(_) => Ok(()),
                    None => Err(AsmrRuntimeError::from(ctx.ptr, format!("no address associated with identifier `{s}`"))),
                }
            }
            else {
                Err(AsmrRuntimeError::from(ctx.ptr, "`jle` takes one parameter of type <Identifier>"))
            }
        },
        Instruction::Je => {
            if params.len() != 1 { return Err(AsmrRuntimeError::from(ctx.ptr, "`je` takes one parameter of type <Identifier>")) }

            if let Some(Token::Identifier(s)) = params.get(0) {
                let addr = ctx.labels.get(s);
                let je_flags_set = ctx.flags.get(Flag::ZF);
                match addr {
                    Some(addr) if je_flags_set => Ok(ctx.next = *addr),
                    Some(_) => Ok(()),
                    None => Err(AsmrRuntimeError::from(ctx.ptr, format!("no address associated with identifier `{s}`"))),
                }
            }
            else {
                Err(AsmrRuntimeError::from(ctx.ptr, "`je` takes one parameter of type <Identifier>"))
            }
        },
        Instruction::Jne => {
            if params.len() != 1 { return Err(AsmrRuntimeError::from(ctx.ptr, "`jne` takes one parameter of type <Identifier>")) }

            if let Some(Token::Identifier(s)) = params.get(0) {
                let addr = ctx.labels.get(s);
                let jne_flags_set = !ctx.flags.get(Flag::ZF);
                match addr {
                    Some(addr) if jne_flags_set => Ok(ctx.next = *addr),
                    Some(_) => Ok(()),
                    None => Err(AsmrRuntimeError::from(ctx.ptr, format!("no address associated with identifier `{s}`"))),
                }
            }
            else {
                Err(AsmrRuntimeError::from(ctx.ptr, "`jne` takes one parameter of type <Identifier>"))
            }
        },

    // Functions
        Instruction::Call => {
            if params.len() != 1 { return Err(AsmrRuntimeError::from(ctx.ptr, "`call` takes one parameter of type <Identifier>" )) }

            if let Some(Token::Identifier(s)) = params.get(0) {
                /*
                 * push eip
                 * push ebp
                 * mov ebp, esp
                 */
                handle_instruction(&Instruction::Push, &vec![ Token::Register(RegisterName::Eip) ], ctx)?;
                handle_instruction(&Instruction::Push, &vec![ Token::Register(RegisterName::Ebp) ], ctx)?;
                handle_instruction(&Instruction::Mov, &vec![ Token::Register(RegisterName::Ebp), Token::Register(RegisterName::Esp) ], ctx)?;

                if s.starts_with("asmr::") {
                    call_builtin_fn(s, ctx)?;
                    handle_instruction(&Instruction::Ret, &vec![], ctx)
                }
                else {
                    // Keep params the same because it still holds the label
                    handle_instruction(&Instruction::Jmp, params, ctx)
                }
            }
            else {
                Err(AsmrRuntimeError::from(ctx.ptr, "`call` takes one parameter of type <Identifier>"))
            }
        },
        Instruction::Ret => {
            if params.len() != 0 { return Err(AsmrRuntimeError::from(ctx.ptr, "`ret` takes no parameters")) }

            /*
             * mov esp, ebp
             * pop ebp
             * pop eip
             */
            handle_instruction(&Instruction::Mov, &vec![ Token::Register(RegisterName::Esp), Token::Register(RegisterName::Ebp) ], ctx)?;
            handle_instruction(&Instruction::Pop, &vec![ Token::Register(RegisterName::Ebp) ], ctx)?;
            handle_instruction(&Instruction::Pop, &vec![ Token::Register(RegisterName::Eip) ], ctx)?;

            ctx.next = ctx.registers.get(&RegisterName::Eip).unwrap().borrow().get_raw() as usize + 1;

            Ok(())
        },
    }
}

/// Handles a variable declaration line
fn handle_variable(identifier: &String, mem_type: &MemType, params: &Vec<Token>, ctx: &mut ExecutingContext) -> Result<(), AsmrRuntimeError> {
    if *mem_type == MemType::Db {
        let mut bytes: Vec<u8> = Vec::new();
        for token in params {
            match token {
                Token::String(s) => bytes.append(&mut s.clone().into_bytes()),
                Token::Numeric(i) => bytes.append(&mut i.to_ne_bytes().to_vec()),
                Token::Identifier(_) => return Err(AsmrRuntimeError::from(ctx.ptr, "`db` takes parameters of type <...[String | Numeric]>")),
                Token::Register(_) => return Err(AsmrRuntimeError::from(ctx.ptr, "`db` takes parameters of type <...[String | Numeric]>")),
            };
        }
        ctx.heap.push(bytes);
        ctx.symtab.insert(identifier.to_string(), (ctx.heap.len() - 1).try_into().unwrap());
        Ok(())
    }
    else {
        if params.len() != 1 { return Err(AsmrRuntimeError::from(ctx.ptr, "`resb` takes one parameter of type <Numeric>")) }

        if let Some(Token::Numeric(i)) = params.first() {
            if *i <= 0 { return Err(AsmrRuntimeError::from(ctx.ptr, "`resb` requires the parameter <Numeric> to be greater than 0")) } // Must reserve a positive integer number of bytes
            ctx.heap.push(Vec::with_capacity(usize::try_from(*i).unwrap()));
            ctx.symtab.insert(identifier.to_string(), (ctx.heap.len() - 1).try_into().unwrap());
            Ok(())
        }
        else {
            // Must be a numeric value for resb
            Err(AsmrRuntimeError::from(ctx.ptr, "`resb` takes one parameter of type <Numeric>"))
        }
    }
}
