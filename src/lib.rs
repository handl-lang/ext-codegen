//! `hnl-ext-codegen` — the `codegen` extension for `hnl` (ADR-0149, PL-004).
//!
//! The pipeline lives in `handl-plugin-codegen` (`CodegenRegistry` over
//! the `hnl-engine-codegen` desired-state kernel) plus the target modules
//! linked through `handl-plugin-ext-codegen`'s features — `c` (backend +
//! libclang), `webidl`, `python` (emitter composing backend `c`), and
//! `cli` — all on by default. This crate is the *packaging*: it exports
//! the ADR-0149 ABI (`handl_ext_manifest` / `handl_ext_dispatch` /
//! `handl_ext_free`) and routes the `hnl codegen [plan|apply|destroy|revert] <c|webidl|python|cli>
//! [<spec> | --from-url <url>] --output <dir> [flags]` argv to
//! [`handl_plugin_ext_codegen::codegen_command`] with the canonical kit's
//! language —
//! byte-for-byte the semantics the built-in `codegen` arm had before the
//! host went dumb. Clean break from `sdkgen`: no alias (PL-004 T-076).
//!
//! `codegen_command` renders its own diagnostics with `eprintln!`;
//! running in-process those land on the host's stderr verbatim, so the
//! response carries only the exit code.

use std::path::Path;

use handl_ext::{ExtRequest, ExtResponse, ExtensionManifest};
use handl_kit_pkg::KitProjectExt;
use handl_kit_prelude::Kit;

handl_ext::export!(
    ExtensionManifest::new("codegen")
        .version(env!("CARGO_PKG_VERSION"))
        .command(
            "codegen",
            "codegen [plan|apply|destroy|revert] <c|webidl|python|cli> [<spec> | --from-url <url>] --output <dir> [--target <triple>] [--include <glob>] [--exclude <glob>] [--config <yaml>] [--source-url <url>] [--emit-spec <path|->] [--cache-dir <dir>] [--check|--dry-run] [--run <id>]",
        ),
    dispatch
);

/// ABI dispatch: only the `codegen` command is declared.
fn dispatch(cmd: &str, req: &ExtRequest) -> ExtResponse {
    match cmd {
        "codegen" => run(&req.args, Path::new(&req.cwd)),
        other => ExtResponse::error(2, format!("codegen: unknown command `{other}`\n")),
    }
}

/// Run `codegen_command` with the project-discovered (or canonical) kit's
/// language — the same `with_tooling_kit` injection the built-in host used
/// (ADR-0128 §"The 0.8 seam"). `codegen_command` prints to stderr itself;
/// the response carries the exit code only.
fn run(args: &[String], cwd: &Path) -> ExtResponse {
    let kit = Kit::discover(cwd)
        .or_else(|_| Kit::new())
        .expect("canonical kit must assemble");
    let code = handl_plugin_ext_codegen::codegen_command(kit.lang(), args);
    ExtResponse {
        code,
        stdout: String::new(),
        stderr: String::new(),
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CStr;

    #[test]
    fn no_args_is_usage_error() {
        // `codegen_command` prints the usage line to the process stderr.
        let res = run(&[], Path::new("."));
        assert_eq!(res.code, 2);
    }

    #[test]
    fn unknown_target_is_usage_error() {
        let res = run(&["bogus".into()], Path::new("."));
        assert_eq!(res.code, 2);
    }

    /// The `export!`-generated manifest symbol, called directly.
    #[test]
    fn manifest_declares_codegen() {
        let raw = handl_ext_manifest();
        assert!(!raw.is_null());
        let json = unsafe { CStr::from_ptr(raw) }
            .to_string_lossy()
            .into_owned();
        unsafe { handl_ext_free(raw) };
        let manifest: ExtensionManifest = serde_json::from_str(&json).unwrap();
        assert_eq!(manifest.name, "codegen");
        let names: Vec<&str> = manifest.commands.iter().map(|c| c.name.as_str()).collect();
        assert_eq!(names, ["codegen"]);
    }
}
