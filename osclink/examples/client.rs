use osclink::OscLink;

#[tokio::main]
async fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    let name = args.get(1).map(|s| &s[..]).unwrap_or("example");

    let client = OscLink::new(name);

    // client.test().compat().await;
    client.chromes().compat().await;
}