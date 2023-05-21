use std::io::{self, Write};

use crate::interpreter::error::AsmrRuntimeError;
use super::{executing_context::ExecutingContext, register::{RegisterData, RegisterName}};

/// Calls the appropriate handlers for the builtin asmr functions.
pub fn call_builtin_fn(func_name: &String, ctx: &mut ExecutingContext) -> Result<(), AsmrRuntimeError> {
    let func = match func_name.as_str() {
        "asmr::io::print" => io_print,
        "asmr::io::readln" => io_readln,
        _ => return Err(AsmrRuntimeError::from(ctx.ptr, format!("no function found: `{}`", func_name))),
    };

    func(ctx)
}

/// Gets the `n`th last parameter pushed to the stack before calling the function.
/// Parameters are pushed in reverse order, thus for a function with signature
/// ```c
/// int mul(int a, int b);
/// ```
/// the call would resemble
/// ```nasm
/// push b
/// push a
/// call mul
/// ```
/// meaning
/// ```rs
/// get_param(1, ctx) // a
/// get_param(2, ctx) // b
/// ```
fn get_param<'a>(n: i32, ctx: &'a &mut ExecutingContext) -> &'a RegisterData {
    ctx.stack.get((ctx.registers.get(&RegisterName::Ebp).unwrap().borrow().get_raw() - 1 - n) as usize).unwrap()
}

/// Clears `n` parameters from the stack.
/// Because calling a function creates a stack frame, the stack after a call resembles
/// ```nasm
/// param3
/// param2
/// param1
/// eip
/// ebp
/// ```
/// To preserve the stack frame, [clear_params] removes the parameters and updates the current ebp and esp.
/// The pushed `eip` and `ebp` making up the stack frame are preserved so `ret` functions as expected.
fn clear_params(n: i32, ctx: &mut ExecutingContext) {
    // Clear n parameters from the stack (leaves stack frame pushed for when 'ret' is called)
    let idx_stack_frame_start = ctx.registers.get(&RegisterName::Ebp).unwrap().borrow().get_raw() as usize - 1;
    ctx.stack.drain((idx_stack_frame_start - n as usize)..idx_stack_frame_start);

    // Update the position of ebp and esp
    *ctx.registers.get(&RegisterName::Ebp).unwrap().borrow_mut() -= n;
    *ctx.registers.get(&RegisterName::Esp).unwrap().borrow_mut() -= n;
}

fn io_print(ctx: &mut ExecutingContext) -> Result<(), AsmrRuntimeError> {
    let msg = match get_param(1, &ctx) {
        RegisterData::Value(i) => i.to_string(),
        RegisterData::Pointer(p) => ctx.heap.get(*p as usize).unwrap().iter()
                                        .map(|b| char::from(*b)).collect::<String>(),
    };
    print!("{}", msg);
    io::stdout()
        .flush()
        .map_err(|e| AsmrRuntimeError::from(ctx.ptr, format!("failed to flush stdout: {}", e)))?;

    clear_params(1, ctx);
    Ok(())
}

fn io_readln(ctx: &mut ExecutingContext) -> Result<(), AsmrRuntimeError> {
    let p = if let RegisterData::Pointer(p) = get_param(1, &ctx) { *p }
            else { return Err(AsmrRuntimeError::from(ctx.ptr, "expected pointer but found value")) };

    let mut str = String::new();
    io::stdin()
        .read_line(&mut str)
        .map_err(|e| AsmrRuntimeError::from(ctx.ptr, format!("failed to read from stdin: {}", e)))?;
    
    ctx.heap.get_mut(p as usize).unwrap()
        .clone_from(&str.trim().as_bytes().to_vec());
    
    clear_params(1, ctx);
    Ok(())
}
