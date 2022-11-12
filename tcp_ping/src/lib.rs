
    use std::{time::SystemTime, time::Duration,net::{TcpStream, ToSocketAddrs, SocketAddr}};
    /// This is the enum that is used as an as part of this package this is technically not a 
    /// rust error as it does not have any implmenetion but rusts ability to hold data in an enum 
    /// value is really nice so we can use that here
    pub enum PingError {
        Timeout,
        HostUnreachable,
        NameUnrasolveable,
        TimeError,
    }

    /// This function will open a sync ack message to the targeted tcp socket
    /// 
    pub fn ping (hostname: &String,timeout: Duration) -> Result<Duration,PingError>{
            let addrs: Vec<SocketAddr> ;
            match hostname.to_socket_addrs() {
                Ok(itr) => {
                    addrs = itr.collect();
                }
                Err(_) => {
                    return Err(PingError::NameUnrasolveable);
                }
            };
            if addrs.len() <= 0 {
                return Err(PingError::NameUnrasolveable);
            }
            let addr = addrs[0];
            let start = SystemTime::now();
            let time_taken: Duration ;
            match TcpStream::connect_timeout(&addr,timeout) {
               Ok(_)  => {}
               Err(_) => { 
                return Err(PingError::HostUnreachable);
               }
            }
            match SystemTime::now().duration_since(start) {
                Ok (dur) => {
                    time_taken = dur;
                    return Ok(time_taken);
                }
                Err (_) => {
                    return Err(PingError::TimeError);
                }
            }
    }