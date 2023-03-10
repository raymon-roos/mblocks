use crate::block::Block;
#[allow(unused_imports)]
use crate::block::BlockType::{Interval, Once, Signal};
#[allow(unused_imports)]
use crate::block::CommandType::{Function, Shell};

use crate::blocks::cpu::cpu_usage;
use crate::blocks::datetime::{current_time, current_date};
use crate::blocks::memory::memory_usage;

pub const SEPARATOR: &str = "  ";
pub const PREFIX: &str = " ";
pub const SUFFIX: &str = " ";

pub const BLOCKS: &[Block] = &[
    Block {
        kind: Interval(30),
        command: Function(cpu_usage),
        prefix: " ",
        suffix: "%",
    },
    Block {
        kind: Interval(30),
        command: Function(memory_usage),
        prefix: " ",
        suffix: "",
    },
    Block {
        kind: Interval(1800),
        command: Function(current_date),
        prefix: " ",
        suffix: "",
    },
    Block {
        kind: Interval(60),
        command: Function(current_time),
        prefix: "",
        suffix: "",
    },
];
