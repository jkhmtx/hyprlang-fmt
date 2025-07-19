#[derive(PartialEq)]
pub struct BlockState {
    // The indentation level of the block
    pub level: u8,
    // The longest identifier in the block's length
    pub lhs_max_length: usize,
    // The longest statement in the block's length
    pub max_length: usize,
}

#[derive(PartialEq, Clone, Copy)]
pub struct Config {
    pub tab_width: u8,
}
