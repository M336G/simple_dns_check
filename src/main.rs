use reqwest::Client;
use std::time::{Duration, Instant};

struct DnsProvider {
    name: &'static str,
    url: &'static str,
    average: u64
}

#[tokio::main]
async fn main() {
    let mut dns_providers: Vec<DnsProvider> = vec![
        DnsProvider { name: "Cloudflare", url: "https://dns.cloudflare.com/dns-query", average: 0 },
        DnsProvider { name: "Google", url: "https://dns.google/dns-query", average: 0 },
        DnsProvider { name: "Quad9", url: "https://dns.quad9.net/dns-query", average: 0 },
        DnsProvider { name: "AdGuard DNS", url: "https://dns.adguard-dns.com/dns-query", average: 0 },
        DnsProvider { name: "NextDNS", url: "https://dns.nextdns.io", average: 0 }
    ];

    let client = Client::builder()
        .timeout(Duration::from_secs(2))
        .build()
        .unwrap();

    for provider in &mut dns_providers {
        let mut total: u64 = 0;

        for _ in 0..3 {
            let start = Instant::now();
            client.get(provider.url).send().await.ok();
            total += start.elapsed().as_millis() as u64;
        }

        provider.average = total / 3;
        println!("{}: ~{}ms ({})", provider.name, provider.average, provider.url);
    }

    if let Some(best) = dns_providers.iter().min_by_key(|provider| provider.average) {
        println!("----------");
        println!("Fastest DNS: {} with ~{}ms ({})", best.name, best.average, best.url);
    }
}
