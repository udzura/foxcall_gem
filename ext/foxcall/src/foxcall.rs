#[derive(Debug, Clone, Copy)]
pub enum Insn {
    IncrPrt,
    DecrPrt,
    IncrVal,
    DecrVal,
    Print,
    Scan,
    JumpFwd(usize),
    JumpBwd(usize),
}

pub use crate::parser::parse;
pub use crate::vm::Machine;

pub fn execute(src: &str, input: &[u8]) -> Result<String, String> {
    let insns = parse(src)?;
    let mut machine = Machine::new(insns, input.to_vec());
    machine.run();
    Ok(String::from_utf8_lossy(&machine.into_output()).into_owned())
}

pub fn translate_bf_into_foxcall(src: &str) -> String {
    let mut result = String::new();
    for c in src.chars() {
        match c {
            '>' => result.push_str("ルー"),
            '<' => result.push_str("ルルー"),
            '+' => result.push_str("ルルルー"),
            '-' => result.push_str("ルルルルー"),
            '.' => result.push_str("ルルルルルー"),
            ',' => result.push_str("ビー"),
            '[' => result.push_str("ルビー"),
            ']' => result.push_str("カイギ"),
            _ => {}
        }
    }
    result
}

#[cfg(target_arch = "wasm32")]
mod wasm {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    pub fn execute(src: &str, stdin: &str) -> Result<String, JsError> {
        crate::execute(src, stdin.as_bytes()).map_err(|e| JsError::new(&e))
    }

    #[wasm_bindgen]
    pub fn translate_bf_into_foxcall(src: &str) -> String {
        crate::translate_bf_into_foxcall(src)
    }
}
