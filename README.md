# PKDNS/PKARR Ecosystem 2025


Public Key Domains (PKD) provide self-sovereign and censorship-resistant domain names. They resolve records hosted on the Mainline DHT, the biggest DHT on the planet with ~10M nodes that have been servicing torrents for 15 years.


## Why Public Key Domains?

Think of it like a TOR onion address but on full internet speed.

- Domains are independent of a central authority like ICANN.
- Censorship-resistant - Nobody can block the 10M Mainline DHT nodes.
- Free - No need to purchase a domain.
- No need to dox your contact address like with regular domains.
- TLS certificates without a Certificate Authority (Rust only).
- In the future: Key delegation.

## Libraries/Tools

[pkarr](https://github.com/pubky/pkarr/) is a mainline DHT IO library that stores and reads DNS zones. Create your own public key domain DNS records.
- [Rust Crate](https://crates.io/crates/pkarr)
    - [1_publish_dns_zone](./examples/1_publish_dns_zone.rs) Publish your own Public Key Domain.
    - [2_http_request](./examples/2_http_request.rs) Make an HTTP request to a PKD.
    - [3_https_serve](./examples/3_https_serve.rs) Create a server protected by HTTPS without a Certificate Authority.
    - [4_save_and_load_key](./examples/4_save_and_load_key.rs)
- [Npm Library](https://www.npmjs.com/package/@synonymdev/pkarr)
    - [1_publish_dns_zone](./nodejs/1_publish_dns_zone.ts) Simple publish and resolve.
    - [2_save_and_load_key](./nodejs/2_save_and_load_key.ts) Save the key and load it again.

[pkdns](https://github.com/pubky/pkdns/) is a DNS server that supports Public Key Domains. Add it to your browser (DNS-over-HTTPS) or run it locally. 
- Use pkdns as your DNS server to resolve queries. Example: `nslookup 7fmjpcuuzf54hw18bsgi3zihzyh4awseeuq5tmojefaezjbd64cy 34.65.109.99`
- Resolve PKD queries in your browser with DNS-Over-HTTPS. Example: `https://pkdns.pubky.org/dns-query`
- Publish your own domains with `pkdns-cli`. See this [Medium post](https://medium.com/pubky/how-to-host-a-public-key-domain-website-v0-6-0-ubuntu-24-04-57e6f2cb6f77).
- [DynDNS Support](https://github.com/pubky/pkdns/blob/master/docs/dyn-dns.md)

[pkdns.net](https://pkdns.net) provides a convenient DNS zone viewer to verify what you published.
- [Demo Site](https://pkdns.net/?id=7fmjpcuuzf54hw18bsgi3zihzyh4awseeuq5tmojefaezjbd64cy) 7fmjpcuuzf54hw18bsgi3zihzyh4awseeuq5tmojefaezjbd64cy
- [Pubky Homeserver](https://pkdns.net/?id=8um71us3fyw6h8wbcxb5ar3rwusy1a6u49956ikzojg3gcwd1dty) 8um71us3fyw6h8wbcxb5ar3rwusy1a6u49956ikzojg3gcwd1dty
- [Severin's Pubky Account](https://pkdns.net/?id=ihgjy51fdnaingcp8rum1omfzd6p8bhm7usune41grd97dho5cwy) ihgjy51fdnaingcp8rum1omfzd6p8bhm7usune41grd97dho5cwy


[pubky-tls-proxy](https://github.com/pubky/pubky-tls-proxy) A proxy terminating raw public key TLS RFC 7250.

## Future Plans / Ideas

**pkdns-resolver** - A recursive DNS resolver library build on top of pkarr.
Pointing your domain to an other domain with a CNAME or SVCB record unlocks key delegation.

**Pubky Browser** A browser that works with Public Key Domains and Public Key TLS out of the box.

**Docs and Reliability Improvements**