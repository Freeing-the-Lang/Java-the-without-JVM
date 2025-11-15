use crate::sponge_ir::SpongeIR;

pub struct MeaningRuntime;

impl MeaningRuntime {
    pub fn new() -> Self { Self }

    pub fn run(&mut self, program: &Vec<SpongeIR>) -> Option<i64> {
        if let Some(SpongeIR::Return(expr)) = program.first() {
            self.eval(expr)
        } else {
            None
        }
    }

    fn eval(&self, expr: &SpongeIR) -> Option<i64> {
        match expr {
            SpongeIR::Int(v) => Some(*v),
            SpongeIR::Add(a, b) => Some(self.eval(a)? + self.eval(b)?),
            SpongeIR::Return(e) => self.eval(e),
        }
    }
}
