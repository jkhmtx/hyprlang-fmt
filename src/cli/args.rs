use crate::config::{Config, IndentMode, SpacingContext};
use clap::Parser;
use std::io::Read;

#[allow(clippy::doc_markdown)]
///  _                      _                           __           _
/// | |                    | |                         / _|         | |
/// | |__  _   _ _ __  _ __| | __ _ _ __   __ _ ______| |_ _ __ ___ | |_
/// | '_ \| | | | '_ \| '__| |/ _` | '_ \ / _` |______|  _| '_ ` _ \| __|
/// | | | | |_| | |_) | |  | | (_| | | | | (_| |      | | | | | | | | |_
/// |_| |_|\__, | .__/|_|  |_|\__,_|_| |_|\__, |      |_| |_| |_| |_|\__|
///         __/ | |                        __/ |
///        |___/|_|                       |___/
///
/// A (the?) formatter for hyprlang.
#[derive(Parser, Debug, Default)]
#[command(version, verbatim_doc_comment)]
pub struct Args {
    /// How many chars to use for indentation
    ///
    /// If not specified, default is 2 (spaces)
    /// If not specified, and "--indent-mode=tab", default is 1
    #[arg(long, verbatim_doc_comment)]
    pub indent_width: Option<u8>,

    /// Whether to use tabs or spaces for indentation.
    #[arg(long, verbatim_doc_comment, default_value_t)]
    pub indent_mode: IndentMode,
    /// When offsetting equal signs for equal width spacing, determine what spacing context to use.
    ///
    /// - in "block" context, equal signs in the same block will be aligned. A block is any group of contiguous statements (lines).
    /// - in "category" context, equal signs in the same category will be aligned. Lines outside of categories will be treated with "block" context.
    /// - in "file" context, equal signs in the entire file will be aligned.
    #[arg(long, verbatim_doc_comment, default_value_t)]
    pub eq_spacing_context: SpacingContext,
    /// When offsetting trailing comments for equal width spacing, determine what spacing context to use.
    ///
    /// - in "block" context, trailing comments in the same block will start at the same position. A block is any group of contiguous statements (lines).
    // FIXME: Implement me!
    // /// - in "category" context, trailing comments in the same category will start at the same position. Lines outside of categories will be treated with "block" context.
    /// - in "file" context, trailing comments in the entire file will start at the same position.
    #[arg(long, verbatim_doc_comment, default_value_t)]
    pub comment_spacing_context: SpacingContext,
}

impl From<Args> for Config {
    fn from(args: Args) -> Self {
        let indent_width = args.indent_width.unwrap_or({
            match args.indent_mode {
                IndentMode::Tabs => 1,
                IndentMode::Spaces => 2,
            }
        });

        let indent_mode = args.indent_mode;
        let eq_spacing_context = args.eq_spacing_context;
        let comment_spacing_context = args.comment_spacing_context;

        Self {
            indent_width,
            indent_mode,
            eq_spacing_context,
            comment_spacing_context,
        }
    }
}

pub fn get_config() -> Config {
    let args = Args::parse();

    args.into()
}

pub fn get_file() -> String {
    let mut file = String::new();
    std::io::stdin()
        .read_to_string(&mut file)
        .expect("Unable to read stdin.");

    file
}
