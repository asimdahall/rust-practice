enum IPAddressKind {
    V4,
    V6,
}
struct Connection {
    ipAddressType: IPAddressKind,
    address: String,
}

impl Connection {
    fn print_ip_address_type(&self) {}
}
fn main() {
    let ipAddressInfo = Connection {
        ipAddressType: IPAddressKind::V4,
        address: String::from("192.168.1.1"),
    };
}
