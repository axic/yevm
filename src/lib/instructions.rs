use std::convert::From;

#[allow(dead_code)]
pub struct Metrics {
    cost: i16,
    num_args: i8,
    num_returns: i8,
}

pub enum Op {
    Stop,
    Add,
    Mul,
    Div,
    Push { size: u8 },
    Swap { size: u8 },

    Undefined,
}


impl From<u8> for Op {
    fn from(b: u8) -> Op {
        match b {
            0x00 => Op::Stop,
            0x01 => Op::Add,
            0x02 => Op::Mul,
            0x03 => Op::Div,
            0x60...0x7f => Op::Push { size: b - 0x60 + 1 },
            0x90...0x9f => Op::Swap { size: b - 0x90 + 1 },
            _ => Op::Undefined,
        }
    }
}

impl Op {
    pub fn metrics(self) -> Metrics {
        return match self {
            Op::Stop => Metrics { cost: 0, num_args: 0, num_returns: 0 },
            Op::Add => Metrics { cost: 3, num_args: 2, num_returns: 1 },
            Op::Mul => Metrics { cost: 3, num_args: 2, num_returns: 1 },
            Op::Div => Metrics { cost: 3, num_args: 2, num_returns: 1 },
            Op::Push { size: _ } => Metrics { cost: 2, num_args: 0, num_returns: 1 },
            Op::Swap { size } => Metrics { cost: 2, num_args: size as i8, num_returns: size as i8 },
            Op::Undefined => panic!("Ooops")
        };
    }
}
