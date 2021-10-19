use ansi_term::{Color, Style};
use linefeed::{DefaultTerminal, Interface, ReadResult};

fn pre(text: &str, style: Style) -> String {
    format!(
        "\x01{prefix}\x02{text}\x01{suffix}\x02",
        prefix = style.prefix(),
        text = text,
        suffix = style.suffix()
    )
}

fn repl() -> Interface<DefaultTerminal> {
    let interface = Interface::new("grafana").unwrap();

    let style = Color::Red.bold();
    let text = "input> ";

    interface.set_prompt(&pre(text, style)).unwrap();

    interface
}

#[tokio::main]
async fn main() {
    let interface = repl();

    println!("Type '\\q' to exit.");
    while let ReadResult::Input(line) = interface.read_line().unwrap() {
        if line == "\\q" {
            std::process::exit(0);
        }
        interface.add_history_unique(line.clone());

        let mut ts = ast::time_series::Query(vec![]);
        let mut query_type = "none";

        if let Ok(s) = spl::grammar::QueryParser::new().parse(line.as_str()) {
            query_type = "spl";
            ts = s.into();
        }

        if let Ok(p) = promql::grammar::QueryParser::new().parse(line.as_str())
        {
            query_type = "promql";
            ts = p.into();
        }

        let style = Color::White.bold();
        let mut text = "parsed:";
        println!("{} {}", &pre(text, style), query_type);
        text = "promql:";
        println!(
            "{} {}",
            &pre(text, style),
            promql::ast::Query::from(ts.clone())
        );
        text = "spl:";
        println!("{} {}", &pre(text, style), spl::ast::Query::from(ts))
    }
}
