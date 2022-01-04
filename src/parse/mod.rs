use serde::Deserialize;

pub use self::{diagnostic::Diagnostic, log::Log, summary::Summary};

mod diagnostic;
mod log;
mod summary;

#[derive(Deserialize)]
#[serde(rename_all = "lowercase", tag = "type", content = "fields")]
#[serde(deny_unknown_fields)]
pub enum Event {
    Diagnostic(Box<Diagnostic>),
    Summary(Summary),
    Log(Log),
}
