use crate::ast::*;

#[derive(Debug, Clone)]
pub enum SpongeIR {
    Int(i64),
    Add(Box<SpongeIR>, Box<SpongeIR>),
    Return(Box<SpongeIR>),
}

pub fn convert_to_ir(class: &JavaClass) -> Vec<SpongeIR> {
    class.methods.iter().map(|m| convert_expr(&m.body)).collect()
}

fn convert_expr(expr: &JavaExpr) -> SpongeIR {
    match expr {
        JavaExpr::Int(v) => SpongeIR::Int(*v),
        JavaExpr::Add(a, b) => {
            SpongeIR::Add(
                Box::new(convert_expr(a)),
                Box::new(convert_expr(b)),
            )
        }
        JavaExpr::Return(e) => SpongeIR::Return(Box::new(convert_expr(e))),
    }
}
