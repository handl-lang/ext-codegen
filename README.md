# ext-codegen

`hnl-ext-codegen` — `hnl codegen` as a dynamic `hnl` extension.

An `ext-*` crate: the desired-state codegen pipeline packaged as a
`libhnl_ext_codegen.{so,dylib,dll}` shared library for the dumb `hnl` host. The
host discovers it via `--ext-dir`, `$HANDL_EXT_DIR`, `~/.handl/ext/`, or a dev
`target/debug/` next to the `hnl` binary.

PL-004 T-076: renamed from `ext-sdkgen` — clean break, `hnl sdkgen` is gone
(no alias). The IR/registry core is `handl-plugin-codegen` in the `host`
repo; the target modules live in the `engine-codegen-*` sibling repos
(`c`, `webidl`, `python`, `cli`, `file`).

## Commands

```text
hnl codegen [plan|apply|destroy|revert] <c|webidl|python|cli> [<spec> | --from-url <url>] --output <dir>
           [--target <triple>] [--include <glob>] [--exclude <glob>] [--config <yaml>]
           [--source-url <url>] [--emit-spec <path|->] [--cache-dir <dir>]
           [--check|--dry-run] [--run <id>]
```

All targets are linked through `handl-plugin-ext-codegen`'s default features
(`c`, `webidl`, `python` — which implies `c` — and `cli`). The canonical
kit's language is injected into `codegen_command` exactly as the built-in
host did (its own `eprintln!` diagnostics reach the host's stderr verbatim
in-process).

The pipeline is `Source<I> → IR → Emitter<I> → Change set → Plan/Apply/
Destroy/Revert` over the `hnl-engine-codegen` kernel (`FsStore` +
`Engine::apply`, manifest + journal under `.handl/codegen/`).

## Development

```bash
cargo run --manifest-path xtask/Cargo.toml -- gate   # fmt → clippy → test → doc
```

`xtask/` provides the gate tasks (`fmt`, `clippy`, `test`, `doc`, `check`,
`gate`); `workflows/gate.yaml` is the per-push gate — the `handl-gates` bundle's rust campaign (fmt → clippy → test → doc), run by bupha/the fleet runner via `peregrin campaign run`. The crate
builds a `cdylib` + `rlib`; `cargo test` links the `rlib` statically.

Deps resolve from sibling checkouts via the `[patch]` table in `Cargo.toml`
(host@main does not yet carry the `handl-plugin-codegen` rename or
`handl-plugin-ext-codegen`) — keep the `handl-lang/*` sibling layout.
