# ext-codegen

`hnl-ext-codegen` — `hnl codegen` as a dynamic `hnl` extension: the
desired-state codegen pipeline packaged as a
`libhnl_ext_codegen.{so,dylib,dll}` cdylib the dumb `hnl` host discovers
at runtime (`--ext-dir`, `$HANDL_EXT_DIR`, `~/.handl/ext/`, or a dev
`target/debug/` next to the `hnl` binary).

## Layout

| path | role |
| --- | --- |
| `src/` | extension ABI exports + `codegen` command wiring |
| `xtask/` | vendored gate crate (`fmt`, `clippy`, `test`, `doc`, `gate`) |
| `workflows/gate.yaml` | per-push gate — `handl-gates` rust campaign via bupha |

## Contract

- Renamed from `ext-sdkgen` (PL-004 T-076) — clean break, no `hnl sdkgen`
  alias. The IR/registry core is `handl-plugin-codegen` in `host`; target
  modules live in the `tool-codegen-*` sibling repos (`c`, `webidl`,
  `python`, `cli`, `file`).
- Commands: `hnl codegen [plan|apply|destroy|revert] <c|webidl|python|cli>`
  over `Source<I> → IR → Emitter<I> → Change set` on the `hnl-tool-codegen`
  kernel (`FsStore` + `Engine::apply`, manifest + journal under
  `.handl/codegen/`).
- Builds `cdylib` + `rlib`; `cargo test` links the `rlib` statically.

## Verification

```bash
cargo run --manifest-path xtask/Cargo.toml -- gate   # fmt → clippy → test → doc
```

<!-- myco:agent-awareness -->

## Agent awareness

- Session start / any work discussion: run `myco status --prompt`
  before proposing next steps — know the board first.
- User describes new work → offer to capture it (`myco todo add`,
  `issue new`, `proposal new`). Don't wait to be asked.
- Picking up a task: `myco resume <ID>` is the briefing.
- Wrapping up: `myco handoff <ID>` anything resumable; keep statuses
  honest (claimed → in-progress → blocked/done).

<!-- /myco:agent-awareness -->
