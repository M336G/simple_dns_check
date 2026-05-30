# simple_dns_check
**Simple tool to know which DNS provider is the best for you**

## Usage
```bash
$ simple_dns_check
Cloudflare: ~66ms (https://dns.cloudflare.com/dns-query)
Google: ~36ms (https://dns.google/dns-query)
Quad9: ~25ms (https://dns.quad9.net/dns-query)
AdGuard DNS: ~43ms (https://dns.adguard-dns.com/dns-query)
NextDNS: ~39ms (https://dns.nextdns.io)
----------
Fastest DNS: Quad9 with ~25ms (https://dns.quad9.net/dns-query)
```

## Installing
### Downloading from releases
- [Windows](https://github.com/M336G/simple_dns_check/releases/latest/download/simple_dns_check-windows.zip)
- [macOS](https://github.com/M336G/simple_dns_check/releases/latest/download/simple_dns_check-macos.zip)
- [Linux](https://github.com/M336G/simple_dns_check/releases/latest/download/simple_dns_check-linux.zip)

To use `simple_dns_check` from anywhere, add it to your PATH:
- **On Windows**: see [this guide](https://learn.microsoft.com/en-us/previous-versions/office/developer/sharepoint-2010/ee537574(v=office.14)#to-add-a-path-to-the-path-environment-variable)
- **On Unix**: move the binary to `/usr/local/bin/`

### Building from source
**Prerequisites:**
- [Rust](https://rust-lang.org/) installed

Run the following from your terminal (this may take some time):
```bash
cargo install --git https://github.com/M336G/simple_dns_check
```

## License
This project is licensed under the [MIT License](https://github.com/M336G/simple_dns_check/blob/main/LICENSE).