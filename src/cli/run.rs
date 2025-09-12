use crate::config::Config;
use crate::parse::get_file_tokens_iterator;
use crate::parsed::format::format_lines;
use crate::parsed::line::get_lines;

pub fn run(config: Config, file: &str) {
    let parsed = parse(config, file);

    print!("{parsed}");
}

fn parse(config: Config, file: &str) -> String {
    let pairs = get_file_tokens_iterator(file).expect("error getting file tokens iterator");

    let lines = get_lines(pairs);

    format_lines(&lines, config).expect("error formatting lines")
}

#[cfg(test)]
mod tests {
    use std::sync::LazyLock;

    use super::*;
    use crate::cli::args::Args;
    use crate::config::SpacingContext;

    fn concat<'a, I>(parts: I) -> String
    where
        I: std::iter::IntoIterator<Item = &'a str>,
    {
        parts.into_iter().collect::<Vec<_>>().join("\n").to_owned()
    }

    fn assert_fmt(expected: &str, actual: &str) {
        let mut expected_inc = String::new();
        let mut actual_inc = String::new();

        let mut expected_lines = expected.split_terminator("\n").into_iter();
        let mut actual_lines = actual.split_terminator("\n").into_iter();

        let mut line_no = 0;
        while let (Some(expected_line), Some(actual_line)) =
            (expected_lines.next(), actual_lines.next())
        {
            line_no += 1;
            let line_no_str = &line_no.to_string();

            expected_inc.push_str(line_no_str);
            expected_inc.push('|');
            expected_inc.push_str(expected_line);

            actual_inc.push_str(line_no_str);
            actual_inc.push('|');
            actual_inc.push_str(actual_line);

            if expected_line != actual_line {
                let mut actual_context = actual_inc
                    .rsplit_terminator('\n')
                    .take(3)
                    .collect::<Vec<&str>>()
                    .into_iter()
                    .rev()
                    .collect::<Vec<&str>>()
                    .join("\n");
                let mut expected_context = expected_inc
                    .rsplit_terminator('\n')
                    .take(3)
                    .collect::<Vec<&str>>()
                    .into_iter()
                    .rev()
                    .collect::<Vec<&str>>()
                    .join("\n");

                actual_context.push_str("|<-- DETECTED");

                // Add two more
                let next_two = [actual_lines.next(), actual_lines.next()];
                for after in next_two.iter() {
                    line_no += 1;
                    let line_no_str = &line_no.to_string();
                    if let Some(after) = after {
                        actual_context.push_str("\n");
                        actual_context.push_str(line_no_str);
                        actual_context.push('|');
                        actual_context.push_str(after);
                    }
                }

                line_no -= 2;

                let next_two = [expected_lines.next(), expected_lines.next()];
                for after in next_two.iter() {
                    line_no += 1;
                    let line_no_str = &line_no.to_string();
                    if let Some(after) = after {
                        expected_context.push_str("\n");
                        expected_context.push_str(line_no_str);
                        expected_context.push('|');
                        expected_context.push_str(after);
                    }
                }

                assert_eq!(
                    expected_line,
                    actual_line,
                    "{}",
                    concat([
                        "",
                        "",
                        "------EXPECTED------",
                        &expected_context,
                        "--------------------",
                        "-------ACTUAL-------",
                        &actual_context,
                        "--------------------",
                    ])
                );
            }

            expected_inc.push_str("\n");
            actual_inc.push_str("\n");
        }
    }

    static FILE: LazyLock<String> = LazyLock::new(|| {
        concat([
            "################",
            "### MONITORS ###",
            "################  ",
            "  ",
            "# See https://wiki.hyprland.org/Configuring/Monitors/",
            "monitor=,preferred,auto,auto  ",
            "",
            "",
            "# hello",
            "foo=barbar",
            "foo=bar  ",
            "",
            "# long long long comment  ",
            "$variable=assignment  ",
            "",
            "bind=foo,bar  ",
            "bind=,bar",
            "bindl=,bar # comment",
            "bindl=;bar;baz # comment",
            "bindl=bar baz # comment",
            "",
            "ident {",
            "something=elseelseelseelseelse # foo",
            "something=$variable                    # foo",
            "}",
            "",
            "",
            "",
            "# this stays here",
            "ident2 {",
            "something=elseelseelseelse # foo",
            "something=elseelse                    # foo",
            "}",
            "bind=[command,command] #not-spaced",
            "bind=,a,b,##escaped,$d # wow",
            "ident2 {",
            "something=elseelseelseelse # foo",
            "something=elseelse                    # foo",
            "",
            "foo=bar",
            "foo=bar # comment",
            "",
            "ident3 {",
            "",
            "foooooooo=# nothing-here",
            "barbarbarbar=foooooooo",
            "}",
            "}",
            "",
            "",
            "foo = bar",
        ])
    });

    #[test]
    fn test_comment_spacing_context_category() {
        let args = Args::default();
        let mut config = Config::from(args);

        config.comment_spacing_context = SpacingContext::Category;

        let expected = concat([
            "################",
            "### MONITORS ###",
            "################",
            "",
            "# See https://wiki.hyprland.org/Configuring/Monitors/",
            "monitor = ,preferred,auto,auto",
            "",
            "",
            "# hello",
            "foo = barbar",
            "foo = bar",
            "",
            "# long long long comment",
            "$variable = assignment",
            "",
            "bind  = foo,bar",
            "bind  = ,bar",
            "bindl = ,bar     # comment",
            "bindl = ;bar;baz # comment",
            "bindl = bar baz  # comment",
            "",
            "ident {",
            "  something = elseelseelseelseelse # foo",
            "  something = $variable            # foo",
            "}",
            "",
            "",
            "# this stays here",
            "ident2 {",
            "  something = elseelseelseelse # foo",
            "  something = elseelse         # foo",
            "}",
            "",
            "bind = [command,command] # not-spaced",
            "bind = ,a,b,##escaped,$d # wow",
            "",
            "ident2 {",
            "  something      = elseelseelseelse # foo",
            "  something      = elseelse         # foo",
            "",
            "  foo            = bar",
            "  foo            = bar              # comment",
            "",
            "  ident3 {",
            "    foooooooo    =                  # nothing-here",
            "    barbarbarbar = foooooooo",
            "  }",
            "}",
            "",
            "",
            "foo = bar",
        ]);

        let actual = parse(config, &FILE);

        assert_fmt(&expected, &actual);
    }

    #[test]
    fn test_comment_spacing_context_block() {
        let args = Args::default();
        let mut config = Config::from(args);

        config.comment_spacing_context = SpacingContext::Block;

        let expected = concat([
            "################",
            "### MONITORS ###",
            "################",
            "",
            "# See https://wiki.hyprland.org/Configuring/Monitors/",
            "monitor = ,preferred,auto,auto",
            "",
            "",
            "# hello",
            "foo = barbar",
            "foo = bar",
            "",
            "# long long long comment",
            "$variable = assignment",
            "",
            "bind  = foo,bar",
            "bind  = ,bar",
            "bindl = ,bar     # comment",
            "bindl = ;bar;baz # comment",
            "bindl = bar baz  # comment",
            "",
            "ident {",
            "  something = elseelseelseelseelse # foo",
            "  something = $variable            # foo",
            "}",
            "",
            "",
            "# this stays here",
            "ident2 {",
            "  something = elseelseelseelse # foo",
            "  something = elseelse         # foo",
            "}",
            "",
            "bind = [command,command] # not-spaced",
            "bind = ,a,b,##escaped,$d # wow",
            "",
            "ident2 {",
            "  something      = elseelseelseelse # foo",
            "  something      = elseelse         # foo",
            "",
            "  foo            = bar",
            "  foo            = bar # comment",
            "",
            "  ident3 {",
            "    foooooooo    =           # nothing-here",
            "    barbarbarbar = foooooooo",
            "  }",
            "}",
            "",
            "",
            "foo = bar",
        ]);

        let actual = parse(config, &FILE);

        assert_fmt(&expected, &actual);
    }

    #[test]
    fn test_comment_spacing_context_file() {
        let args = Args::default();
        let mut config = Config::from(args);

        config.comment_spacing_context = SpacingContext::File;

        let expected = concat([
            "################",
            "### MONITORS ###",
            "################",
            "",
            "# See https://wiki.hyprland.org/Configuring/Monitors/",
            "monitor = ,preferred,auto,auto",
            "",
            "",
            "# hello",
            "foo = barbar",
            "foo = bar",
            "",
            "# long long long comment",
            "$variable = assignment",
            "",
            "bind  = foo,bar",
            "bind  = ,bar",
            "bindl = ,bar                        # comment",
            "bindl = ;bar;baz                    # comment",
            "bindl = bar baz                     # comment",
            "",
            "ident {",
            "  something = elseelseelseelseelse  # foo",
            "  something = $variable             # foo",
            "}",
            "",
            "",
            "# this stays here",
            "ident2 {",
            "  something = elseelseelseelse      # foo",
            "  something = elseelse              # foo",
            "}",
            "",
            "bind = [command,command]            # not-spaced",
            "bind = ,a,b,##escaped,$d            # wow",
            "",
            "ident2 {",
            "  something      = elseelseelseelse # foo",
            "  something      = elseelse         # foo",
            "",
            "  foo            = bar",
            "  foo            = bar              # comment",
            "",
            "  ident3 {",
            "    foooooooo    =                  # nothing-here",
            "    barbarbarbar = foooooooo",
            "  }",
            "}",
            "",
            "",
            "foo = bar",
        ]);

        let actual = parse(config, &FILE);

        assert_fmt(&expected, &actual);
    }

    #[test]
    fn test_eq_spacing_context_category() {
        let args = Args::default();
        let mut config = Config::from(args);

        config.eq_spacing_context = SpacingContext::Category;

        let expected = concat([
            "################",
            "### MONITORS ###",
            "################",
            "",
            "# See https://wiki.hyprland.org/Configuring/Monitors/",
            "monitor = ,preferred,auto,auto",
            "",
            "",
            "# hello",
            "foo = barbar",
            "foo = bar",
            "",
            "# long long long comment",
            "$variable = assignment",
            "",
            "bind  = foo,bar",
            "bind  = ,bar",
            "bindl = ,bar     # comment",
            "bindl = ;bar;baz # comment",
            "bindl = bar baz  # comment",
            "",
            "ident {",
            "  something = elseelseelseelseelse # foo",
            "  something = $variable            # foo",
            "}",
            "",
            "",
            "# this stays here",
            "ident2 {",
            "  something = elseelseelseelse # foo",
            "  something = elseelse         # foo",
            "}",
            "",
            "bind = [command,command] # not-spaced",
            "bind = ,a,b,##escaped,$d # wow",
            "",
            "ident2 {",
            "  something      = elseelseelseelse # foo",
            "  something      = elseelse         # foo",
            "",
            "  foo            = bar",
            "  foo            = bar              # comment",
            "",
            "  ident3 {",
            "    foooooooo    =                  # nothing-here",
            "    barbarbarbar = foooooooo",
            "  }",
            "}",
            "",
            "",
            "foo = bar",
        ]);

        let actual = parse(config, &FILE);

        assert_fmt(&expected, &actual);
    }

    #[test]
    fn test_eq_spacing_context_block() {
        let args = Args::default();
        let mut config = Config::from(args);

        config.eq_spacing_context = SpacingContext::Block;

        let expected = concat([
            "################",
            "### MONITORS ###",
            "################",
            "",
            "# See https://wiki.hyprland.org/Configuring/Monitors/",
            "monitor = ,preferred,auto,auto",
            "",
            "",
            "# hello",
            "foo = barbar",
            "foo = bar",
            "",
            "# long long long comment",
            "$variable = assignment",
            "",
            "bind  = foo,bar",
            "bind  = ,bar",
            "bindl = ,bar     # comment",
            "bindl = ;bar;baz # comment",
            "bindl = bar baz  # comment",
            "",
            "ident {",
            "  something = elseelseelseelseelse # foo",
            "  something = $variable            # foo",
            "}",
            "",
            "",
            "# this stays here",
            "ident2 {",
            "  something = elseelseelseelse # foo",
            "  something = elseelse         # foo",
            "}",
            "",
            "bind = [command,command] # not-spaced",
            "bind = ,a,b,##escaped,$d # wow",
            "",
            "ident2 {",
            "  something = elseelseelseelse # foo",
            "  something = elseelse         # foo",
            "",
            "  foo = bar",
            "  foo = bar                    # comment",
            "",
            "  ident3 {",
            "    foooooooo    =             # nothing-here",
            "    barbarbarbar = foooooooo",
            "  }",
            "}",
            "",
            "",
            "foo = bar",
        ]);

        let actual = parse(config, &FILE);

        assert_fmt(&expected, &actual);
    }

    #[test]
    fn test_eq_spacing_context_file() {
        let args = Args::default();
        let mut config = Config::from(args);

        config.eq_spacing_context = SpacingContext::File;

        let expected = concat([
            "################",
            "### MONITORS ###",
            "################",
            "",
            "# See https://wiki.hyprland.org/Configuring/Monitors/",
            "monitor          = ,preferred,auto,auto",
            "",
            "",
            "# hello",
            "foo              = barbar",
            "foo              = bar",
            "",
            "# long long long comment",
            "$variable        = assignment",
            "",
            "bind             = foo,bar",
            "bind             = ,bar",
            "bindl            = ,bar     # comment",
            "bindl            = ;bar;baz # comment",
            "bindl            = bar baz  # comment",
            "",
            "ident {",
            "  something      = elseelseelseelseelse # foo",
            "  something      = $variable            # foo",
            "}",
            "",
            "",
            "# this stays here",
            "ident2 {",
            "  something      = elseelseelseelse # foo",
            "  something      = elseelse         # foo",
            "}",
            "",
            "bind             = [command,command] # not-spaced",
            "bind             = ,a,b,##escaped,$d # wow",
            "",
            "ident2 {",
            "  something      = elseelseelseelse # foo",
            "  something      = elseelse         # foo",
            "",
            "  foo            = bar",
            "  foo            = bar              # comment",
            "",
            "  ident3 {",
            "    foooooooo    =                  # nothing-here",
            "    barbarbarbar = foooooooo",
            "  }",
            "}",
            "",
            "",
            "foo              = bar",
        ]);

        let actual = parse(config, &FILE);

        assert_fmt(&expected, &actual);
    }
}
