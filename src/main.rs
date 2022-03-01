use colored::Colorize;
use futures::future;
use futures::{stream, StreamExt};
use reqwest::header;
use reqwest::{Client, Proxy};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::net::IpAddr;
use structopt::StructOpt;

const CONCURRENCY: usize = 10;
const EXIT_ERROR_CODE: i32 = 1;

#[derive(Deserialize, Serialize, Debug)]
struct Host {
    cpes: Vec<String>,
    hostnames: Vec<String>,
    ip: String,
    ports: Vec<u16>,
    tags: Vec<String>,
    vulns: Vec<String>,
}

#[derive(Debug, StructOpt)]
#[structopt(name = "nrich", about = "Add network information to IPs")]
struct Cli {
    /// Output format (shell or json)
    #[structopt(default_value = "shell", short, long)]
    output: String,

    /// Proxy URI (HTTP, HTTPS or SOCKS)
    #[structopt(default_value = "", short, long)]
    proxy: String,

    /// File containing an IP per line. Non-IPs are ignored.
    filename: String,
}

#[tokio::main]
async fn main() {
    let args = Cli::from_args();

    let input: Box<dyn io::Read> = match args.filename == "-" {
        true => Box::new(io::stdin()),
        _ => {
            let file = File::open(args.filename);
            if let Err(e) = file {
                println!("{}: {}", "Error".red(), e);
                std::process::exit(EXIT_ERROR_CODE);
            }
            Box::new(file.unwrap())
        }
    };
    let reader = BufReader::new(input);

    // Create the HTTP client that we're using for all the requests to internetdb.shodan.io
    // Use the Brotli encoding
    let mut headers = header::HeaderMap::new();
    headers.insert("accept-encoding", header::HeaderValue::from_static("br"));

    let mut client_builder = Client::builder()
        .user_agent("nrich")
        .default_headers(headers)
        .brotli(true);

    if !args.proxy.is_empty() {
        let proxy = match Proxy::all(args.proxy) {
            Ok(proxy) => proxy,
            Err(e) => {
                println!("{}: {}", "Error".red(), e);
                std::process::exit(EXIT_ERROR_CODE);
            }
        };
        client_builder = client_builder
            .proxy(proxy)
            .danger_accept_invalid_certs(true); // We disable certificate validation to allow for self-signed certs
    }

    let client = match client_builder.build() {
        Ok(client) => client,
        Err(e) => {
            println!("{}: {}", "Error".red(), e);
            std::process::exit(EXIT_ERROR_CODE);
        }
    };

    let ip_lookups = stream::iter(reader.lines())
        // We only care about IP addresses
        .filter(|line| match line {
            Ok(line) => future::ready(line.parse::<IpAddr>().is_ok()),
            Err(_) => future::ready(false),
        })
        // Do the IP lookup in InternetDB
        .map(|line| {
            let client = &client;
            async move {
                let url = format!("https://internetdb.shodan.io/{}", line.unwrap());
                let response = client.get(url).send().await;

                // If we can't connect to the API then error out
                if let Err(e) = response {
                    println!("{}: {}", "Error".red(), e);
                    std::process::exit(EXIT_ERROR_CODE);
                }

                response.unwrap().json::<Host>().await
            }
        })
        .buffer_unordered(CONCURRENCY);

    // On Windows we need to enable the virtual terminal so colors show up
    #[cfg(windows)]
    let _ = colored::control::set_virtual_terminal(true);

    ip_lookups
        .for_each(|result| async {
            // We got some information from InternetDB
            if let Ok(host) = result {
                if args.output == "json" {
                    println!("{}", serde_json::to_string(&host).unwrap());
                } else {
                    // Terminal output should look something like this
                    //
                    // 1.1.1.1 (one.one.one.one)
                    //   Ports: 53, 443
                    //   Vulnerabilities: CVE-2014-0160
                    print!("{}", host.ip.white().bold());
                    if !host.hostnames.is_empty() {
                        print!(" ({})", host.hostnames.join(", "));
                    }
                    println!();

                    if !host.ports.is_empty() {
                        println!(
                            "  Ports: {}",
                            host.ports
                                .iter()
                                .map(|p| p.to_string().green().to_string())
                                .collect::<Vec<String>>()
                                .join(", ")
                        );
                    }
                    if !host.tags.is_empty() {
                        println!(
                            "  Tags: {}",
                            host.tags
                                .iter()
                                .map(|p| p.blue().to_string())
                                .collect::<Vec<String>>()
                                .join(", ")
                        );
                    }
                    if !host.cpes.is_empty() {
                        println!(
                            "  CPEs: {}",
                            host.cpes
                                .iter()
                                .map(|p| p.yellow().to_string())
                                .collect::<Vec<String>>()
                                .join(", ")
                        );
                    }
                    if !host.vulns.is_empty() {
                        println!(
                            "  Vulnerabilities: {}",
                            host.vulns
                                .iter()
                                .map(|p| p.red().to_string())
                                .collect::<Vec<String>>()
                                .join(", ")
                        );
                    }

                    println!();
                }
            }
        })
        .await;
}
