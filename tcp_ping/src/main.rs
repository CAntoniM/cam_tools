
use std::{time::Duration,process};

use clap::Parser;
use clap_verbosity_flag::{Verbosity, InfoLevel};

use tcpping::{ping, PingError};

#[derive(Parser)]
#[command(name    = "tcp_ping")]
#[command(author  = "Callum M. <Callum.Moore@microfocus.com>")]
#[command(version = "1.0")]
#[command(about   = "a simple tester to see if you can connect to a tcp end point", long_about = None)]
struct Cli {
    
    ///Lists of host strings in the form <hostname>|<ipaddr>[:<portno>] if no port number is given a default will be used 
    hosts: Vec<String>,

    ///Defines the default port number that will be connected to
    #[arg(short,long,default_value_t=80)]
    port: u16,

    ///Defines the timeout on reciving the connection in milliseconds
    #[arg(short,long,default_value_t=1000)]
    timeout: u64,

    ///Defines the number of times a that a host will be pinged
    #[arg(short,long,default_value_t=1)]
    count: u8,

    #[clap(flatten)]
    verbose: Verbosity<InfoLevel>,
}

fn main() {
    //Parse args
    let pinger = Cli::parse(); 
    //Set up the logger (we want to dick around with this as little as possible)
    env_logger::Builder::new().filter_level(pinger.verbose.log_level_filter()).init();

    if pinger.count <= 0 || pinger.hosts.len() <= 0 {
        log::error!("Bad commandline arguments given please ensure that the count is set to atleast 1 and that one host is provded");
        process::exit(1);
    }
    
    for mut host in pinger.hosts {
        if !host.contains(':') {
            host = format!("{}:{}",host,pinger.port);
        }
        let mut count : usize = 0;
        for n in 0..pinger.count {
            match ping(&host,Duration::from_millis(pinger.timeout)) {
                Ok(duration) => {
                    log::info!("{}: Connected to {} in {}ms.",n, host,duration.as_millis());
                }
                Err(types) => {
                    count+=1;
                    match types {
                        PingError::Timeout => {
                            log::error!("{}: Failed to connect to {}: As the connection timedout.",n,host)
                        }
                        PingError::HostUnreachable => {
                            log::error!("{}: Failed to connect to {}: As Destination host unreachable.",n,host)
                        }
                        PingError::NameUnrasolveable => {
                            log::error!("{}: Failed to connect to {}: As Hostname is unreciveable.",n,host)
                        }
                        PingError::TimeError => {
                            log::warn!("{}: Connected to {}: However we failed track the time .",n,host)
                        }
                    }
                } 
            }
        }
        process::exit( count as i32);
    }
}
