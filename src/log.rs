use std::fmt::{self, Display};

use serde::Deserialize;

use crate::{PrintLevel, PrintValues};

#[derive(Deserialize)]
#[serde(rename_all = "lowercase", tag = "type", content = "fields")]
#[serde(deny_unknown_fields)]
pub enum LogEntry {
    Diagnostic(Diagnostic),
    Summary(Summary),
    Log(Log),
}

impl LogEntry {
    pub fn print(&self) -> PrintValues {
        match self {
            Self::Diagnostic(d) => d.print(),
            Self::Summary(s) => s.print(),
            Self::Log(l) => l.print(),
        }
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Diagnostic {
    code: Code,
    graphs: Vec<Graph>,
    labels: Vec<Label>,
    message: String,
    severity: Severity,
}

impl Diagnostic {
    pub fn print(&self) -> PrintValues {
        let mut buf = String::new();

        for label in &self.labels {
            buf.push_str(&label.print());
            buf.push('\n');
        }

        if !self.graphs.is_empty() {
            buf.push_str("\nDependency graph:\n");
        }

        for graph in &self.graphs {
            buf.push_str(&graph.print());
            buf.push('\n');
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

impl Graph {
    fn print(&self) -> String {
        self.print_internal(0)
    }

    fn print_internal(&self, indent: usize) -> String {
        let mut buf = format!(
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
        );

        for parent in &self.parents {
            buf.push('\n');
            buf.push_str(&parent.print_internal(indent + 1));
        }

        buf
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

#[derive(Deserialize)]
// #[serde(deny_unknown_fields)]
struct Label {
    // line: usize,
    // column: usize,
    span: String,
    message: String,
}

impl Label {
    fn print(&self) -> String {
        format!("{}\n\n{}", self.message, self.span)
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

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Summary {
    advisories: Stats,
    bans: Stats,
    licenses: Stats,
    sources: Stats,
}

impl Summary {
    pub fn print(&self) -> PrintValues {
        PrintValues {
            title: Some("Statistics".to_owned()),
            message: format!(
                "{}\n{}\n{}\n{}",
                self.advisories.print("advisories"),
                self.bans.print("bans"),
                self.licenses.print("licenses"),
                self.sources.print("sources")
            ),
            level: PrintLevel::Notice,
        }
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Stats {
    errors: u32,
    warnings: u32,
    notes: u32,
    helps: u32,
}

impl Stats {
    fn print(&self, category: &str) -> String {
        format!(
            "{:>10} {:>6}: {} errors, {} warnings, {} notes",
            category,
            if self.errors > 0 { "FAILED" } else { "ok" },
            self.errors,
            self.warnings,
            self.notes + self.helps
        )
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Log {
    timestamp: String,
    level: Level,
    message: String,
}

impl Log {
    pub fn print(&self) -> PrintValues {
        PrintValues {
            title: None,
            message: format!("{} [{:>5}] {}", self.timestamp, self.level, self.message),
            level: PrintLevel::Notice,
        }
    }
}

#[derive(Clone, Copy, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
#[serde(deny_unknown_fields)]
enum Level {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

impl Display for Level {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Error => "ERROR",
            Self::Warn => "WARN",
            Self::Info => "INFO",
            Self::Debug => "DEBUG",
            Self::Trace => "TRACE",
        })
    }
}
