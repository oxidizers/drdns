# DrDNS [![crate][crate-image]][crate-link] [![Build Status][build-image]][build-link] [![MIT/Apache2.0 Licensed][license-image]][license-link]

[crate-image]: https://img.shields.io/crates/v/drdns.svg
[crate-link]: https://crates.io/crates/drdns
[build-image]: https://travis-ci.org/oxidizers/drdns.svg?branch=master
[build-link]: http://travis-ci.org/oxidizers/drdns
[license-image]: https://img.shields.io/badge/license-MIT/Apache2.0-blue.svg
[license-link]: https://github.com/oxidizers/drdns#license

The [djbdns] collection of Domain Name System tools, translated automatically
from C to Rust using [Corrode].

[djbdns]: https://cr.yp.to/djbdns/blurb/overview.html
[Corrode]: https://github.com/jameysharp/corrode

## Contributing

Bug reports and pull requests are welcome on GitHub at https://github.com/oxidizers/drdns

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as below, without any additional terms or
conditions.

### Obtaining original C source code

This project contains only mechanical Rust translations of the original C
source code which may be hard to understand. If you are working on improving
the code, you may want to look at the original C sources to better understand
the intended behavior of the original code.

The easiest way to view the original C source code is by reverting the commit
which removed it from this repository:

```shell
$ git revert 4c1a7d0fd
```

Please use this only to view the original code. Do not open PRs with this
commit reverted.

You can also download the original sources from:

https://cr.yp.to/djbdns/djbdns-1.05.tar.gz

The SHA-256 digest of this file should be
`3ccd826a02f3cde39be088e1fc6aed9fd57756b8f970de5dc99fcd2d92536b48`.

## License

Copyright (C) 2001 D. J. Bernstein

Licensed under either of:

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Code of Conduct

Everyone interacting in the **drdns** project’s codebases, issue trackers, chat
rooms and mailing lists is expected to follow the [code of conduct].

[code of conduct]: https://github.com/oxidizers/drdns/blob/master/CODE_OF_CONDUCT.md
