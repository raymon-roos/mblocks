use crate::block::Block;
#[allow(unused_imports)]
use crate::block::BlockType::{Once, Periodic, Signal, PeriodicOrSignal};
#[allow(unused_imports)]
use crate::block::CommandType::{Function, Shell};

use crate::blocks::cpu::cpu_usage;
use crate::blocks::datetime::current_time;
use crate::blocks::memory::memory_usage;

pub const SEPARATOR: &str = " | ";
pub const PREFIX: &str = " ";
pub const SUFFIX: &str = " ";

pub const BLOCKS: &[Block] = &[
    Block {
        kind: Periodic(1),
        command: Function(cpu_usage),
        prefix: "CPU: ",
        suffix: "%",
    },
    Block {
        kind: Periodic(1),
        command: Function(memory_usage),
        prefix: "MEM: ",
        suffix: "",
    },
    Block {
        kind: PeriodicOrSignal(5, 1),
        command: Shell(&["date", "+%a, %b %d %Y %H:%M:%S"]),
        prefix: "",
        suffix: "",
    },
    Block {
        kind: Periodic(30),
        command: Function(current_time),
        prefix: "",
        suffix: "",
    },
    Block {
        kind: Once,
        command: Shell(&["whoami"]),
        prefix: "",
        suffix: "",
    },
];
