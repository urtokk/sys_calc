use std::error;
use crate::parsemath::parser::Node;

pub fn eval(expr: Node) -> Result<f64, Box<dyn error::Error>> {
  use self::Node::*;
  match expr {
    Num(num) => Ok(num),
    Add(left, right) => Ok(eval(*left)? + eval(*right)?),
    Subtract(left, right) => Ok(eval(*left)? - eval(*right)?),
    Multiply(left, right) => Ok(eval(*left)? * eval(*right)?),
    Divide(left, right) => Ok(eval(*left)? / eval(*right)?),
    Negative(left) => Ok(-eval(*left)?),
    Caret(left, right) => Ok(eval(*left)?.powf(eval(*right)?)),
  }
}

