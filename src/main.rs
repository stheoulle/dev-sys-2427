
use clap::{Arg, ArgAction, Command};

use dev_sys_2427::{parse_ports, scan_ports};

#[tokio::main]
async fn main() {
    let matches = Command::new("scanner")
        .version("0.1.0")
        .about("Multi-threaded port scanner")
        .arg(Arg::new("target")
            .help("Target URL or IP address")
            .required(true)
            .index(1))
        .arg(Arg::new("ports")
            .short('p')
            .long("ports")
            .help("Port range (e.g. 80,1000-1010)")
            .action(ArgAction::Append)
            .num_args(1..))
        .arg(Arg::new("threads")
            .short('t')
            .long("threads")
            .help("Max number of threads")
            .default_value("100"))
        .get_matches();

    let target = matches.get_one::<String>("target").unwrap();
    let port_args = matches.get_many::<String>("ports");
    let threads = matches.get_one::<String>("threads").unwrap().parse::<usize>().unwrap_or(100);

    let ports = if let Some(port_args) = port_args {
        parse_ports(port_args.map(|s| s.as_str()).collect())
    } else {
        vec![80, 443]
    };

    println!("Scanning {} on ports {:?} with {} threads...", target, ports, threads);
    scan_ports(target, ports, threads).await;
}

