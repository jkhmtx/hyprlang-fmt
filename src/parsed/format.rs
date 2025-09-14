use core::fmt;
use std::collections::HashMap;
use std::error::Error;

use crate::config::SpacingContext;

use crate::parsed::line::{Line, LineInfo};

use crate::config::Config;

use std::rc::Rc;

#[derive(Clone, Copy, Debug, Default)]
struct Sizes {
    max_len: usize,
}

#[derive(Clone, Debug, Default)]
struct GroupingInfo {
    block: Rc<[Sizes]>,
    category: Rc<[Sizes]>,
    file: Sizes,
}

#[derive(Debug)]
pub struct Table<'a> {
    rows: Vec<String>,
    lines: &'a Rc<[Line<'a>]>,
    grouping_info: Option<GroupingInfo>,
}

#[derive(Debug)]
pub enum TableError {
    InvalidIndex(usize),
}

impl Error for TableError {}

impl fmt::Display for TableError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TableError::InvalidIndex(idx) => write!(f, "invalid index: {idx}"),
        }
    }
}

type TableResult<T = ()> = Result<T, TableError>;

impl<'a> Table<'a> {
    fn new(config: Config, lines: &'a Rc<[Line<'_>]>) -> Self {
        let mut rows = vec![];

        let leading_whitespace_char = config.indent_mode.into_str();

        for line in lines.iter() {
            let Some(info) = (match line {
                Line::CategoryStart(line_info)
                | Line::CategoryEnd(line_info)
                | Line::Comment(line_info)
                | Line::Sectioned(line_info) => Some(line_info),
                Line::Newline => None,
            }) else {
                rows.push(String::new());
                continue;
            };

            let repeat = usize::from(info.indent * config.indent_width);

            let mut cell = leading_whitespace_char.repeat(repeat);

            cell.push_str(info.lhs);

            match (line, info.comment_text) {
                (Line::CategoryStart(_), _) => {
                    cell.push_str(" {");
                }
                (Line::Comment(_), Some(text)) => {
                    cell.push_str(text);
                }
                _ => {}
            }

            rows.push(cell);
        }

        Self {
            rows,
            lines,
            grouping_info: None,
        }
    }

    fn get_sizes_for_pos(&self, context: SpacingContext, line: &LineInfo<'_>) -> Sizes {
        let Some(grouping_info) = &self.grouping_info else {
            panic!("Tried to append spaces without setting grouping info");
        };

        match context {
            SpacingContext::Block => grouping_info.block[line.group_id as usize],
            SpacingContext::Category => {
                // Use block grouping when category grouping is not applicable
                if line.indent != 0 {
                    grouping_info.category[line.category_id as usize]
                } else {
                    grouping_info.block[line.group_id as usize]
                }
            }
            SpacingContext::File => grouping_info.file,
        }
    }

    pub fn append_spaces(&mut self, context: SpacingContext, line: &LineInfo<'_>, pos: usize) {
        let sizes = self.get_sizes_for_pos(context, line);

        let row = &mut self.rows[pos];

        let spaces = " ".repeat({
            sizes
                .max_len
                .checked_sub(row.len())
                .expect("cell length underflowed the longest string by being too big")
        });

        row.push_str(&spaces);
    }

    pub fn append_to_row(&mut self, idx: usize, string: &str) -> TableResult {
        let row: &mut String = self
            .rows
            .get_mut(idx)
            .ok_or(TableError::InvalidIndex(idx))?;

        row.push_str(string);

        Ok(())
    }

    pub fn set_next_grouping_info(&mut self) {
        let file = Sizes {
            max_len: self
                .rows
                .iter()
                .enumerate()
                .filter_map(|(pos, str)| self.lines[pos].as_sectionable().map(|_| str))
                .map(std::string::String::len)
                .max()
                .unwrap_or(0),
        };

        let pos_id_groups = self.lines.iter().enumerate().filter_map(|(pos, line)| {
            line.as_groupable().map(|info| {
                (
                    pos,
                    (
                        matches!(line, Line::Comment(_)),
                        info.group_id,
                        info.category_id,
                    ),
                )
            })
        });

        let pos_group_id = pos_id_groups
            .clone()
            .map(|(pos, (is_comment, id, _))| (is_comment, id, pos));
        let pos_category_id = pos_id_groups.map(|(pos, (is_comment, _, id))| (is_comment, id, pos));

        let block = self.build_group_info_group_array(pos_group_id).into();
        let category = self.build_group_info_group_array(pos_category_id).into();

        self.grouping_info = Some(GroupingInfo {
            block,
            category,
            file,
        });
    }

    // Since group_id and category_id are zero-indexed and sequential,
    // a lookup table for them can be a simple Vec.
    // The usize in question is the largest lhs len for the given group.
    fn build_group_info_group_array(
        &self,
        items: impl Iterator<Item = (bool, u16, usize)>,
    ) -> Vec<Sizes> {
        let group_ids_map = items.fold(
            HashMap::<u16, Vec<(bool, usize)>>::new(),
            |mut acc, (is_comment, id, pos)| {
                let Some(positions) = acc.get_mut(&id) else {
                    acc.insert(id, vec![(is_comment, pos)]);

                    return acc;
                };

                positions.push((is_comment, pos));

                acc
            },
        );

        let mut ids = group_ids_map.keys().collect::<Vec<_>>();

        ids.sort_unstable();

        let mut groups = vec![];
        for id in ids {
            let positions = group_ids_map.get(id).unwrap();
            let group = positions.iter().filter_map(|(is_comment, pos)| {
                if *is_comment {
                    None
                } else {
                    self.rows.get(*pos)
                }
            });

            let max_len = group
                .clone()
                .map(std::string::String::len)
                .max()
                .unwrap_or(0);
            groups.push(Sizes { max_len });
        }

        groups
    }

    pub fn format(self) -> String {
        self.rows
            .iter()
            .map(|row| row.trim_end_matches(' '))
            .collect::<Vec<_>>()
            .join("\n")
            + "\n"
    }
}

pub fn format_lines(lines: &Rc<[Line<'_>]>, config: Config) -> TableResult<String> {
    let mut table = Table::new(config, lines);

    update_mid_column(config, lines, &mut table)?;
    update_rhs_column(lines, &mut table)?;
    update_comment_column(config, lines, &mut table)?;

    Ok(table.format())
}

pub fn update_mid_column(config: Config, lines: &Rc<[Line]>, table: &mut Table) -> TableResult {
    table.set_next_grouping_info();
    for (pos, line) in lines.iter().enumerate() {
        let Some(info) = line.as_sectionable() else {
            continue;
        };

        table.append_spaces(config.eq_spacing_context, info, pos);

        let mid = if info.rhs.is_none() { " =" } else { " = " };

        table.append_to_row(pos, mid)?;
    }

    Ok(())
}

pub fn update_rhs_column(lines: &Rc<[Line]>, table: &mut Table) -> TableResult {
    for (pos, line) in lines.iter().enumerate() {
        if let Some(rhs) = line.as_sectionable().and_then(|info| info.rhs) {
            table.append_to_row(pos, rhs)?;
        }
    }

    Ok(())
}

pub fn update_comment_column(config: Config, lines: &Rc<[Line]>, table: &mut Table) -> TableResult {
    table.set_next_grouping_info();
    for (pos, line) in lines.iter().enumerate() {
        let Some(info) = line.as_sectionable() else {
            continue;
        };
        let Some(hashes) = info.comment_hashes else {
            continue;
        };

        table.append_spaces(config.comment_spacing_context, info, pos);
        table.append_to_row(pos, " ")?;
        table.append_to_row(pos, hashes)?;

        if !hashes.ends_with(' ') {
            table.append_to_row(pos, " ")?;
        }

        let Some(text) = info.comment_text else {
            continue;
        };
        table.append_to_row(pos, text)?;
    }

    Ok(())
}
