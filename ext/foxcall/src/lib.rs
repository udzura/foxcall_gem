use magnus::{function, prelude::*, Error, Ruby};

pub mod foxcall;
pub mod parser;
pub mod vm;

fn call(src: String) -> Result<String, Error> {
    do_call(src, None)
}

fn call_with_input(src: String, input: Option<String>) -> Result<String, Error> {
    do_call(src, input)
}

fn do_call(src: String, input: Option<String>) -> Result<String, Error> {
    let ruby = Ruby::get().expect("failed to get Ruby");
    foxcall::execute(&src, input.as_deref().unwrap_or("").as_bytes())
        .map_err(|e| Error::new(ruby.exception_runtime_error(), e))
}

fn convert_bf_to_foxcall(src: String) -> String {
    foxcall::translate_bf_into_foxcall(&src)
}

#[magnus::init]
fn init(ruby: &Ruby) -> Result<(), Error> {
    let module = ruby.define_module("Foxcall")?;
    module.define_singleton_method("call", function!(call, 1))?;
    module.define_singleton_method("call_with_input", function!(call_with_input, 2))?;
    module.define_singleton_method("convert_bf_to_foxcall", function!(convert_bf_to_foxcall, 1))?;

    Ok(())
}
