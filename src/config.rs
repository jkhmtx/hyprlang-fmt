#[derive(clap::ValueEnum, PartialEq, Clone, Copy, Debug, Default)]
pub enum IndentMode {
    Tabs,
    #[default]
    Spaces,
}

impl std::fmt::Display for IndentMode {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::result::Result<(), std::fmt::Error> {
        formatter.write_str(match self {
            Self::Tabs => "tabs",
            Self::Spaces => "spaces",
        })
    }
}

#[derive(clap::ValueEnum, PartialEq, Clone, Copy, Debug, Default)]
pub enum CommentSpacingContext {
    #[default]
    Block,
    Category,
    File,
}

impl std::fmt::Display for CommentSpacingContext {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::result::Result<(), std::fmt::Error> {
        formatter.write_str(match self {
            Self::Block => "block",
            Self::Category => "category",
            Self::File => "file",
        })
    }
}

#[derive(clap::ValueEnum, PartialEq, Clone, Copy, Debug, Default)]
pub enum CommandRhsSpacingMode {
    #[default]
    Compact,
    Equidistant,
}

impl std::fmt::Display for CommandRhsSpacingMode {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::result::Result<(), std::fmt::Error> {
        formatter.write_str(match self {
            Self::Compact => "compact",
            Self::Equidistant => "equidistant",
        })
    }
}

#[derive(clap::ValueEnum, PartialEq, Clone, Copy, Debug, Default)]
pub enum CommandRhsSpacingContext {
    #[default]
    Block,
    Category,
}

impl std::fmt::Display for CommandRhsSpacingContext {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::result::Result<(), std::fmt::Error> {
        formatter.write_str(match self {
            Self::Block => "block",
            Self::Category => "category",
        })
    }
}

#[derive(PartialEq, Clone, Copy, Debug, Default)]
pub struct Config {
    pub indent_width: u8,
    pub indent_mode: IndentMode,
    pub comment_spacing_context: CommentSpacingContext,
    pub command_rhs_spacing_mode: CommandRhsSpacingMode,
    pub command_rhs_spacing_context: CommandRhsSpacingContext,
}
