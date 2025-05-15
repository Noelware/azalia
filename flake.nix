# 🐻‍❄️🪚 azalia: Noelware's Rust commons library.
# Copyright (c) 2024-2025 Noelware, LLC. <team@noelware.org>
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
{
  description = "Collection of Rust crates that are used by and built for Noelware's projects";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
      };
    };

    flake-compat = {
      url = "github:edolstra/flake-compat";
      flake = false;
    };
  };

  nixConfig = {
    extra-substituters = [
      # TODO: switch to https://nix.noelware.org
      "https://noelware.cachix.org"
    ];

    extra-trusted-public-keys = [
      "noelware.cachix.org-1:22A8ELRjkqEycSHz+R5A5ReX2jyjU3rftsBmlD6thn0="
    ];
  };

  outputs = {
    nixpkgs,
    rust-overlay,
    ...
  }: let
    eachSystem = nixpkgs.lib.genAttrs nixpkgs.lib.systems.flakeExposed;
    overlays = [
      (import rust-overlay)
    ];

    nixpkgsFor = system:
      import nixpkgs {
        inherit system overlays;
      };
  in {
    formatter = eachSystem (system: (nixpkgsFor system).alejandra);
    devShells = eachSystem (system: let
      pkgs = nixpkgsFor system;
    in {
      default = import ./nix/devshell.nix {inherit pkgs;};
    });
  };
}
