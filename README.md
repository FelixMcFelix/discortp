[![docs-badge][]][docs] [![crates.io version][]][crates.io link] [![license][]][license link]

# DiscoRTP
DiscoRTP is a lightweight, flexible [Real-time Transport Protocol] parsing library designed for use in non-standards compliant environments, such as [Discord].

DiscoRTP differs from other Rust RTP libraries in that packet construction *should never fail*, unless there are too few bytes.
Not all implementations treat fields as they should (*i.e.*, length), and so DiscoRTP's philosophy is that **the user knows best**.
Packet parsers are building blocks to be manually assembled, and validation mechanisms exist but are manual.

DiscoRTP was originally developed for use in [Serenity].

## Installation
Add the following to your cargo.toml:
```toml
[dependencies]
discortp = "0.2"
```

[Real-time Transport Protocol]: https://tools.ietf.org/html/rfc3550
[Discord]: discord.gg
[Serenity]: https://github.com/serenity-rs/serenity

[docs-badge]: https://img.shields.io/badge/docs-online-4d76ae.svg?style=flat-square
[docs]: https://docs.rs/discortp

[crates.io link]: https://crates.io/crates/discortp
[crates.io version]: https://img.shields.io/crates/v/discortp.svg?style=flat-square

[license]: https://img.shields.io/crates/l/discortp?style=flat-square
[license link]: https://opensource.org/licenses/ISC