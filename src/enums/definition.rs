#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String),
}

pub fn master(show: bool) {
    if show {
        let home = IpAddr::V4(String::from("127.0.0.1"));
        let loopback = IpAddr::V6(String::from("::1"));

        println!(
            "The value of home is: {:?}",
            home
        );

        println!(
            "The value of loopback is: {:?}",
            loopback
        );
    }
}
