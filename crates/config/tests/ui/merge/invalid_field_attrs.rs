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

#[derive(azalia_config::merge::Merge)]
pub struct A {
    #[merge(skip)]
    #[merge(skip)]
    a: bool,
}

#[derive(azalia_config::merge::Merge)]
pub struct B {
    #[merge(strategy = "x + 1")]
    b: u32,
}

#[derive(azalia_config::merge::Merge)]
pub struct C {
    #[merge(strategy = 1234)]
    c: String,
}

#[derive(azalia_config::merge::Merge)]
pub struct D {
    #[merge(strategy = azalia_config::merge::strategy::f32::with_negatives)]
    #[merge(strategy = azalia_config::merge::strategy::f32::with_negatives)]
    d: f32,
}

#[derive(azalia_config::merge::Merge)]
pub struct E {
    #[merge(skip = "data")]
    e: usize,
}

#[derive(azalia_config::merge::Merge)]
pub struct F {
    #[merge(unknown_field)]
    f: f64,
}

fn main() {}
