use std::env;

use crate::types::obj::Prog;
use crate::types::obj2::Prog2;

use crate::declare::skeleton::Skeleton;
use crate::declare::skeleton2::Skeleton as Skeleton2;

impl Skeleton for Prog {}

pub fn exec() {
    let args: Vec<String> = std::env::args().collect();
    let arg = args[1];

    let prog = Prog {};
    let prog2 = Prog2 {};

    let a = prog.f(arg);
    let b = prog2.f(arg);
}
