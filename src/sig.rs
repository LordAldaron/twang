// Twang
// Copyright © 2018-2021 Jeron Aldaron Lau.
//
// Licensed under any of:
// - Apache License, Version 2.0 (https://www.apache.org/licenses/LICENSE-2.0)
// - MIT License (https://mit-license.org/)
// - Boost Software License, Version 1.0 (https://www.boost.org/LICENSE_1_0.txt)
// At your choosing (See accompanying files LICENSE_APACHE_2_0.txt,
// LICENSE_MIT.txt and LICENSE_BOOST_1_0.txt).

//! Digital audio signal.

use fon::{chan::Ch64, mono::Mono};
use std::f64::consts::PI;

/// A signed digital audio signal that can be routed through processing
/// components.  This differs from `Mono64` in that the values are not clamped
/// between -1 and 1.
#[derive(Copy, Clone, Debug)]
pub struct Signal(f64);

impl Signal {
    /// Sine wave generator component - takes a sawtooth (`Fc`) wave.
    #[inline(always)]
    pub fn sine(self) -> Self {
        Self((self.0 * PI).cos())
    }

    /// Triangle wave generator component - takes a sawtooth (`Fc`) wave.
    #[inline(always)]
    pub fn triangle(self) -> Self {
        Self(self.0.abs() * 2.0 - 1.0)
    }

    /// Pulse wave generator component - takes a sawtooth (`Fc`) wave.
    /// - `half_duty`: ½ Duty cycle - range: 0~1 (1.0 for square wave)
    #[inline(always)]
    pub fn pulse<S: Into<Self>>(self, half_duty: S) -> Self {
        let phase_shifted = self.shift(half_duty.into().0);
        Self((self.0 - phase_shifted.0).signum())
    }

    /// Shift signal.  Takes a signal and adds an amount to it, wrapping to -1
    /// if it goes over 1, and to 1 if it goes under -1.
    #[inline(always)]
    pub fn shift<S: Into<Self>>(self, amount: S) -> Self {
        match (self.0 + amount.into().0) % 2.0 {
            x if x < -1.0 => Self(x + 2.0),
            x if x > 1.0 => Self(x - 2.0),
            x => Self(x),
        }
    }

    /// Increase (amplify) or decrease the gain of the signal.
    #[inline(always)]
    pub fn gain<S: Into<Self>>(self, volume: S) -> Self {
        Self(self.0 * volume.into().0)
    }

    /// Invert (negate) signal.
    #[inline(always)]
    pub fn invert(self) -> Self {
        Self(-self.0)
    }

    /// Absolute value of signal.
    #[inline(always)]
    pub fn abs(self) -> Self {
        Self(self.0.abs())
    }

    /// The minimum of two signals.
    #[inline(always)]
    pub fn min<S: Into<Self>>(self, limit: S) -> Self {
        Self(self.0.min(limit.into().0))
    }

    /// The maximum of two signals.
    #[inline(always)]
    pub fn max<S: Into<Self>>(self, limit: S) -> Self {
        Self(self.0.min(limit.into().0))
    }

    /// Apply noise gate by side-chaining a the cutoff signal level
    #[inline(always)]
    pub fn gate<S: Into<Self>>(self, limit: S) -> Self {
        let pass: u8 = (self.abs().0 > limit.into().0).into();
        let pass: f64 = pass.into();
        Self(self.0 * pass)
    }

    /// Raise a signal to a power.  This can be used to get the `x` root of a
    /// signal as well with `1 / x`.
    #[inline(always)]
    pub fn pow<S: Into<Self>>(self, exp: S) -> Self {
        Self(self.0.powf(exp.into().0))
    }

    /// Amplify a signal with soft clipping.
    #[inline(always)]
    pub fn clip_soft<S: Into<Self>>(self, volume: S) -> Self {
        let volume = volume.into().0;
        Self(
            (2.0 / (1.0 + (self.0 * -volume).exp()) - 1.0)
                / (2.0 / (1.0 + (-volume).exp()) - 1.0),
        )
    }

    /// Clamp a signal -1 to 1 (hard clipping)
    #[inline(always)]
    pub fn clamp(self) -> Self {
        self.min(1.0).max(-1.0)
    }

    /// Convert signal into Mono channel.
    #[inline(always)]
    pub fn to_mono(self) -> Mono<Ch64> {
        Mono::new(Ch64::new(self.0.min(1.0).max(-1.0)))
    }
}

impl From<f64> for Signal {
    fn from(signal: f64) -> Signal {
        Signal(signal)
    }
}

impl From<Signal> for f64 {
    fn from(signal: Signal) -> f64 {
        signal.0
    }
}
