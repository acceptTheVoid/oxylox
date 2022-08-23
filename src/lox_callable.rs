use crate::interpreter::Interpreter;
use crate::value::Value;

pub trait Callable<O> {
    fn call(&self, interpreter: &mut Interpreter, args: &[Value]) -> O;
    fn arity(&self) -> usize;
}
