use std::rc::Rc;

use crate::{grammar::Rule, parse::ParseIterator};

#[derive(Debug, Clone, PartialEq)]
pub struct LineInfo<'a> {
    pub category_id: u16,
    pub comment_hashes: Option<&'a str>,
    pub comment_text: Option<&'a str>,
    pub group_id: u16,
    pub indent: u8,
    pub lhs: &'a str,
    pub rhs: Option<&'a str>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Line<'a> {
    Newline,
    Comment(LineInfo<'a>),
    Sectioned(LineInfo<'a>),
    CategoryStart(LineInfo<'a>),
    CategoryEnd(LineInfo<'a>),
}

impl<'a> Line<'a> {
    pub fn comment(category_id: u16, indent: u8, text: &'a str) -> Self {
        Self::Comment(LineInfo {
            group_id: u16::MAX,
            category_id: if indent == 0 { u16::MAX } else { category_id },
            indent,
            lhs: text,
            rhs: None,
            comment_hashes: None,
            comment_text: None,
        })
    }

    pub fn assignment(lhs: &'a str) -> Self {
        Self::Sectioned(LineInfo {
            group_id: u16::MAX,
            category_id: u16::MAX,
            indent: 0,
            lhs,
            rhs: None,
            comment_hashes: None,
            comment_text: None,
        })
    }

    pub fn bind(category_id: u16, indent: u8, lhs: &'a str) -> Self {
        Self::Sectioned(LineInfo {
            group_id: u16::MAX,
            category_id: if indent == 0 { u16::MAX } else { category_id },
            indent,
            lhs,
            rhs: None,
            comment_hashes: None,
            comment_text: None,
        })
    }

    pub fn category_start(category_id: u16, indent: u8, category_ident: &'a str) -> Self {
        Self::CategoryStart(LineInfo {
            group_id: u16::MAX,
            category_id,
            indent,
            lhs: category_ident,
            rhs: None,
            comment_hashes: None,
            comment_text: None,
        })
    }

    pub fn category_end(category_id: u16, indent: u8) -> Self {
        Self::CategoryEnd(LineInfo {
            group_id: u16::MAX,
            category_id,
            indent,
            lhs: "}",
            rhs: None,
            comment_hashes: None,
            comment_text: None,
        })
    }

    pub fn set_group_id(&mut self, group_id: u16) {
        match self {
            Self::Sectioned(line) | Self::Comment(line) => {
                line.group_id = group_id;
            }
            Self::Newline | Self::CategoryStart(_) | Self::CategoryEnd(_) => {
                panic!("cannot call on newline")
            }
        }
    }

    pub fn as_sectionable(&self) -> Option<&LineInfo<'a>> {
        match self {
            Self::Sectioned(info) => Some(info),
            _ => None,
        }
    }

    pub fn as_groupable(&self) -> Option<&LineInfo<'a>> {
        match self {
            Self::Sectioned(info) | Self::Comment(info) => Some(info),
            Self::Newline | Self::CategoryStart(_) | Self::CategoryEnd(_) => None,
        }
    }

    pub fn set_comment_hashes(&mut self, text: &'a str) {
        match self {
            Self::Sectioned(line) | Self::CategoryStart(line) | Self::CategoryEnd(line) => {
                line.comment_hashes = Some(text);
            }
            _ => panic!("cannot call on non-comment-having lines"),
        }
    }

    pub fn set_comment_text(&mut self, text: &'a str) {
        if text.trim_end_matches(' ').is_empty() {
            return;
        }

        match self {
            Self::Sectioned(line)
            | Self::CategoryStart(line)
            | Self::CategoryEnd(line)
            | Self::Comment(line) => {
                line.comment_text = Some(text);
            }
            Self::Newline => panic!("cannot call on non-comment-having lines"),
        }
    }

    pub fn set_rhs(&mut self, part: &'a str) {
        match self {
            Self::Sectioned(line) => {
                line.rhs = Some(part);
            }
            _ => panic!("can only set_rhs on a sectionable"),
        }
    }
}

pub fn get_lines(pairs: ParseIterator) -> Rc<[Line]> {
    let mut lines = get_lines_inner(pairs.map(|pair| (pair.as_rule(), pair.as_span().as_str())));

    // Remove trailing newlines at EOF
    while matches!(lines.last(), Some(Line::Newline)) {
        lines.pop();
    }

    let mut group_id: u16 = 0;
    let mut has_set_group_id = false;

    let lines_readonly = lines.clone();

    for (pos, line) in lines.iter_mut().enumerate() {
        let is_groupable = line.as_groupable().is_some();
        let last = lines_readonly.get(pos.saturating_sub(1));

        if let (Some(Line::Newline | Line::CategoryStart(_)), true) =
            (last, is_groupable && has_set_group_id)
        {
            group_id = group_id.checked_add(1).expect("Too many groups");
        }

        if is_groupable {
            has_set_group_id = true;
            line.set_group_id(group_id);
        }
    }

    lines.into()
}

#[expect(clippy::needless_continue)]
fn get_lines_inner<'a>(pairs: impl Iterator<Item = (Rule, &'a str)>) -> Vec<Line<'a>> {
    let mut lines: Vec<Line> = Vec::new();

    let mut indent: u8 = 0;
    let mut category_id = 0;

    let mut line = Line::Newline;
    for (rule, span_str) in pairs {
        let is_contentful_line =
            !matches!(rule, Rule::newline) && !matches!(rule, Rule::category_end);

        let adding_contentful_line_after_category_end =
            matches!(lines.last(), Some(Line::CategoryEnd(_))) && is_contentful_line;

        let was_contentful_line = !matches!(lines.last(), Some(Line::Newline))
            && !matches!(lines.last(), Some(Line::CategoryEnd(_)));

        let adding_category_start_after_contentful_line =
            was_contentful_line && matches!(rule, Rule::category);

        let adding_category_start_after_comment =
            matches!(lines.last(), Some(Line::Comment(_))) && matches!(rule, Rule::category);

        if !adding_category_start_after_comment
            && (adding_contentful_line_after_category_end
                || adding_category_start_after_contentful_line)
        {
            lines.push(Line::Newline);
        }

        match rule {
            Rule::newline => {
                if let (Some([Line::Newline, Line::Newline]), &Line::Newline) =
                    (lines.last_chunk::<2>(), &line)
                {
                    // Allow at most two consecutive newlines
                    continue;
                }

                if matches!(lines.last(), Some(Line::CategoryStart(_))) && line == Line::Newline {
                    // Do not allow empty newlines after category start
                    continue;
                }

                lines.push(line);
                line = Line::Newline;
            }
            Rule::category
            | Rule::category_inner
            | Rule::category_start
            | Rule::EOI
            | Rule::bind
            | Rule::bind_rule
            | Rule::comment
            | Rule::assignment => {
                continue;
            }
            Rule::comment_hashes => match line {
                Line::Sectioned(_) | Line::CategoryStart(_) | Line::CategoryEnd(_) => {
                    line.set_comment_hashes(span_str);
                }
                _ => {
                    line = Line::comment(category_id, indent, span_str);
                }
            },
            Rule::comment_text => match line {
                Line::Sectioned(_)
                | Line::CategoryStart(_)
                | Line::CategoryEnd(_)
                | Line::Comment(_) => {
                    line.set_comment_text(span_str.trim_end());
                }
                Line::Newline => unreachable!(),
            },
            Rule::bind_ident => {
                line = Line::bind(category_id, indent, span_str.trim_end());
            }
            Rule::variable_ident => {
                line = Line::assignment(span_str.trim_end());
            }
            Rule::bind_rhs | Rule::variable_expression => line.set_rhs(span_str.trim_end()),
            Rule::category_ident => {
                line = Line::category_start(category_id, indent, span_str.trim_end());
                indent = indent.checked_add(1).expect("exceeded max indentation");
            }
            Rule::category_end => {
                indent = indent
                    .checked_sub(1)
                    .expect("added category end without starting a category");

                line = Line::category_end(category_id, indent);

                if indent == 0 {
                    category_id += 1;
                }
            }
            a => {
                dbg!(a);
                unreachable!();
            }
        }
    }

    lines
}
