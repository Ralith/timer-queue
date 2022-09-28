# timer-queue

[![License: Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](LICENSE-APACHE)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE-MIT)
[![License: Zlib](https://img.shields.io/badge/License-Zlib-blue.svg)](LICENSE-ZLIB)

A pure, minimal, and scalable structure for tracking expiration of timers

```rust
let mut q = TimerQueue::new();
q.insert(42, "second");
q.insert(17, "first");
assert!(q.next_timeout().unwrap() <= 17);
assert_eq!(q.poll(16), None);
assert_eq!(q.poll(17), Some("first"));
assert_eq!(q.poll(100), Some("second"));
```

## License

Licensed under any of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
 * Zlib license ([LICENSE-ZLIB](LICENSE-ZLIB) or
   https://opensource.org/licenses/Zlib)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
triple licensed as above, without any additional terms or conditions.
