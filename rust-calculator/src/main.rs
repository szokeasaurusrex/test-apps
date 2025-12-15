use rust_calculator::{parser, tui};

fn main() {
    let _guard = sentry::init((
        "https://02d146c6dd150714d1f7ec254c2f0016@o447951.ingest.us.sentry.io/4506139505131520",
        sentry::ClientOptions {
            release: sentry::release_name!(),
            // Capture user IPs and potentially sensitive headers when using HTTP server integrations
            // see https://docs.sentry.io/platforms/rust/data-management/data-collected for more info
            send_default_pii: true,
            ..Default::default()
        },
    ));

    loop {
        let input = tui::prompt();
        match parser::parse_input(&input) {
            Ok(tokens) => match parser::evaluate(&tokens) {
                Ok(result) => println!("{result}"),
                Err(err) => eprintln!("Error: {err}"),
            },
            Err(err) => eprintln!("Error: {err}"),
        }
    }
}
