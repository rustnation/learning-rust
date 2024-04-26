use crate::print_title;

pub fn master(show: bool) {
    if show {
        print_title("Network");

        let ip_address = "192.168.1.255";
        if is_valid_ip(ip_address) {
            println!("ip_address: {} is a valid IP address", ip_address);
        }
    }
}

fn is_valid_ip(ip_address: &str) -> bool {
    let octets: Vec<&str> = ip_address.split(".").collect();

    if octets.len() != 4 {
        return false
    }

    for octect in octets {
        match octect.parse::<u8>() {
            Ok(num) => {
                println!("num: {}", num);
            },
            Err(_) => {
                return false;
            }
        }
    }

    true
}