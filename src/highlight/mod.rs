mod theme;

use theme::theme;

use std::sync::OnceLock;

use ratatui::{style::Style, text::Span};

use tree_sitter_go as go;
use tree_sitter_highlight::{Highlight, HighlightConfiguration, HighlightEvent, Highlighter};
use tree_sitter_rust as rust;

// Syntax colors as styled spans for each line, or plain text without a grammar
pub struct Highlighted {
    lines: Vec<Vec<Span<'static>>>,
}

impl Highlighted {
    pub fn new(language: &str, code: &str) -> Self {
        Self {
            lines: spans(language, code),
        }
    }

    // Style of the char at `line` and `column` (chars, not bytes)
    pub fn style_at(&self, line: usize, column: usize) -> Style {
        let Some(spans) = self.lines.get(line) else {
            return Style::default();
        };
        let mut end = 0;

        for span in spans {
            end += span.content.chars().count();

            if column < end {
                return span.style;
            }
        }

        Style::default()
    }
}

fn spans(language: &str, code: &str) -> Vec<Vec<Span<'static>>> {
    let Some(config) = config(language) else {
        return plain(code);
    };
    let mut highlighter = Highlighter::new();
    let Ok(events) = highlighter.highlight(config, code.as_bytes(), None, |_| None) else {
        return plain(code);
    };

    let mut lines = Vec::new();
    let mut line = Vec::new();
    let mut styles: Vec<Style> = Vec::new();

    // Events open and close highlights (a style stack); source ranges are cut at newlines
    for event in events {
        match event {
            Ok(HighlightEvent::HighlightStart(Highlight(index))) => styles.push(theme()[index].1),
            Ok(HighlightEvent::HighlightEnd) => {
                styles.pop();
            }
            Ok(HighlightEvent::Source { start, end }) => {
                let style = styles.last().copied().unwrap_or_default();

                for (i, text) in code[start..end].split('\n').enumerate() {
                    if i > 0 {
                        lines.push(std::mem::take(&mut line));
                    }
                    if !text.is_empty() {
                        line.push(Span::styled(text.to_string(), style));
                    }
                }
            }
            Err(_) => return plain(code),
        }
    }

    // The last line has no newline after it
    lines.push(line);
    lines
}

fn plain(code: &str) -> Vec<Vec<Span<'static>>> {
    code.split('\n')
        .map(|text| vec![Span::raw(text.to_string())])
        .collect()
}

fn config(language: &str) -> Option<&'static HighlightConfiguration> {
    match language {
        "rust" => Some(rust_config()),
        "go" => Some(go_config()),
        _ => None,
    }
}

fn rust_config() -> &'static HighlightConfiguration {
    static CONFIG: OnceLock<HighlightConfiguration> = OnceLock::new();

    CONFIG.get_or_init(|| {
        let mut config = HighlightConfiguration::new(
            rust::LANGUAGE.into(),
            "rust",
            rust::HIGHLIGHTS_QUERY,
            rust::INJECTIONS_QUERY,
            "",
        )
        .expect("the rust highlight queries are valid");
        let names: Vec<&str> = theme().iter().map(|(name, _)| *name).collect();

        config.configure(&names);
        config
    })
}

fn go_config() -> &'static HighlightConfiguration {
    static CONFIG: OnceLock<HighlightConfiguration> = OnceLock::new();

    CONFIG.get_or_init(|| {
        let mut config =
            HighlightConfiguration::new(go::LANGUAGE.into(), "go", go::HIGHLIGHTS_QUERY, "", "")
                .expect("the go highlight queries are valid");
        let names: Vec<&str> = theme().iter().map(|(name, _)| *name).collect();

        config.configure(&names);
        config
    })
}
