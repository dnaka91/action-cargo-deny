use std::fmt::{self, Display, Write};

use serde::Deserialize;

use crate::{PrintLevel, PrintValues};

#[allow(dead_code)]
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Diagnostic {
    code: Code,
    graphs: Vec<Graph>,
    labels: Vec<Label>,
    message: String,
    #[serde(default)]
    notes: Vec<String>,
    severity: Severity,
    advisory: Option<Advisory>,
}

impl Diagnostic {
    pub fn print(&self) -> PrintValues {
        let mut buf = String::new();

        for label in &self.labels {
            buf.push_str(&label.to_string());
            buf.push('\n');
        }

        if !self.graphs.is_empty() {
            buf.push_str("\nDependency graph:\n\n");
        }

        for graph in &self.graphs {
            buf.push_str(&graph.to_string());
            buf.push('\n');
        }

        if let Some(advisory) = &self.advisory {
            buf.push_str("\nAdvisory:\n\n");
            buf.push_str(&advisory.to_string());
        }

        PrintValues {
            title: Some(format!(
                "{}[{}]: {}",
                self.severity, self.code.0, self.message
            )),
            message: buf,
            level: match self.severity {
                Severity::Error => PrintLevel::Error,
                Severity::Warning => PrintLevel::Warning,
                _ => PrintLevel::Notice,
            },
        }
    }
}

#[derive(Deserialize)]
#[serde(transparent)]
#[serde(deny_unknown_fields)]
struct Code(String);

impl Code {
    #[allow(dead_code)]
    fn category(&self) -> Option<Category> {
        self.0
            .chars()
            .next()
            .map(|c| match c {
                'a' | 'A' => Some(Category::Advisories),
                'b' | 'B' => Some(Category::Bans),
                'l' | 'L' => Some(Category::Licenses),
                's' | 'S' => Some(Category::Sources),
                _ => None,
            })
            .flatten()
    }
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
enum Category {
    Advisories,
    Bans,
    Licenses,
    Sources,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Graph {
    name: String,
    version: String,
    #[serde(default)]
    repeat: bool,
    #[serde(default)]
    parents: Vec<Graph>,
    #[serde(default)]
    kind: DepKind,
}

impl Display for Graph {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_internal(f, 0)
    }
}

impl Graph {
    fn fmt_internal(&self, f: &mut fmt::Formatter<'_>, indent: usize) -> fmt::Result {
        write!(
            f,
            "{}{}{} {}{}",
            "  ".repeat(indent),
            match self.kind {
                DepKind::Normal => "",
                DepKind::Dev => "(dev) ",
                DepKind::Build => "(build) ",
            },
            self.name,
            self.version,
            if self.repeat { " (*)" } else { "" }
        )?;

        for parent in &self.parents {
            f.write_char('\n')?;
            parent.fmt_internal(f, indent + 1)?;
        }

        Ok(())
    }
}

#[derive(Clone, Copy, Deserialize)]
#[serde(rename_all = "lowercase")]
#[serde(deny_unknown_fields)]
enum DepKind {
    #[serde(rename = "")]
    Normal,
    Dev,
    Build,
}

impl Default for DepKind {
    fn default() -> Self {
        Self::Normal
    }
}

#[allow(dead_code)]
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Label {
    line: usize,
    column: usize,
    span: String,
    message: String,
}

impl Display for Label {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}\n\n{}", self.message, self.span)
    }
}

#[derive(Clone, Copy, Deserialize)]
#[serde(rename_all = "lowercase")]
#[serde(deny_unknown_fields)]
enum Severity {
    Error,
    Warning,
    Note,
    Help,
}

impl Display for Severity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Error => "error",
            Self::Warning => "warning",
            Self::Note => "note",
            Self::Help => "help",
        })
    }
}

#[allow(dead_code)]
#[derive(serde::Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Advisory {
    id: String,
    package: String,
    title: String,
    description: String,
    date: String,
    aliases: Vec<String>,
    related: Vec<String>,
    collection: Option<String>,
    categories: Vec<String>,
    keywords: Vec<String>,
    cvss: Option<String>,
    informational: Option<String>,
    url: Option<String>,
    references: Vec<String>,
    withdrawn: Option<String>,
}

impl Display for Advisory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ID: {}\nIssue: {}\n\n{}\n\n{}",
            self.id,
            self.url.as_deref().unwrap_or("<none>"),
            self.title,
            self.description,
        )
    }
}
