// 🐻‍❄️🪚 azalia: Noelware's Rust commons library.
// Copyright (c) 2024-2025 Noelware, LLC. <team@noelware.org>
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

//! Defines common types when dealing with the [**any** module](crate::libstd::any).

use crate::libstd::{self, any};

/// Allows converting a `&self` into a `&dyn Any` or `&mut dyn Any`.
pub trait AsAny: private::Sealed + any::Any {
    /// Transform `self` into `&dyn Any`.
    fn as_any(&self) -> &dyn any::Any;

    /// Transform a mutable `self` into `&mut dyn Any`.
    fn as_mut_any(&mut self) -> &mut dyn any::Any;
}

impl<T: any::Any> AsAny for T {
    fn as_any(&self) -> &dyn any::Any {
        self
    }

    fn as_mut_any(&mut self) -> &mut dyn any::Any {
        self
    }
}

/// Allows upcasting from a <code>[`Arc`](libstd::Arc)\<T\></code> -> <code>[`Arc`](libstd::Arc)\<dyn [`Any`](any::Any)\></code>.
///
/// **Note**: This requires either the `std` or `alloc` feature to be enabled.
///
/// ## Example
/// ```
/// use azalia::rust::AsArcAny;
/// use std::{any::Any, sync::Arc};
///
/// pub trait A: AsArcAny {}
///
/// pub struct B;
/// impl A for B {}
///
/// let b: Arc<dyn A> = Arc::new(B);
/// let _: Arc<dyn Any> = b.as_arc_any();
/// ```
#[cfg(any(feature = "std", feature = "alloc"))]
#[cfg_attr(any(noeldoc, docsrs), doc(cfg(any(feature = "std", feature = "alloc"))))]
pub trait AsArcAny: any::Any {
    /// Upcasts <code>[`Arc`](libstd::Arc)\<Self\></code> -> <code>[`Arc`](libstd::Arc)\<dyn [`Any`](any::Any)\></code> easily.
    fn as_arc_any(self: libstd::Arc<Self>) -> libstd::Arc<dyn any::Any>;
}

#[cfg(any(feature = "std", feature = "alloc"))]
#[cfg_attr(any(noeldoc, docsrs), doc(cfg(any(feature = "std", feature = "alloc"))))]
impl<T: any::Any> AsArcAny for T {
    fn as_arc_any(self: libstd::Arc<Self>) -> libstd::Arc<dyn any::Any> {
        self
    }
}

/// Analogous of [`AsArcAny`] but uses the [`Rc`](libstd::Rc) smart pointer instead.
///
/// **Note**: This requires either the `std` or `alloc` feature to be enabled.
///
/// ## Example
/// ```
/// use azalia::rust::AsRcAny;
/// use std::{any::Any, rc::Rc};
///
/// pub trait A: AsRcAny {}
///
/// pub struct B;
/// impl A for B {}
///
/// let b: Rc<dyn A> = Rc::new(B);
/// let _: Rc<dyn Any> = b.as_rc_any();
/// ```
#[cfg(any(feature = "std", feature = "alloc"))]
#[cfg_attr(any(noeldoc, docsrs), doc(cfg(any(feature = "std", feature = "alloc"))))]
pub trait AsRcAny: any::Any {
    /// Upcasts <code>[`Rc`](libstd::Rc)\<Self\></code> -> <code>[`Rc`](libstd::Rc)\<dyn [`Any`](any::Any)\></code> easily.
    fn as_rc_any(self: libstd::Rc<Self>) -> libstd::Rc<dyn any::Any>;
}

#[cfg(any(feature = "std", feature = "alloc"))]
#[cfg_attr(any(noeldoc, docsrs), doc(cfg(any(feature = "std", feature = "alloc"))))]
impl<T: any::Any> AsRcAny for T {
    fn as_rc_any(self: libstd::Rc<Self>) -> libstd::Rc<dyn any::Any> {
        self
    }
}

/// Allows upcasting from a <code>[`Box`]\<T\></code> -> <code>[`Box`]\<dyn [`Any`](any::Any)\></code>.
///
/// **Note**: This requires either the `std` or `alloc` feature to be enabled.
///
/// ## Example
/// ```
/// use azalia::rust::AsBoxAny;
/// use std::any::Any;
///
/// pub trait A: AsBoxAny {}
///
/// pub struct B;
/// impl A for B {}
///
/// let b: Box<dyn A> = Box::new(B);
/// let _: Box<dyn Any> = b.as_box_any();
/// ```
#[cfg(any(feature = "std", feature = "alloc"))]
#[cfg_attr(any(noeldoc, docsrs), doc(cfg(any(feature = "std", feature = "alloc"))))]
pub trait AsBoxAny: any::Any {
    /// Upcasts <code>[`Box`]\<Self\></code> -> <code>[`Box`]\<dyn [`Any`](any::Any)\></code> easily.
    fn as_box_any(self: libstd::Box<Self>) -> libstd::Box<dyn any::Any>;
}

#[cfg(any(feature = "std", feature = "alloc"))]
#[cfg_attr(any(noeldoc, docsrs), doc(cfg(any(feature = "std", feature = "alloc"))))]
impl<T: any::Any> AsBoxAny for T {
    fn as_box_any(self: libstd::Box<Self>) -> libstd::Box<dyn any::Any> {
        self
    }
}

mod private {
    pub trait Sealed {}

    impl<T: ?Sized + super::AsAny> Sealed for T {}
}
