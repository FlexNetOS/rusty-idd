#!/usr/bin/env bash
set -euo pipefail

mode="${1:-ci}"

workspace="${GITHUB_WORKSPACE:-}"
if [[ -z "$workspace" ]]; then
  workspace="$(git rev-parse --show-toplevel 2>/dev/null || pwd)"
fi
workspace="$(cd "$workspace" && pwd -P)"

find_meta_root() {
  if [[ -n "${RUSTY_IDD_META_ROOT:-}" ]]; then
    printf '%s\n' "$RUSTY_IDD_META_ROOT"
    return
  fi
  if [[ -n "${META_ROOT:-}" ]]; then
    printf '%s\n' "$META_ROOT"
    return
  fi

  local cursor="$workspace"
  while [[ "$cursor" != "/" ]]; do
    if [[ -f "$cursor/.meta.yaml" || -d "$cursor/envctl" ]]; then
      printf '%s\n' "$cursor"
      return
    fi
    cursor="$(dirname "$cursor")"
  done

  printf '%s\n' "$(cd "$workspace/.." && pwd -P)/meta"
}

META_ROOT="$(find_meta_root)"
META_ROOT="$(mkdir -p "$META_ROOT" && cd "$META_ROOT" && pwd -P)"
export META_ROOT

ci_group_start() {
  if [[ -n "${GITHUB_ACTIONS:-}" ]]; then
    printf '::group::%s\n' "$1"
  else
    printf '==> %s\n' "$1"
  fi
}

ci_group_end() {
  if [[ -n "${GITHUB_ACTIONS:-}" ]]; then
    printf '::endgroup::\n'
  fi
}

timed_step() {
  local label="$1"
  shift
  local start end elapsed status
  start="$(date +%s)"
  ci_group_start "$label"
  set +e
  "$@"
  status=$?
  set -e
  end="$(date +%s)"
  elapsed=$((end - start))
  if [[ "$status" -eq 0 ]]; then
    printf 'Rusty IDD bootstrap span: %s completed in %ss\n' "$label" "$elapsed"
  else
    printf 'Rusty IDD bootstrap span: %s failed in %ss\n' "$label" "$elapsed" >&2
  fi
  ci_group_end
  return "$status"
}

select_rust_layout() {
  if [[ -n "${RUSTY_IDD_RUST_LAYOUT:-}" ]]; then
    printf '%s\n' "$RUSTY_IDD_RUST_LAYOUT"
    return
  fi

  if [[ -n "${GITHUB_ACTIONS:-}" ]]; then
    printf '%s\n' "isolated"
    return
  fi

  if [[ -x "$META_ROOT/.toolchains/cargo/bin/rustup" && -d "$META_ROOT/.toolchains/rustup" ]]; then
    printf '%s\n' "toolchains"
    return
  fi

  printf '%s\n' "isolated"
}

rust_layout="$(select_rust_layout)"
case "$rust_layout" in
  isolated|env)
    RUSTUP_HOME="${RUSTUP_HOME:-$META_ROOT/.env/rust/rustup}"
    CARGO_HOME="${CARGO_HOME:-$META_ROOT/.env/rust/cargo}"
    RUSTY_IDD_RUST_BIN="${RUSTY_IDD_RUST_BIN:-$META_ROOT/.env/rust/bin}"
    ;;
  toolchains)
    RUSTUP_HOME="${RUSTUP_HOME:-$META_ROOT/.toolchains/rustup}"
    CARGO_HOME="${CARGO_HOME:-$META_ROOT/.toolchains/cargo}"
    RUSTY_IDD_RUST_BIN="${RUSTY_IDD_RUST_BIN:-$CARGO_HOME/bin}"
    ;;
  *)
    echo "unknown Rusty IDD Rust layout '$rust_layout'; expected isolated, env, or toolchains" >&2
    exit 2
    ;;
esac

RUSTY_IDD_CACHE_BASE="${RUSTY_IDD_CACHE_BASE:-$META_ROOT/.cache/rust}"
CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-$workspace/target}"

export RUSTY_IDD_RUST_LAYOUT="$rust_layout"
export RUSTUP_HOME CARGO_HOME RUSTY_IDD_RUST_BIN RUSTY_IDD_CACHE_BASE CARGO_TARGET_DIR
export PATH="$RUSTY_IDD_RUST_BIN:$CARGO_HOME/bin:$PATH"

mkdir -p "$RUSTUP_HOME" "$CARGO_HOME" "$RUSTY_IDD_RUST_BIN" "$RUSTY_IDD_CACHE_BASE" "$CARGO_TARGET_DIR"

case "$mode" in
  ci|promote)
    toolchain="${RUSTY_IDD_RUST_TOOLCHAIN:-nightly}"
    components=(rustfmt clippy)
    strict_rust_contract=true
    ;;
  msrv)
    toolchain="${RUSTY_IDD_RUST_TOOLCHAIN:-1.96.0}"
    components=()
    strict_rust_contract=false
    ;;
  release)
    toolchain="${RUSTY_IDD_RUST_TOOLCHAIN:-1.96.0}"
    components=()
    strict_rust_contract=false
    ;;
  *)
    echo "unknown envctl Rust mode: $mode" >&2
    exit 2
    ;;
esac

if ! command -v rustup >/dev/null 2>&1; then
  echo "rustup is required on PATH so it can materialize the meta-owned RUSTUP_HOME=$RUSTUP_HOME" >&2
  exit 1
fi

rustup_args=(toolchain install "$toolchain" --profile minimal)
for component in "${components[@]}"; do
  rustup_args+=(--component "$component")
done
timed_step "rustup toolchain install $toolchain" rustup "${rustup_args[@]}"

export RUSTUP_TOOLCHAIN="$toolchain"

if [[ "$strict_rust_contract" == true ]]; then
  codegen_component_ok=false
  for component in rustc-codegen-gcc rustc-codegen-gcc-preview; do
    if timed_step "rustup component add $component" rustup component add "$component" --toolchain "$toolchain"; then
      codegen_component_ok=true
      break
    fi
  done
  if [[ "$codegen_component_ok" != true ]]; then
    echo "nightly Rust is installed, but no rustc_codegen_gcc component was available for $toolchain" >&2
    exit 1
  fi
fi

cargo_for_toolchain() {
  RUSTUP_TOOLCHAIN="$toolchain" cargo "$@"
}

install_cargo_tool() {
  local crate="$1"
  local binary="$2"
  local binary_path="$RUSTY_IDD_RUST_BIN/$binary"
  if [[ -x "$binary_path" ]]; then
    printf 'Rusty IDD bootstrap reuse: %s at %s\n' "$binary" "$binary_path"
    return
  fi
  timed_step "cargo install $crate" cargo_for_toolchain install --locked --root "$RUSTY_IDD_RUST_BIN/.." "$crate"
}

install_cache_wrapper() {
  local wrapper="${RUSTY_IDD_CACHE_WRAPPER:-kache}"
  case "$wrapper" in
    kache|hurry|zccache) ;;
    sccache)
      echo "sccache is not part of this CI bootstrap; provision kache, hurry, or zccache through meta/envctl" >&2
      exit 1
      ;;
    *)
      echo "unsupported Rust cache wrapper '$wrapper'; expected kache, hurry, or zccache" >&2
      exit 1
      ;;
  esac

  if [[ ! -x "$RUSTY_IDD_RUST_BIN/$wrapper" ]]; then
    install_cargo_tool "$wrapper" "$wrapper"
  fi

  local wrapper_path
  wrapper_path="$RUSTY_IDD_RUST_BIN/$wrapper"
  local cache_root="$RUSTY_IDD_CACHE_BASE/$wrapper"
  mkdir -p "$cache_root"

  case "$wrapper" in
    kache)
      export KACHE_CACHE_DIR="$cache_root"
      ;;
    hurry)
      export HURRY_CACHE_DIR="$cache_root"
      ;;
    zccache)
      export ZCCACHE_CACHE_DIR="$cache_root"
      ;;
  esac

  export RUSTC_WRAPPER="$wrapper_path"
  export RUSTY_IDD_CACHE_WRAPPER="$wrapper"
  export RUSTY_IDD_CACHE_ROOT="$cache_root"
}

if [[ "$strict_rust_contract" == true ]]; then
  install_cache_wrapper
  install_cargo_tool wild-linker wild
  export RUSTY_IDD_LINKER_PATH="$RUSTY_IDD_RUST_BIN/wild"
  export RUSTY_IDD_CODEGEN_BACKEND="rustc_codegen_gcc"
  install_cargo_tool cargo-audit cargo-audit

  if ! command -v clang >/dev/null 2>&1; then
    echo "clang is required as the wild-linker driver for the strict Linux Rust contract" >&2
    exit 1
  fi
  export CC="${CC:-clang}"
  export RUSTFLAGS="${RUSTFLAGS:-} -C linker=clang -C link-arg=-fuse-ld=$RUSTY_IDD_LINKER_PATH"
elif [[ "$mode" == "ci" || "$mode" == "promote" ]]; then
  install_cargo_tool cargo-audit cargo-audit
fi

rustc_path="$(rustup which rustc)"
cargo_path="$(rustup which cargo)"

echo "Rusty IDD envctl Rust environment"
echo "- mode: $mode"
echo "- layout: $RUSTY_IDD_RUST_LAYOUT"
echo "- META_ROOT: $META_ROOT"
echo "- RUSTUP_HOME: $RUSTUP_HOME"
echo "- CARGO_HOME: $CARGO_HOME"
echo "- CARGO_TARGET_DIR: $CARGO_TARGET_DIR"
echo "- RUSTUP_TOOLCHAIN: $RUSTUP_TOOLCHAIN"
echo "- rustc path: $rustc_path"
echo "- cargo path: $cargo_path"
echo "- RUSTC_WRAPPER: ${RUSTC_WRAPPER:-<unset>}"
echo "- RUSTFLAGS: ${RUSTFLAGS:-<unset>}"

if [[ -n "${GITHUB_ENV:-}" ]]; then
  {
    echo "META_ROOT=$META_ROOT"
    echo "RUSTY_IDD_RUST_LAYOUT=$RUSTY_IDD_RUST_LAYOUT"
    echo "RUSTUP_HOME=$RUSTUP_HOME"
    echo "CARGO_HOME=$CARGO_HOME"
    echo "RUSTY_IDD_RUST_BIN=$RUSTY_IDD_RUST_BIN"
    echo "RUSTY_IDD_CACHE_BASE=$RUSTY_IDD_CACHE_BASE"
    echo "CARGO_TARGET_DIR=$CARGO_TARGET_DIR"
    echo "RUSTUP_TOOLCHAIN=$RUSTUP_TOOLCHAIN"
    echo "RUSTC_WRAPPER=${RUSTC_WRAPPER:-}"
    echo "RUSTY_IDD_CACHE_WRAPPER=${RUSTY_IDD_CACHE_WRAPPER:-}"
    echo "RUSTY_IDD_CACHE_ROOT=${RUSTY_IDD_CACHE_ROOT:-}"
    echo "RUSTY_IDD_LINKER_PATH=${RUSTY_IDD_LINKER_PATH:-}"
    echo "RUSTY_IDD_CODEGEN_BACKEND=${RUSTY_IDD_CODEGEN_BACKEND:-}"
    echo "CC=${CC:-}"
    echo "KACHE_CACHE_DIR=${KACHE_CACHE_DIR:-}"
    echo "HURRY_CACHE_DIR=${HURRY_CACHE_DIR:-}"
    echo "ZCCACHE_CACHE_DIR=${ZCCACHE_CACHE_DIR:-}"
    echo "RUSTFLAGS=${RUSTFLAGS:-}"
  } >> "$GITHUB_ENV"
fi

if [[ -n "${GITHUB_PATH:-}" ]]; then
  {
    echo "$RUSTY_IDD_RUST_BIN"
    echo "$CARGO_HOME/bin"
  } >> "$GITHUB_PATH"
fi
