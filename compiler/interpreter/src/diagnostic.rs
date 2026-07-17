//! Human-friendly runtime diagnostics.

use lexer::{source::SourceFile, span::Span};

pub struct Diagnostic<'a> {
    pub code: &'static str,

    pub title: String,

    pub message: String,

    pub note: Option<String>,

    pub help: Option<String>,

    pub example: Option<String>,

    pub span: Span,

    pub source: &'a SourceFile,
}

impl<'a> Diagnostic<'a> {
    pub fn render(&self) -> String {
        let (line, column, line_text) = self.source.location(self.span.start);

        let pointer = format!(
            "{}{}",
            " ".repeat(column.saturating_sub(1)),
            "^".repeat(self.span.length().max(1))
        );

        let mut output = String::new();

        output.push_str(&format!("error[{}]: {}\n\n", self.code, self.title));

        output.push_str(&format!(
            " --> {}:{}:{}\n\n",
            self.source.path, line, column
        ));

        output.push_str(&format!("{:>4} │ {}\n", line, line_text));

        output.push_str(&format!("     │ {}\n\n", pointer));

        output.push_str(&self.message);

        if let Some(note) = &self.note {
            output.push_str("\n\nnote:\n");
            output.push_str(note);
        }

        if let Some(help) = &self.help {
            output.push_str("\n\nhelp:\n");
            output.push_str(help);
        }

        if let Some(example) = &self.example {
            output.push_str("\n\nexample:\n\n");
            output.push_str(example);
        }

        output
    }
}
