use std::{env, path::Path, fmt::Display};
mod files;
mod parser;
mod tokenizer;
mod compiler;

fn main() {
    let mut args = env::args_os();
    args.next(); // Skip the first argument
    let arg = &args.next();
    let path = match arg {
        Some(n) => Path::new(n),
        None => {
            println!("Please provide the location of the Fire project to compile.");
            return;
        }
    };
    match compile_project_normal(path) {
        Ok(_) => todo!(),
        Err(d) => println!("{}", d)
    }
}

// Compile a normal project
fn compile_project_normal(path: &Path) -> Result<crate::compiler::CompiledCommands, Box<dyn Display>> {
    if let Err(r) = files::verify_project_format(path) {
        return Err(Box::new(r));
    }
    println!("{:?}", files::get_all_project_files(path));
    todo!();
}