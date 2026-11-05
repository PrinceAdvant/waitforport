use std::env;
use std::io;
use std::net::{TcpStream, ToSocketAddrs};
use std::process;
use std::thread;
use std::time::{Duration, Instant};

fn usage() -> ! {
    eprintln!("Usage: waitforport <host:port> [--timeout-secs <n>] [--interval-ms <n>]");
    process::exit(2);
}

fn parse_args() -> (String, Duration, Duration) {
    let mut args = env::args().skip(1);
    let target = match args.next() {
        Some(v) if !v.starts_with('-') => v,
        _ => usage(),
    };
    let mut timeout = Duration::from_secs(60);
    let mut interval = Duration::from_millis(500);
    while let Some(flag) = args.next() {
        match flag.as_str() {
            "--timeout-secs" => {
                let n: u64 = args.next().and_then(|s| s.parse().ok()).unwrap_or_else(|| usage());
                timeout = Duration::from_secs(n);
            }
            "--interval-ms" => {
                let n: u64 = args.next().and_then(|s| s.parse().ok()).unwrap_or_else(|| usage());
                interval = Duration::from_millis(n);
            }
            _ => usage(),
        }
    }
    (target, timeout, interval)
}

fn try_connect(target: &str) -> io::Result<()> {
    let mut last = None;
    for addr in target.to_socket_addrs()? {
        match TcpStream::connect_timeout(&addr, Duration::from_secs(2)) {
            Ok(_) => return Ok(()),
            Err(e) => last = Some(e),
        }
    }
    Err(last.unwrap_or_else(|| io::Error::new(io::ErrorKind::Other, "no addresses")))
}

fn main() {
    let (target, timeout, interval) = parse_args();
    let start = Instant::now();
    loop {
        match try_connect(&target) {
            Ok(()) => {
                println!("ready: {target}");
                process::exit(0);
            }
            Err(e) => {
                if start.elapsed() >= timeout {
                    eprintln!("timeout waiting for {target}: {e}");
                    process::exit(1);
                }
                thread::sleep(interval);
            }
        }
    }
}
