fn main() {
    println!("Hello, world!");
    use time::OffsetDateTime;

let now = OffsetDateTime::now_utc();
// let local = OffsetDateTime::now_local();

println!("{now}");
}
