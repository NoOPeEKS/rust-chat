use std::fmt;
use std::io::Write;
use std::net::TcpListener;
use std::result;

type Result<T> = result::Result<T, ()>;

const SAFE_MODE: bool = false;

struct Sensitive<T> {
    inner: T,
}

impl<T> Sensitive<T> {
    fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T: fmt::Display> fmt::Display for Sensitive<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if SAFE_MODE {
            writeln!(f, "[REDACTED]")
        } else {
            writeln!(f, "{inner}", inner = self.inner)
        }
    }
}

fn main() -> Result<()> {
    let address = "127.0.0.1:6969";

    let listener = TcpListener::bind(address).map_err(|err| {
        eprintln!(
            "ERROR: Could not bind address:{} {}",
            Sensitive::new(address),
            Sensitive::new(err)
        );
    })?;

    println!("INFO: Listening to {}", Sensitive::new(address));
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let _ = writeln!(stream, "Hello friend!").map_err(|err| {
                    eprintln!("ERROR: Could not write message to user: {err}");
                });
            }
            Err(err) => {
                eprintln!("ERROR: Could not accept connection: {err}");
            }
        }
    }

    Ok(())
}
