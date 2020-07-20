# BGPView CLI

**Working in progress**, only `ip` sub command usable for now.

Other sub commands can be added easily, but I do not use them personally, so they may not be added forever if no one asks.

You can refer to the [document of bgpview.io API][doc-of-bgpview.io] if you want to contribute or implement other commands.

## Install

```bash
$ cargo install bgpview-cli
```

## Usage

```bash
$ bgpview-cli ip 1.1.1.1
     IP: 1.1.1.1
Country: AU
   City: Unknown

   IANA: APNIC
AssigSt: allocated
AssigAt: Unknown
  Whois: whois.apnic.net

    RIR: APNIC - AU
 Prefix: 1.1.1.0/24
AllocSt: assigned
AllocAt: 2011-08-11 00:00:00

    ASN: 13335 - CLOUDFLARENET - Cloudflare, Inc. - US
 Prefix: 1.1.1.0/24
   Name: APNIC-LABS
   Desc: APNIC and Cloudflare DNS Resolver project
Country: AU
```

Use `-h`/`--help` for help.

You can set [alias][wikipedia:alias] if you use some command very often, bash for example:

```bash
alias bvip="bgpview-cli ip"
bvip 1.1.1.1
```

## License

See `UNLICENSE`.

[doc-of-bgpview.io]: https://bgpview.docs.apiary.io/
[wikipedia:alias]: https://en.wikipedia.org/wiki/Alias_(command)