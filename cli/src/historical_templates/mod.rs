//! Opt-in scaffolding: tests start their own historical Surfpool forks.
//! Reconstruction and execution stay in Surfpool and SVM Replay.
mod node;
mod rust;
#[cfg(test)]
mod tests;

use {
    crate::template::{AnchorVersion, TestTemplate},
    anyhow::{bail, Result},
    std::{fs, path::Path},
};

pub(crate) fn validate(template: &TestTemplate) -> Result<()> {
    if matches!(template, TestTemplate::Litesvm | TestTemplate::Mollusk) {
        bail!(
            "--historical requires --test-template mocha, jest or rust; LiteSVM/Mollusk execute \
             in-process, not through an RPC fork"
        );
    }
    Ok(())
}

pub(crate) fn write(
    name: &str,
    javascript: bool,
    template: &TestTemplate,
    version: AnchorVersion,
) -> Result<()> {
    validate(template)?;
    if template.uses_node() {
        fs::write("tests/historical.js", include_str!("historical_fork.js"))?;
        if !javascript {
            fs::write(
                "tests/historical.d.ts",
                include_str!("historical_fork.d.ts"),
            )?;
            // The test imports its IDL JSON to bind the program to the fork's
            // own provider.
            let path = Path::new("tsconfig.json");
            let mut config: serde_json::Value = serde_json::from_slice(&fs::read(path)?)?;
            config["compilerOptions"]["resolveJsonModule"] = true.into();
            fs::write(path, serde_json::to_string_pretty(&config)? + "\n")?;
        }
        let extension = if javascript { "js" } else { "ts" };
        let suffix = if *template == TestTemplate::Jest {
            ".test"
        } else {
            ""
        };
        fs::write(
            format!("tests/{name}{suffix}.{extension}"),
            node::test(name, javascript, *template == TestTemplate::Jest, version),
        )?;
    } else {
        rust::write(name, version)?;
    }
    fs::write("HISTORICAL.md", GUIDE)?;
    Ok(())
}

const GUIDE: &str = r#"# Historical tests

These tests run your program against Solana mainnet state at a chosen point in history. Each
test starts its own historical Surfpool fork: Surfpool serves the ordinary RPC that Anchor's
clients use, and SVM Replay supplies the historical accounts and runtime.

Requirements: `surfpool` with historical support (`surfpool start --after`) on PATH, an
activated SVM Replay bundle, and a provider with archive access configured in the
`[historical]` section of Anchor.toml. Keys come from the environment variable it names:

```sh
export API_ALCHEMY="your-account-archive-key"
anchor build --arch v0
anchor test --skip-build --skip-deploy
```

Build for a program format the boundary's runtime supports (`v0` for the default boundary).
Each test chooses its boundary in code (`after: "413023260"`, a slot or a transaction
signature). The fork's clock is frozen: it moves only when a test calls `advance()`.

The test funds its wallet and installs this build of the program with Surfpool's fast
program-writing helper (`installProgram`), which is not a loader deployment. Accounts the test
creates are declared first (`setAccount`). Remove event and account listeners before `close()`. These are local, hypothetical changes, never sent to
mainnet. `anchor test` applies the generated `[test.validator]` fixtures (`fund_accounts`, `mints`,
`token_accounts`) to every fork and terminates any fork a crashed test leaves behind. It refuses
`[[test.validator.account]]`, `[[test.validator.account_dir]]`, `[[test.genesis]]` and
`[test.validator] url` (`UNSUPPORTED_HISTORICAL_CONFIG`): set accounts with `setAccount`, install
programs with `installProgram`, and read history through the `[historical]` provider.
`[[test.validator.clone]]` addresses are not copied up front: reading a known address hydrates it
from the boundary. Missing history or an unsupported runtime fails: nothing
falls back to current state.
"#;
