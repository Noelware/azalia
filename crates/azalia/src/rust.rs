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

/// A declarative macro to implement for `dyn <Type>`:
///
/// - [`<dyn Type>::is`][`\<dyn Type\>::is`]: Checks if `self` is of type `T`.
/// - [`<dyn Type>::downcast`][`\<dyn Type\>::downcast`]: Downcasts `self` to <code>[`Option`]\<[`&`]T\></code>.
/// - [`<dyn Type>::downcast_mut`][`\<dyn Type\>::downcast_mut`]: Downcasts `self` to <code>[`Option`]\<[`&mut`] T\></code>.
/// - [`<dyn Type>::downcast_unchecked`][`\<dyn Type\>::downcast_unchecked`]: Downcasts `self` to <code>[`&`]T</code> unsafely.
/// - [`<dyn Type>::downcast_unchecked_mut`][`\<dyn Type\>::downcast_unchecked_mut`]: Downcasts `self` to <code>[`&mut`] T</code> unsafely.
///
/// [`\<dyn Type\>::downcast_unchecked_mut`]: https://doc.rust-lang.org/std/any/trait.Any.html#method.downcast_mut_unchecked
/// [`\<dyn Type\>::downcast_unchecked`]: https://doc.rust-lang.org/std/any/trait.Any.html#method.downcast_ref_unchecked
/// [`\<dyn Type\>::downcast_mut`]: https://doc.rust-lang.org/std/any/trait.Any.html#method.downcast_mut
/// [`\<dyn Type\>::downcast`]: https://doc.rust-lang.org/std/any/trait.Any.html#method.downcast_ref
/// [`\<dyn Type\>::is`]: https://doc.rust-lang.org/std/any/trait.Any.html#method.is
#[macro_export]
#[cfg(not(upcast_for_dyn_any))]
macro_rules! impl_dyn_any {
    ($Ty:ident) => {
        const _: () = {
            impl dyn $Ty + 'static {
                /// Compare if `self` is of type `T`, similar to [`<dyn Any>::is`][`\<dyn Any\>::is`].
                ///
                /// [`\<dyn Any\>::is`]: https://doc.rust-lang.org/std/any/trait.Any.html#method.is
                pub fn is<T: ::core::any::Any>(&self) -> bool {
                    ::core::any::Any::type_id(self) == ::core::any::TypeId::of::<T>()
                }

                /// Downcasts `self` to `T` by comparing the type. Otherwise,
                /// returns [`None`].
                ///
                /// This method is similar to [`<dyn Any>::downcast_ref`][`\<dyn Any\>::downcast_ref`].
                ///
                /// [`\<dyn Any\>::downcast_ref`]: https://doc.rust-lang.org/std/any/trait.Any.html#method.downcast_ref
                pub fn downcast<T: ::core::any::Any>(&self) -> ::core::option::Option<&T> {
                    if self.is::<T>() {
                        // Safety: we checked if `self` is `T`.
                        Some(unsafe { self.downcast_unchecked() })
                    } else {
                        None
                    }
                }

                /// Downcasts `mut self` to `&mut T` by comparing the type. Otherwise,
                /// returns [`None`].
                ///
                /// This method is similar to [`<dyn Any>::downcast_mut`][`\<dyn Any\>::downcast_mut`].
                ///
                /// [`\<dyn Any\>::downcast_mut`]: https://doc.rust-lang.org/std/any/trait.Any.html#method.downcast_mut
                pub fn downcast_mut<T: ::core::any::Any>(&mut self) -> ::core::option::Option<&mut T> {
                    if self.is::<T>() {
                        // Safety: we checked if `self` is `T`.
                        Some(unsafe { self.downcast_mut_unchecked() })
                    } else {
                        None
                    }
                }

                /// Unsafely downcasts `&mut self` into `&mut T`.
                ///
                /// This method is similar to [`dyn Any::downcast_ref_unchecked`][`\<dyn Any\>::downcast_ref_unchecked`]
                /// and can be used in a stable Rust compiler.
                ///
                /// ## Safety
                /// The caller ensures that `self` is of type `T`. *calling this method
                /// is undefined behaviour! bad bad!...*
                ///
                /// [`\<dyn Any\>::downcast_ref_unchecked`]: https://doc.rust-lang.org/std/any/trait.Any.html#method.downcast_ref_unchecked
                pub unsafe fn downcast_unchecked<T: ::core::any::Any>(&self) -> &T {
                    ::core::debug_assert!(self.is::<T>());
                    unsafe { &*(self as *const dyn $Ty as *const T) }
                }

                /// Unsafely downcasts `&mut self` into `&mut T`.
                ///
                /// This method is similar to [`dyn Any::downcast_mut_unchecked`][`\<dyn Any\>::downcast_mut_unchecked`]
                /// and can be used in a stable Rust compiler.
                ///
                /// ## Safety
                /// The caller ensures that `self` is of type `T`. *calling this method
                /// is undefined behaviour! bad bad!...*
                ///
                /// [`\<dyn Any\>::downcast_mut_unchecked`]: https://doc.rust-lang.org/std/any/trait.Any.html#method.downcast_mut_unchecked
                pub unsafe fn downcast_mut_unchecked<T: ::core::any::Any>(&mut self) -> &mut T {
                    ::core::debug_assert!(self.is::<T>());
                    unsafe { &mut *(self as *mut dyn $Ty as *mut T) }
                }
            }

            impl dyn $Ty + Send + 'static {
                #[doc = concat!("Forwards to [`<dyn ", stringify!($Ty), ">::is`].")]
                ///
                #[doc = concat!("[`<dyn ", stringify!($Ty), ">::is`]: trait.", stringify!($Ty), ".html#method.is")]
                pub fn is<T: ::core::any::Any>(&self) -> bool {
                    <dyn $Ty>::is::<T>(self)
                }

                #[doc = concat!("Forwards to [`<dyn ", stringify!($Ty), ">::downcast`].")]
                ///
                #[doc = concat!("[`<dyn ", stringify!($Ty), ">::downcast`]: trait.", stringify!($Ty), ".html#method.downcast")]
                pub fn downcast<T: ::core::any::Any>(&self) -> Option<&T> {
                    <dyn $Ty>::downcast::<T>(self)
                }

                #[doc = concat!("Forwards to [`<dyn ", stringify!($Ty), ">::downcast_mut`].")]
                ///
                #[doc = concat!("[`<dyn ", stringify!($Ty), ">::downcast_mut`]: trait.", stringify!($Ty), ".html#method.downcast_mut")]
                pub fn downcast_mut<T: ::core::any::Any>(&mut self) -> Option<&mut T> {
                    <dyn $Ty>::downcast_mut::<T>(self)
                }

                #[doc = concat!("Forwards to [`<dyn ", stringify!($Ty), ">::downcast_unchecked`].")]
                ///
                /// ## Safety
                #[doc = concat!("Review the `Safety` section of [`<dyn ", stringify!($Ty), ">::downcast_unchecked`].")]
                ///
                #[doc = concat!("[`<dyn ", stringify!($Ty), ">::downcast_unchecked`]: trait.", stringify!($Ty), ".html#method.downcast_unchecked")]
                pub unsafe fn downcast_unchecked<T: ::core::any::Any>(&self) -> &T {
                    unsafe { <dyn $Ty>::downcast_unchecked::<T>(self) }
                }

                #[doc = concat!("Forwards to [`<dyn ", stringify!($Ty), ">::downcast_mut_unchecked`].")]
                ///
                /// ## Safety
                #[doc = concat!("Review the `Safety` section of [`<dyn ", stringify!($Ty), ">::downcast_mut_unchecked`].")]
                ///
                #[doc = concat!("[`<dyn ", stringify!($Ty), ">::downcast_mut_unchecked`]: trait.", stringify!($Ty), ".html#method.downcast_mut_unchecked")]
                pub unsafe fn downcast_mut_unchecked<T: ::core::any::Any>(&mut self) -> &mut T {
                    unsafe { <dyn $Ty>::downcast_mut_unchecked::<T>(self) }
                }
            }

            impl dyn $Ty + Send + Sync + 'static {
                #[doc = concat!("Forwards to [`<dyn ", stringify!($Ty), ">::is`].")]
                ///
                #[doc = concat!("[`<dyn ", stringify!($Ty), ">::is`]: trait.", stringify!($Ty), ".html#method.is")]
                pub fn is<T: ::core::any::Any>(&self) -> bool {
                    <dyn $Ty>::is::<T>(self)
                }

                #[doc = concat!("Forwards to [`<dyn ", stringify!($Ty), ">::downcast`].")]
                ///
                #[doc = concat!("[`<dyn ", stringify!($Ty), ">::downcast`]: trait.", stringify!($Ty), ".html#method.downcast")]
                pub fn downcast<T: ::core::any::Any>(&self) -> Option<&T> {
                    <dyn $Ty>::downcast::<T>(self)
                }

                #[doc = concat!("Forwards to [`<dyn ", stringify!($Ty), ">::downcast_mut`].")]
                ///
                #[doc = concat!("[`<dyn ", stringify!($Ty), ">::downcast_mut`]: trait.", stringify!($Ty), ".html#method.downcast_mut")]
                pub fn downcast_mut<T: ::core::any::Any>(&mut self) -> Option<&mut T> {
                    <dyn $Ty>::downcast_mut::<T>(self)
                }

                #[doc = concat!("Forwards to [`<dyn ", stringify!($Ty), ">::downcast_unchecked`].")]
                ///
                /// ## Safety
                #[doc = concat!("Review the `Safety` section of [`<dyn ", stringify!($Ty), ">::downcast_unchecked`].")]
                ///
                #[doc = concat!("[`<dyn ", stringify!($Ty), ">::downcast_unchecked`]: trait.", stringify!($Ty), ".html#method.downcast_unchecked")]
                pub unsafe fn downcast_unchecked<T: ::core::any::Any>(&self) -> &T {
                    unsafe { <dyn $Ty>::downcast_unchecked::<T>(self) }
                }

                #[doc = concat!("Forwards to [`<dyn ", stringify!($Ty), ">::downcast_mut_unchecked`].")]
                ///
                /// ## Safety
                #[doc = concat!("Review the `Safety` section of [`<dyn ", stringify!($Ty), ">::downcast_mut_unchecked`].")]
                ///
                #[doc = concat!("[`<dyn ", stringify!($Ty), ">::downcast_mut_unchecked`]: trait.", stringify!($Ty), ".html#method.downcast_mut_unchecked")]
                pub unsafe fn downcast_mut_unchecked<T: ::core::any::Any>(&mut self) -> &mut T {
                    unsafe { <dyn $Ty>::downcast_mut_unchecked::<T>(self) }
                }
            }
        };
    };
}

/// A declarative macro to implement for `dyn <Type>`:
///
/// - [`<dyn Type>::is`][`\<dyn Type\>::is`]: Checks if `self` is of type `T`.
/// - [`<dyn Type>::downcast`][`\<dyn Type\>::downcast`]: Downcasts `self` to <code>[`Option`]\<[`&`]T\></code>.
/// - [`<dyn Type>::downcast_mut`][`\<dyn Type\>::downcast_mut`]: Downcasts `self` to <code>[`Option`]\<[`&mut`] T\></code>.
/// - [`<dyn Type>::downcast_unchecked`][`\<dyn Type\>::downcast_unchecked`]: Downcasts `self` to <code>[`&`]T</code> unsafely.
/// - [`<dyn Type>::downcast_unchecked_mut`][`\<dyn Type\>::downcast_unchecked_mut`]: Downcasts `self` to <code>[`&mut`] T</code> unsafely.
///
/// [`\<dyn Type\>::downcast_unchecked_mut`]: https://doc.rust-lang.org/std/any/trait.Any.html#method.downcast_mut_unchecked
/// [`\<dyn Type\>::downcast_unchecked`]: https://doc.rust-lang.org/std/any/trait.Any.html#method.downcast_ref_unchecked
/// [`\<dyn Type\>::downcast_mut`]: https://doc.rust-lang.org/std/any/trait.Any.html#method.downcast_mut
/// [`\<dyn Type\>::downcast`]: https://doc.rust-lang.org/std/any/trait.Any.html#method.downcast_ref
/// [`\<dyn Type\>::is`]: https://doc.rust-lang.org/std/any/trait.Any.html#method.is
#[macro_export]
#[cfg(upcast_for_dyn_any)] // TODO(@auguwu): remove cfg once rust-version is 1.86
macro_rules! impl_dyn_any {
    ($Ty:ident) => {
        const _: () = {
            impl dyn $Ty + 'static {
                /// Compare if `self` is of type `T`, similar to [`<dyn Any>::is`][`\<dyn Any\>::is`].
                ///
                /// [`\<dyn Any\>::is`]: https://doc.rust-lang.org/std/any/trait.Any.html#method.is
                pub fn is<T: ::core::any::Any>(&self) -> bool {
                    (self as &dyn ::core::any::Any).is::<T>()
                }

                /// Downcasts `self` to `T` by comparing the type. Otherwise,
                /// returns [`None`].
                ///
                /// This method is similar to [`<dyn Any>::downcast_ref`][`\<dyn Any\>::downcast_ref`].
                ///
                /// [`\<dyn Any\>::downcast_ref`]: https://doc.rust-lang.org/std/any/trait.Any.html#method.downcast_ref
                pub fn downcast<T: ::core::any::Any>(&self) -> ::core::option::Option<&T> {
                    (self as &dyn ::core::any::Any).downcast_ref::<T>()
                }

                /// Downcasts `mut self` to `&mut T` by comparing the type. Otherwise,
                /// returns [`None`].
                ///
                /// This method is similar to [`<dyn Any>::downcast_mut`][`\<dyn Any\>::downcast_mut`].
                ///
                /// [`\<dyn Any\>::downcast_mut`]: https://doc.rust-lang.org/std/any/trait.Any.html#method.downcast_mut
                pub fn downcast_mut<T: ::core::any::Any>(&mut self) -> ::core::option::Option<&mut T> {
                    (self as &mut dyn ::core::any::Any).downcast_mut::<T>()
                }

                /// Unsafely downcasts `&mut self` into `&mut T`.
                ///
                /// This method is similar to [`dyn Any::downcast_ref_unchecked`][`\<dyn Any\>::downcast_ref_unchecked`]
                /// and can be used in a stable Rust compiler.
                ///
                /// ## Safety
                /// The caller ensures that `self` is of type `T`. *calling this method
                /// is undefined behaviour! bad bad!...*
                ///
                /// [`\<dyn Any\>::downcast_ref_unchecked`]: https://doc.rust-lang.org/std/any/trait.Any.html#method.downcast_ref_unchecked
                pub unsafe fn downcast_unchecked<T: ::core::any::Any>(&self) -> &T {
                    // TODO(@auguwu): use <self as &dyn ::core::any::Any>::downcast_ref_unchecked()
                    // once rust-lang/rust#90850 (`downcast_unchecked`) is stablised
                    ::core::debug_assert!(self.is::<T>());
                    unsafe { &*(self as *const dyn $Ty as *const T) }
                }

                /// Unsafely downcasts `&mut self` into `&mut T`.
                ///
                /// This method is similar to [`dyn Any::downcast_mut_unchecked`][`\<dyn Any\>::downcast_mut_unchecked`]
                /// and can be used in a stable Rust compiler.
                ///
                /// ## Safety
                /// The caller ensures that `self` is of type `T`. *calling this method
                /// is undefined behaviour! bad bad!...*
                ///
                /// [`\<dyn Any\>::downcast_mut_unchecked`]: https://doc.rust-lang.org/std/any/trait.Any.html#method.downcast_mut_unchecked
                pub unsafe fn downcast_mut_unchecked<T: ::core::any::Any>(&mut self) -> &mut T {
                    // TODO(@auguwu): use <self as &dyn ::core::any::Any>::downcast_mut_unchecked()
                    // once rust-lang/rust#90850 (`downcast_unchecked`) is stablised
                    ::core::debug_assert!(self.is::<T>());
                    unsafe { &mut *(self as *mut dyn $Ty as *mut T) }
                }
            }

            impl dyn $Ty + Send + 'static {
                #[doc = concat!("Forwards to [`<dyn ", stringify!($Ty), ">::is`].")]
                ///
                #[doc = concat!("[`<dyn ", stringify!($Ty), ">::is`]: trait.", stringify!($Ty), ".html#method.is")]
                pub fn is<T: ::core::any::Any>(&self) -> bool {
                    <dyn $Ty>::is::<T>(self)
                }

                #[doc = concat!("Forwards to [`<dyn ", stringify!($Ty), ">::downcast`].")]
                ///
                #[doc = concat!("[`<dyn ", stringify!($Ty), ">::downcast`]: trait.", stringify!($Ty), ".html#method.downcast")]
                pub fn downcast<T: ::core::any::Any>(&self) -> Option<&T> {
                    <dyn $Ty>::downcast::<T>(self)
                }

                #[doc = concat!("Forwards to [`<dyn ", stringify!($Ty), ">::downcast_mut`].")]
                ///
                #[doc = concat!("[`<dyn ", stringify!($Ty), ">::downcast_mut`]: trait.", stringify!($Ty), ".html#method.downcast_mut")]
                pub fn downcast_mut<T: ::core::any::Any>(&mut self) -> Option<&mut T> {
                    <dyn $Ty>::downcast_mut::<T>(self)
                }

                #[doc = concat!("Forwards to [`<dyn ", stringify!($Ty), ">::downcast_unchecked`].")]
                ///
                /// ## Safety
                #[doc = concat!("Review the `Safety` section of [`<dyn ", stringify!($Ty), ">::downcast_unchecked`].")]
                ///
                #[doc = concat!("[`<dyn ", stringify!($Ty), ">::downcast_unchecked`]: trait.", stringify!($Ty), ".html#method.downcast_unchecked")]
                pub unsafe fn downcast_unchecked<T: ::core::any::Any>(&self) -> &T {
                    unsafe { <dyn $Ty>::downcast_unchecked::<T>(self) }
                }

                #[doc = concat!("Forwards to [`<dyn ", stringify!($Ty), ">::downcast_mut_unchecked`].")]
                ///
                /// ## Safety
                #[doc = concat!("Review the `Safety` section of [`<dyn ", stringify!($Ty), ">::downcast_mut_unchecked`].")]
                ///
                #[doc = concat!("[`<dyn ", stringify!($Ty), ">::downcast_mut_unchecked`]: trait.", stringify!($Ty), ".html#method.downcast_mut_unchecked")]
                pub unsafe fn downcast_mut_unchecked<T: ::core::any::Any>(&mut self) -> &mut T {
                    unsafe { <dyn $Ty>::downcast_mut_unchecked::<T>(self) }
                }
            }

            impl dyn $Ty + Send + Sync + 'static {
                #[doc = concat!("Forwards to [`<dyn ", stringify!($Ty), ">::is`].")]
                ///
                #[doc = concat!("[`<dyn ", stringify!($Ty), ">::is`]: trait.", stringify!($Ty), ".html#method.is")]
                pub fn is<T: ::core::any::Any>(&self) -> bool {
                    <dyn $Ty>::is::<T>(self)
                }

                #[doc = concat!("Forwards to [`<dyn ", stringify!($Ty), ">::downcast`].")]
                ///
                #[doc = concat!("[`<dyn ", stringify!($Ty), ">::downcast`]: trait.", stringify!($Ty), ".html#method.downcast")]
                pub fn downcast<T: ::core::any::Any>(&self) -> Option<&T> {
                    <dyn $Ty>::downcast::<T>(self)
                }

                #[doc = concat!("Forwards to [`<dyn ", stringify!($Ty), ">::downcast_mut`].")]
                ///
                #[doc = concat!("[`<dyn ", stringify!($Ty), ">::downcast_mut`]: trait.", stringify!($Ty), ".html#method.downcast_mut")]
                pub fn downcast_mut<T: ::core::any::Any>(&mut self) -> Option<&mut T> {
                    <dyn $Ty>::downcast_mut::<T>(self)
                }

                #[doc = concat!("Forwards to [`<dyn ", stringify!($Ty), ">::downcast_unchecked`].")]
                ///
                /// ## Safety
                #[doc = concat!("Review the `Safety` section of [`<dyn ", stringify!($Ty), ">::downcast_unchecked`].")]
                ///
                #[doc = concat!("[`<dyn ", stringify!($Ty), ">::downcast_unchecked`]: trait.", stringify!($Ty), ".html#method.downcast_unchecked")]
                pub unsafe fn downcast_unchecked<T: ::core::any::Any>(&self) -> &T {
                    unsafe { <dyn $Ty>::downcast_unchecked::<T>(self) }
                }

                #[doc = concat!("Forwards to [`<dyn ", stringify!($Ty), ">::downcast_mut_unchecked`].")]
                ///
                /// ## Safety
                #[doc = concat!("Review the `Safety` section of [`<dyn ", stringify!($Ty), ">::downcast_mut_unchecked`].")]
                ///
                #[doc = concat!("[`<dyn ", stringify!($Ty), ">::downcast_mut_unchecked`]: trait.", stringify!($Ty), ".html#method.downcast_mut_unchecked")]
                pub unsafe fn downcast_mut_unchecked<T: ::core::any::Any>(&mut self) -> &mut T {
                    unsafe { <dyn $Ty>::downcast_mut_unchecked::<T>(self) }
                }
            }
        };
    };
}

mod private {
    pub trait Sealed {}

    impl<T: ?Sized + super::AsAny> Sealed for T {}
}

#[cfg(test)]
mod tests {
    #[cfg(upcast_for_dyn_any)]
    #[allow(dead_code)]
    #[test]
    fn upcast_for_dyn_any_compiles_correctly() {
        use core::any::Any;

        trait A: Any {}
        impl A for String {}

        impl_dyn_any!(A);

        let mut a: Box<dyn A> = Box::new(String::new());

        assert!(a.is::<String>());
        assert!(a.downcast::<String>().is_some());
        assert!(a.downcast_mut::<String>().is_some());
    }
}
