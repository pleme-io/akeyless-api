{
  description = "Auto-generated Rust SDK for the Akeyless API (604 endpoints, 1334 types)";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.11";
    substrate = { url = "github:pleme-io/substrate"; inputs.nixpkgs.follows = "nixpkgs"; };
    flake-utils.url = "github:numtide/flake-utils";
  };
  outputs = inputs: (import "${inputs.substrate}/lib/repo-flake.nix" {
    inherit (inputs) nixpkgs flake-utils;
  }) {
    self = inputs.self;
    language = "rust";
    pname = "akeyless-api";
    description = "Auto-generated Rust SDK for the Akeyless API";
  };
}
