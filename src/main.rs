mod ast;
mod parser;
mod sponge_ir;
mod runtime;
mod memory;
mod object;

use parser::parse_java;
use sponge_ir::convert_to_ir;
use runtime::MeaningRuntime;

fn main() {
    println!("=== Java-the-without-JVM ===");
    println!("Java → Sponge IR → Rust Meaning Runtime");
    println!("--------------------------------------");

    let src = r#"
        class Hello {
            int x = 10;
            int y = 22;

            int add() {
                return x + y;
            }
        }
    "#;

    // 1) Java Parsing
    println!("\n[1] Java Parsing...");
    let ast = parse_java(src);
    println!("{:#?}", ast);

    // 2) Sponge IR conversion
    println!("\n[2] Converting to Sponge IR...");
    let ir = convert_to_ir(&ast);
    println!("{:#?}", ir);

    // 3) Rust Meaning Runtime execute
    println!("\n[3] Executing Meaning Runtime...");
    let mut engine = MeaningRuntime::new();
    let result = engine.run(&ir);

    println!("\n=== EXECUTION RESULT ===");
    match result {
        Some(v) => println!("Result: {}", v),
        None => println!("(no result)"),
    }

    println!("--------------------------------------");
    println!("Java-the-without-JVM execution complete\n");
}
