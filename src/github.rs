use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

const MESSAGE: &AsciiSet = &CONTROLS.add(b'%').add(b'\r').add(b'\n');
const PROPERTY: &AsciiSet = &MESSAGE.add(b':').add(b',');

pub fn notice(title: Option<&str>, message: &str) {
    print("notice", title, message);
}

pub fn warning(title: Option<&str>, message: &str) {
    print("warning", title, message);
}

pub fn error(title: Option<&str>, message: &str) {
    print("error", title, message);
}

fn print(level: &str, title: Option<&str>, message: &str) {
    println!(
        "::{} title={}::{}",
        level,
        utf8_percent_encode(title.unwrap_or_default(), PROPERTY),
        utf8_percent_encode(message, MESSAGE),
    );
}
