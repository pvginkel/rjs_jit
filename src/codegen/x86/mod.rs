mod emit;

use self::emit::Emit;
pub use self::emit::Reg;

pub struct Codegen {
    emit: Emit
}

impl Codegen {
    pub fn new() -> Codegen {
        Codegen {
            emit: Emit::new()
        }
    }
}
