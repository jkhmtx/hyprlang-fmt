use crate::components::node::Node;

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
                    (None, Some(section_rhs)) => Some(mid + section_rhs.len()),
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

pub struct SectionsView<'a> {
    pub lhs: &'a str,
    pub mid: &'a str,
    pub rhs: Option<&'a str>,
}

pub trait Sections {
    fn as_sections(&self) -> Option<SectionsView<'_>>;
}
