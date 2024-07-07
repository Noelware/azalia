// 🐻‍❄️🪚 Azalia: Family of crates that implement common Rust code
// Copyright (c) 2024 Noelware, LLC. <team@noelware.org>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

//! Module to provide extensions for the [`Any`](std::any::Any) trait.
// This code was ported from charted-server: https://github.com/charted-dev/charted/blob/5f7903ca6c4771d3c6a15681751f773e99483174/src/common/as_any.rs

use std::any::Any;

#[cfg(feature = "std")]
use std::sync::Arc;

#[cfg(not(feature = "std"))]
use alloc::sync::Arc;

/// Trait that is implemented for [`Any`] implementators. It'll *safely* translate `self` into
/// a reference to `dyn Any`.
pub trait AsAny: __private::Sealed + Any {
    /// Transform `self` into `dyn Any` easily.
    fn as_any(&self) -> &dyn Any;
}

impl<T: Any> AsAny for T {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Allows downcasting from `self` into `T`, which will need to implement [`AsAny`]. This uses
/// the word "cast" to make it easier for Noel's brain.
pub trait Cast: __private::Sealed + AsAny {
    /// Downcasts `self` into an option of a reference to `T`.
    fn cast<T: AsAny>(&self) -> Option<&T> {
        self.as_any().downcast_ref()
    }
}

impl<T: ?Sized + AsAny> Cast for T {}

/// Allows upcasting `Arc<dyn T>` ~> `Arc<dyn Any>` easily. You can implement this into your traits:
///
/// ```rust
/// # use azalia::rust::AsArcAny;
/// # use std::sync::Arc;
/// #
/// pub trait Trait: AsArcAny {}
/// pub struct Test;
///
/// impl Trait for Test {}
///
/// let a: Arc<dyn Trait> = Arc::new(Test);
/// a.as_arc_any();
/// ```
pub trait AsArcAny: Any {
    /// Upcasts `Arc<dyn T>` ~> `Arc<dyn Any>`.
    fn as_arc_any(self: Arc<Self>) -> Arc<dyn Any>;
}

impl<T: Any> AsArcAny for T {
    fn as_arc_any(self: Arc<Self>) -> Arc<dyn Any> {
        self
    }
}

mod __private {
    use super::AsAny;

    pub trait Sealed {}

    impl<T: ?Sized + AsAny> Sealed for T {}
}
