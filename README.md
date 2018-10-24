# hvac

[![Build Status](https://travis-ci.org/uber-foo/hvac.svg?branch=master)](https://travis-ci.org/uber-foo/hvac)
[![Latest Version](https://img.shields.io/crates/v/hvac.svg)](https://crates.io/crates/hvac)
[![docs](https://docs.rs/hvac/badge.svg)](https://docs.rs/hvac)
![rustc 1.31+](https://img.shields.io/badge/rustc-1.31+-blue.svg)

This crate provides a state machine for an
[HVAC controller](https://en.wikipedia.org/wiki/HVAC_control_system).

HVAC control systems regulate the operation of a heating and/or air conditioning system.
Essentially, they turn on or off the heating, cooling, and air circulation as instructed by some
other system–typically a thermostat.

This crate currently supports only single-stage HVAC implementations wherein the heating and
cooling systems can be either on or off with no intermediate states of operation. Optional
constraints on the minimum run and recovery time are supported for the heat, cool, and fan
services.

This crate has no dependencies on the standard library or any other crates, making it
easily used in standard applications as well as embedded targets leveraging
[`#![no_std]`](https://doc.rust-lang.org/reference/attributes.html?highlight=no_std#crate-only-attributes).


# Example
```rust
use hvac::prelude::*;

// create a new hvac controller with the
// following constraints:
//
// heat:
// - no min run time
// - min recover of 1 minute (60 sec)
//
// cool:
// - min run time of 5 minutes (300 sec)
// - min recover of 5 minutes (300 sec)
//
// fan:
// - no min run time
// - no min recovery
let mut hvac_controller = Hvac::default()
    .with_heat(None, Some(60))
    .with_cool(Some(300), Some(300))
    .with_fan(None, None);

// enable heat as soon as permissible
let state = hvac_controller.heat();

for i in 0..60 {
    // advance state machine to `i`
    // seconds elapsed
    let state = hvac_controller.tick(i);
    // even though we have called for
    // heat, it will not be enabled
    // until we have met our 60 second
    // minimum recovery time
    assert_eq!(state.service, None);
    // and since the fan is set to auto
    // by default, it remains disabled
    assert_eq!(state.fan, false);
}

// once the state machine is at
// 60 seconds elappsed...
let state = hvac_controller.tick(60);
// we have now met our minimum recover
// time and heat is enabled
assert_eq!(state.service, Some(HvacService::Heat));
// along with the fan
assert_eq!(state.fan, true);

// we can now call for cool
let state = hvac_controller.cool();
// and heat will be immediately disabled
// since we gave it no min run time but
// cool is not immediately enabled due
// to its 300 second recovery time
assert_eq!(state.service, None);
// fan is still set to auto and has no
// minimum run time, it is also disabled
assert_eq!(state.fan, false);

// advancing to cool's minimum recovery
// time will result in cool starting
let state = hvac_controller.tick(300);
assert_eq!(state.service, Some(HvacService::Cool));
// fan also starts again
assert_eq!(state.fan, true);

// we idle the system calls
let state = hvac_controller.idle();
// which has no immediate effect because
// of cool's min run time
assert_eq!(state.service, Some(HvacService::Cool));
assert_eq!(state.fan, true);

// we disable auto mode for the fan
let state = hvac_controller.fan_auto(false);
// which still has no immediate effect
assert_eq!(state.service, Some(HvacService::Cool));
assert_eq!(state.fan, true);

// until we advance another 300 seconds
// elapsed to meet cool's min run time
let state = hvac_controller.tick(600);
// now cool has stopped but fan
// continues with auto mode disabled
assert_eq!(state.service, None);
assert_eq!(state.fan, true);

// without a minimum run time, fan will
// immediately shut down when put back
// into auto mode
let state = hvac_controller.fan_auto(true);
assert_eq!(state.fan, false);
```

## License

Licensed under either of the following, at your option:

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for
inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed
as above, without any additional terms or conditions.