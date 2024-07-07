# 🐻‍❄️🪚 Azalia: Family of crates that implement common Rust code
# Copyright (c) 2024 Noelware, LLC. <team@noelware.org>
#
# Permission is hereby granted, free of charge, to any person obtaining a copy
# of this software and associated documentation files (the "Software"), to deal
# in the Software without restriction, including without limitation the rights
# to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
# copies of the Software, and to permit persons to whom the Software is
# furnished to do so, subject to the following conditions:
#
# The above copyright notice and this permission notice shall be included in all
# copies or substantial portions of the Software.
#
# THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
# IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
# FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
# AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
# LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
# OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
# SOFTWARE.

# We use Noeldoc to allow the documentation generator that lives in the `crates` repository
# to get the documentation for all crates except `azalia-serde`. Each Azalia release builds
# the documentation suite and publishes the artifacts to S3.

noeldoc {
    version     = ">=0.1.0"
    experiments = ["dockerRunner"]

    extension "noeldoc/rustdoc" {
        version = "0.1.0"
    }
}

locals {
    cargoTOML = readCargoToml({ root = "${cwd}/Cargo.toml" })
}

project "azalia" {
    description = "🐻‍❄️🪚 Family of crates that implement common Rust code"
    version     = local.cargoTOML.workspace.package.version

    extension "noeldoc/rustdoc" {
        cargoTOML = local.cargoTOML
        crates    = omit(cargoTOML.workspace.members, ["azalia-serde"])

        # Noeldoc's Rustdoc extension has the ability to extract `#[doc(cfg)]` comments.
        #
        # example:
        #
        # #[cfg_attr(any(noeldoc, rustdoc), doc(cfg(feature = "weow")))]
        rustflags = [
            "--cfg=noeldoc"
        ]
    }
}
