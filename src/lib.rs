//! This crate provides a state machine for an
//! [HVAC controller](https://en.wikipedia.org/wiki/HVAC_control_system).
//!
//! HVAC control systems regulate the operation of a heating and/or air conditioning system.
//! Essentially, they turn on or off the heating, cooling, and air circulation as instructed by some
//! other system–typically a thermostat.
//!
//! This crate currently supports only single-stage HVAC implementations wherein the heating and
//! cooling systems can be either on or off with no intermediate states of operation. Optional
//! constraints on the minimum run and recovery time are supported for the heat, cool, and fan
//! services.
//!
//! This crate has no dependencies on the standard library or any other crates, making it
//! easily used in standard applications as well as embedded targets leveraging
//! [`#![no_std]`](https://doc.rust-lang.org/reference/attributes.html?highlight=no_std#crate-only-attributes).

//!
//! # Example
//! ```
//! use hvac::prelude::*;
//!
//! // create a new hvac controller with the
//! // following constraints:
//! //
//! // heat:
//! // - no min run time
//! // - min recover of 1 minute (60 sec)
//! //
//! // cool:
//! // - min run time of 5 minutes (300 sec)
//! // - min recover of 5 minutes (300 sec)
//! //
//! // fan:
//! // - no min run time
//! // - no min recovery
//! let mut hvac_controller = Hvac::default()
//!     .with_heat(None, Some(60))
//!     .with_cool(Some(300), Some(300))
//!     .with_fan(None, None);
//!
//! // enable heat as soon as permissible
//! let state = hvac_controller.heat();

//! for i in 0..60 {
//!     // advance state machine to `i`
//!     // seconds elapsed
//!     let state = hvac_controller.tick(i);
//!     // even though we have called for
//!     // heat, it will not be enabled
//!     // until we have met our 60 second
//!     // minimum recovery time
//!     assert_eq!(state.service, None);
//!     // and since the fan is set to auto
//!     // by default, it remains disabled
//!     assert_eq!(state.fan, false);
//! }
//!
//! // once the state machine is at
//! // 60 seconds elappsed...
//! let state = hvac_controller.tick(60);
//! // we have now met our minimum recover
//! // time and heat is enabled
//! assert_eq!(state.service, Some(HvacService::Heat));
//! // along with the fan
//! assert_eq!(state.fan, true);
//!
//! // we can now call for cool
//! let state = hvac_controller.cool();
//! // and heat will be immediately disabled
//! // since we gave it no min run time but
//! // cool is not immediately enabled due
//! // to its 300 second recovery time
//! assert_eq!(state.service, None);
//! // fan is still set to auto and has no
//! // minimum run time, it is also disabled
//! assert_eq!(state.fan, false);
//!
//! // advancing to cool's minimum recovery
//! // time will result in cool starting
//! let state = hvac_controller.tick(300);
//! assert_eq!(state.service, Some(HvacService::Cool));
//! // fan also starts again
//! assert_eq!(state.fan, true);
//!
//! // we idle the system calls
//! let state = hvac_controller.idle();
//! // which has no immediate effect because
//! // of cool's min run time
//! assert_eq!(state.service, Some(HvacService::Cool));
//! assert_eq!(state.fan, true);
//!
//! // we disable auto mode for the fan
//! let state = hvac_controller.fan_auto(false);
//! // which still has no immediate effect
//! assert_eq!(state.service, Some(HvacService::Cool));
//! assert_eq!(state.fan, true);
//!
//! // until we advance another 300 seconds
//! // elapsed to meet cool's min run time
//! let state = hvac_controller.tick(600);
//! // now cool has stopped but fan
//! // continues with auto mode disabled
//! assert_eq!(state.service, None);
//! assert_eq!(state.fan, true);
//!
//! // without a minimum run time, fan will
//! // immediately shut down when put back
//! // into auto mode
//! let state = hvac_controller.fan_auto(true);
//! assert_eq!(state.fan, false);
//! ```
#![no_std]
#![deny(warnings)]
#![deny(bad_style)]
#![deny(future_incompatible)]
#![deny(nonstandard_style)]
#![deny(unused)]
#![deny(rust_2018_compatibility)]
#![deny(rust_2018_idioms)]
#![deny(macro_use_extern_crate)]
#![deny(missing_copy_implementations)]
#![deny(missing_debug_implementations)]
#![deny(missing_docs)]
#![deny(trivial_casts)]
#![deny(trivial_numeric_casts)]
#![deny(unreachable_pub)]
#![deny(unsafe_code)]
#![deny(unstable_features)]
#![deny(unused_import_braces)]
#![deny(unused_lifetimes)]
#![deny(unused_qualifications)]
#![deny(unused_results)]
#![deny(variant_size_differences)]
#![cfg_attr(feature = "cargo-clippy", deny(clippy::all))]

/// hvac services
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum HvacService {
    /// heat
    Heat,
    /// cool
    Cool,
}

/// hvac state
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct HvacState {
    /// active service, if any
    pub service: Option<HvacService>,
    /// if fan is active
    pub fan: bool,
}

/// hvac state machine
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Hvac {
    active_service: Option<HvacService>,
    fan_active: bool,
    last_update: Option<u32>,
    heat_calling: bool,
    heat_min_run_seconds: Option<u32>,
    heat_min_recover_seconds: Option<u32>,
    heat_wait_seconds: Option<u32>,
    heat_last_start_seconds: Option<u32>,
    heat_last_stop_seconds: Option<u32>,
    cool_calling: bool,
    cool_min_run_seconds: Option<u32>,
    cool_min_recover_seconds: Option<u32>,
    cool_wait_seconds: Option<u32>,
    cool_last_start_seconds: Option<u32>,
    cool_last_stop_seconds: Option<u32>,
    fan_auto: bool,
    fan_min_run_seconds: Option<u32>,
    fan_min_recover_seconds: Option<u32>,
    fan_wait_seconds: Option<u32>,
    fan_last_start_seconds: Option<u32>,
    fan_last_stop_seconds: Option<u32>,
}

impl Default for Hvac {
    fn default() -> Self {
        Self {
            active_service: None,
            fan_active: false,
            last_update: None,
            heat_calling: false,
            heat_min_run_seconds: Some(60),
            heat_min_recover_seconds: Some(60),
            heat_wait_seconds: Some(60),
            heat_last_start_seconds: None,
            heat_last_stop_seconds: None,
            cool_calling: false,
            cool_min_run_seconds: Some(300),
            cool_min_recover_seconds: Some(300),
            cool_wait_seconds: Some(60),
            cool_last_start_seconds: None,
            cool_last_stop_seconds: None,
            fan_auto: true,
            fan_min_run_seconds: Some(60),
            fan_min_recover_seconds: Some(60),
            fan_wait_seconds: Some(60),
            fan_last_start_seconds: None,
            fan_last_stop_seconds: None,
        }
    }
}

fn wait_seconds(
    last_update: Option<u32>,
    min_seconds: Option<u32>,
    last_change: Option<u32>,
) -> Option<u32> {
    if let Some(last_update) = last_update {
        if let Some(min_seconds) = min_seconds {
            let delta = last_update - last_change.unwrap_or(0);
            if delta < min_seconds {
                Some(min_seconds - delta)
            } else {
                None
            }
        } else {
            None
        }
    } else {
        min_seconds
    }
}

impl Hvac {
    /// use custom heat run and recover time constraints
    pub fn with_heat(
        mut self,
        min_run_seconds: Option<u32>,
        min_recover_seconds: Option<u32>,
    ) -> Self {
        self.heat_min_run_seconds = min_run_seconds;
        self.heat_min_recover_seconds = min_recover_seconds;
        self
    }

    /// use custom cool run and recover time constraints
    pub fn with_cool(
        mut self,
        min_run_seconds: Option<u32>,
        min_recover_seconds: Option<u32>,
    ) -> Self {
        self.cool_min_run_seconds = min_run_seconds;
        self.cool_min_recover_seconds = min_recover_seconds;
        self
    }

    /// use custom fan run and recover time constraints
    pub fn with_fan(
        mut self,
        min_run_seconds: Option<u32>,
        min_recover_seconds: Option<u32>,
    ) -> Self {
        self.fan_min_run_seconds = min_run_seconds;
        self.fan_min_recover_seconds = min_recover_seconds;
        self
    }

    fn state(&self) -> HvacState {
        HvacState {
            service: self.active_service,
            fan: self.fan_active,
        }
    }

    fn compute(&mut self) -> HvacState {
        self.heat_wait_seconds = if self.active_service == Some(HvacService::Heat) {
            wait_seconds(
                self.last_update,
                self.heat_min_run_seconds,
                self.heat_last_start_seconds,
            )
        } else {
            wait_seconds(
                self.last_update,
                self.heat_min_recover_seconds,
                self.heat_last_stop_seconds,
            )
        };

        self.cool_wait_seconds = if self.active_service == Some(HvacService::Cool) {
            wait_seconds(
                self.last_update,
                self.cool_min_run_seconds,
                self.cool_last_start_seconds,
            )
        } else {
            wait_seconds(
                self.last_update,
                self.cool_min_recover_seconds,
                self.cool_last_stop_seconds,
            )
        };

        self.fan_wait_seconds = if self.fan_active {
            wait_seconds(
                self.last_update,
                self.fan_min_run_seconds,
                self.fan_last_start_seconds,
            )
        } else {
            wait_seconds(
                self.last_update,
                self.fan_min_recover_seconds,
                self.fan_last_stop_seconds,
            )
        };

        if let Some(active_service) = self.active_service {
            match active_service {
                HvacService::Heat => {
                    if !self.heat_calling && self.heat_wait_seconds.is_none() {
                        self.heat_last_stop_seconds = self.last_update;
                        self.active_service = None;
                        if self.cool_calling && self.cool_wait_seconds.is_none() {
                            self.cool_last_start_seconds = self.last_update;
                            self.active_service = Some(HvacService::Cool);
                        } else if self.fan_auto && self.fan_wait_seconds.is_none() {
                            self.fan_last_stop_seconds = self.last_update;
                            self.fan_active = false;
                        };
                    };
                }
                HvacService::Cool => {
                    if !self.cool_calling && self.cool_wait_seconds.is_none() {
                        self.cool_last_stop_seconds = self.last_update;
                        self.active_service = None;
                        if self.heat_calling && self.heat_wait_seconds.is_none() {
                            self.heat_last_start_seconds = self.last_update;
                            self.active_service = Some(HvacService::Heat);
                        } else if self.fan_auto && self.fan_wait_seconds.is_none() {
                            self.fan_last_stop_seconds = self.last_update;
                            self.fan_active = false;
                        };
                    };
                }
            };
        } else if self.heat_calling && self.heat_wait_seconds.is_none() {
            if !self.fan_active && self.fan_wait_seconds.is_none() {
                self.fan_last_start_seconds = self.last_update;
                self.fan_active = true;
            };
            if self.fan_active {
                self.heat_last_start_seconds = self.last_update;
                self.active_service = Some(HvacService::Heat);
            };
        } else if self.cool_calling && self.cool_wait_seconds.is_none() {
            if !self.fan_active && self.fan_wait_seconds.is_none() {
                self.fan_last_start_seconds = self.last_update;
                self.fan_active = true;
            };
            if self.fan_active {
                self.cool_last_start_seconds = self.last_update;
                self.active_service = Some(HvacService::Cool);
            };
        };

        if self.fan_active && self.fan_auto {
            if self.active_service.is_none() && self.fan_wait_seconds.is_none() {
                self.fan_active = false;
            };
        } else if !self.fan_auto && self.fan_wait_seconds.is_none() {
            self.fan_active = true;
        };

        self.state()
    }

    /// update the state machine with new seconds elappsed value
    pub fn tick(&mut self, current_seconds: u32) -> HvacState {
        self.last_update = Some(current_seconds);
        self.compute()
    }

    /// update state machine with a call for heat, disabling call for cool in the process
    pub fn heat(&mut self) -> HvacState {
        self.heat_calling = true;
        self.cool_calling = false;
        self.compute()
    }

    /// update state machine with call for cool, disabling call for heat in the process
    pub fn cool(&mut self) -> HvacState {
        self.heat_calling = false;
        self.cool_calling = true;
        self.compute()
    }

    /// update state machine setting fan to auto (on with service) or manual (on always)
    pub fn fan_auto(&mut self, fan_auto: bool) -> HvacState {
        self.fan_auto = fan_auto;
        self.compute()
    }

    /// update state machine disabling any calls for service
    pub fn idle(&mut self) -> HvacState {
        self.heat_calling = false;
        self.cool_calling = false;
        self.compute()
    }
}

/// convienence module that re-exports the typical api
pub mod prelude {
    #[doc(no_inline)]
    pub use crate::{Hvac, HvacService, HvacState};
}
