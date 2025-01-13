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

#[test]
fn merge() {
    let cases = trybuild::TestCases::new();
    cases.compile_fail("./tests/ui/merge/union.rs");
    cases.compile_fail("./tests/ui/merge/enum.rs");

    cases.pass("./tests/ui/merge/strategy_multi.rs");
    cases.pass("./tests/ui/merge/custom_strategy.rs");
    cases.pass("./tests/ui/merge/skip_test.rs");
    cases.pass("./tests/ui/merge/generics.rs");
    cases.pass("./tests/ui/merge/unnamed.rs");
    cases.pass("./tests/ui/merge/struct.rs");
}

#[test]
#[cfg(feature = "unstable")]
fn unstable_env_test() {
    let cases = trybuild::TestCases::new();

    cases.compile_fail("./tests/ui/unstable/env_test/disallow_inputs.rs");
    cases.pass("./tests/ui/unstable/env_test/should_work.rs");
}
