use super::SynthRef;

#[derive(Clone, Debug)]
pub enum ArithmeticOp {
    Add,
    Mul,
}

#[derive(Clone, Debug)]
pub struct Arithmetic {
    operation: ArithmeticOp,
    operands: Vec<SynthRef>,
}

impl Arithmetic {
    pub fn new_desc(operation: ArithmeticOp, operands: Vec<SynthRef>) -> Arithmetic {
        Arithmetic {
            operation,
            operands,
        }
    }

    pub fn sample(&self, phase: f32) -> f32 {
        let results = self.operands.iter().map(|s| s.sample(phase));
        match self.operation {
            ArithmeticOp::Add => results.sum(),
            ArithmeticOp::Mul => results.reduce(|a, b| a * b).unwrap_or(1.0),
        }
    }
}
