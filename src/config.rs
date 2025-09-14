#[derive(clap::ValueEnum, PartialEq, Clone, Copy, Debug, Default)]
pub enum IndentMode {
    Tabs,
    #[default]
    Spaces,
}

impl IndentMode {
    pub fn into_str(self) -> &'static str {
        match self {
            IndentMode::Tabs => "\t",
            IndentMode::Spaces => " ",
        }
    }
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

#[derive(clap::ValueEnum, PartialEq, Clone, Copy, Debug, Default, Eq, Hash)]
pub enum SpacingContext {
    Block,
    #[default]
    Category,
    File,
}

impl std::fmt::Display for SpacingContext {
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

#[derive(PartialEq, Clone, Copy, Debug, Default)]
pub struct Config {
    pub indent_width: u8,
    pub indent_mode: IndentMode,
    pub eq_spacing_context: SpacingContext,
    pub comment_spacing_context: SpacingContext,
}
