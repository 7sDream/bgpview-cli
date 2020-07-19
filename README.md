# BGPView CLI

**Working in progress**, only `ip` sub command usable for now, and it only prints ASNs info.

Other sub commands can be added easily, but I do not use them very often, so they may not be added forever if no one asks.

## Install

```bash
$ git clone <this-repo>
$ cd bgpview
$ cargo install --path .
```

## Usage

```bash
$ bgpview ip 1.1.1.1
1.1.1.1

ASN13335: CLOUDFLARENET - Cloudflare, Inc. - US
 Prefix: 1.1.1.0/24
   Name: APNIC-LABS
   Desc: APNIC and Cloudflare DNS Resolver project
Country: AU
```

use `-h`/`--help` for help.

## License

See `UNLICENSE`.
