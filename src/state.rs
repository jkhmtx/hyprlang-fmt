use crate::components::node::Node;
use crate::format::{Sections, Width};

#[derive(PartialEq, Debug)]
pub struct LengthsInclusive {
    pub lhs: usize,
    pub mid: usize,
    pub rhs: Option<usize>,
}

#[derive(PartialEq, Debug)]
pub struct BlockState {
    // The indentation level of the block
    pub level: u8,
    pub max_lengths: Option<LengthsInclusive>,
}

impl Width for BlockState {
    fn lhs_width(&self) -> usize {
        match &self.max_lengths {
            Some(lengths) => lengths.lhs,
            _ => 0,
        }
    }

    fn total_width(&self, config: &Config) -> usize {
        match &self.max_lengths {
            Some(lengths) => {
                lengths.rhs.unwrap_or(lengths.mid) + usize::from(config.tab_width * self.level)
            }
            _ => 0,
        }
    }
}

impl BlockState {
    pub fn new(nodes: &[Node], level: u8) -> Self {
        let max_lengths = {
            let mut lhs = 0;
            let mut mid = 0;
            let mut rhs = None;

            for section in nodes.iter().filter_map(Sections::as_sections) {
                lhs = std::cmp::max(lhs, section.lhs.len());
                mid = std::cmp::max(mid, lhs + section.mid.len());
                rhs = match (rhs, section.rhs) {
                    (Some(rhs), Some(section_rhs)) => {
                        Some(std::cmp::max(rhs, mid + section_rhs.len()))
                    }
                    (Some(rhs), None) => Some(rhs),
                    _ => None,
                };
            }

            if lhs == 0 {
                None
            } else {
                Some(LengthsInclusive { lhs, mid, rhs })
            }
        };

        BlockState { level, max_lengths }
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct Config {
    pub tab_width: u8,
}
