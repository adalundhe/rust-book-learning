// This is how we declar an enum.
enum IpAddrKind {
    V4,
    V6
}

// We can store values for enum types using the
// syntax below. We *do not* need to implement
// an additional struct. We can also implement different
// types and amounts of associated data for each
// variant of an enum.
enum IpAddrType {
    V4(u8, u8, u8, u8),
    V6(String)
}

// We can pass any option specified by an Enum
// to this funciton.
fn route(ip_kind: IpAddrKind) -> IpAddrKind {
    ip_kind
}


fn main() {
    // We create instances of enum values using "::" syntax;
    // let ipv4 = IpAddrKind::V4;
    // let ipv6 = IpAddrKind::V6;


    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

    let home = IpAddrType::V4(127, 0, 0, 1);
    let loopback = IpAddrType::V6(String::from("::1"));    

}
