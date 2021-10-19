use ansi_term::Color;
use linefeed::{DefaultTerminal, Interface, ReadResult};

fn repl() -> Interface<DefaultTerminal> {
    let interface = Interface::new("grafana").unwrap();

    let style = Color::Red.bold();
    let text = "grafana> ";

    interface
        .set_prompt(&format!(
            "\x01{prefix}\x02{text}\x01{suffix}\x02",
            prefix = style.prefix(),
            text = text,
            suffix = style.suffix()
        ))
        .unwrap();

    interface
}

async fn main() {
    let interface = repl();

    println!("Type '\\q' to exit.");
    while let ReadResult::Input(line) = interface.read_line().unwrap() {
        if line == "\\q" {
            std::process::exit(0);
        }
        interface.add_history_unique(line.clone());
    }
}
