#!/usr/bin/env bash
set -euo pipefail

required=(
  META_ROOT
  RUSTUP_HOME
  CARGO_HOME
  RUSTUP_TOOLCHAIN
  RUSTC_WRAPPER
  RUSTY_IDD_CACHE_WRAPPER
  RUSTY_IDD_CACHE_ROOT
  RUSTY_IDD_LINKER_PATH
  RUSTY_IDD_CODEGEN_BACKEND
)

for name in "${required[@]}"; do
  if [[ -z "${!name:-}" ]]; then
    echo "$name is required for strict envctl Rust audit" >&2
    exit 1
  fi
done

if [[ "$RUSTY_IDD_CACHE_WRAPPER" == "sccache" ]]; then
  echo "sccache is not accepted by this CI audit path; use kache, hurry, or zccache" >&2
  exit 1
fi

rustc_path="$(rustup which rustc)"
cargo_path="$(rustup which cargo)"

cargo run --bin rusty-idd -- codex system-audit \
  --rust-toolchain \
  --meta-root "$META_ROOT" \
  --rust-toolchain-name "$RUSTUP_TOOLCHAIN" \
  --rustc-path "$rustc_path" \
  --cargo-bin "$cargo_path" \
  --rustup-home "$RUSTUP_HOME" \
  --cargo-home "$CARGO_HOME" \
  --rustc-wrapper "$RUSTC_WRAPPER" \
  --cache-wrapper "$RUSTY_IDD_CACHE_WRAPPER" \
  --cache-root "$RUSTY_IDD_CACHE_ROOT" \
  --linker-path "$RUSTY_IDD_LINKER_PATH" \
  --codegen-backend "$RUSTY_IDD_CODEGEN_BACKEND"
