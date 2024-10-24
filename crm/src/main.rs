pub mod pd {
    include!(concat!(env!("OUT_DIR"),"/crm.rs"));
}
// include!(concat!(env!("OUT_DIR"), "/crm.rs"));
fn main() {
    
    let user = pd::User::default();
    
    println!("Hello, world user:{:?}",user);
}
