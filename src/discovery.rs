use mdns_sd::{ServiceDaemon, ServiceEvent};
use std::collections::HashSet;
use std::net::Ipv4Addr;

// TODO: async function
// TODO: after multicast discovery try some basic command to see if given device is Yamaha

pub fn discover(name: &str) -> Box<HashSet<Ipv4Addr>>
{
    let mdns = ServiceDaemon::new().expect("Failed to create daemon");

    let service_type = "_http._tcp.local.";
    let receiver = mdns.browse(service_type).expect("Failed to browse");
    let expected_full_name = format!("{}.{}", name, service_type);

    println!("Testing");

    let mut search_attempts = 0u32;

    while let Ok(event) = receiver.recv() {
        match event {
            ServiceEvent::SearchStarted(_) => {
                search_attempts += 1;
                if search_attempts >= 6 {
                    return Box::new(HashSet::new());
                }
            }
            ServiceEvent::ServiceResolved(info) => {
                if info.get_fullname() == expected_full_name {
                    return Box::new(info.get_addresses().clone());
                }
            }
            _ => {}
        }
    }

    Box::new(HashSet::new())
}

#[cfg(test)]
mod tests {
    /*
    use super::*;
    #[test]
    fn discovery_works() {
        let syp_addr = discover("Sypialnia");
        println!("Sypialnia address: {:?}", syp_addr);
    }
    */
}
