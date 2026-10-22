# Unofficial historical testing preview

This branch contains the combined SVM Replay integration, not an upstream
Anchor release. Proposed upstream PRs remain separate and unmerged.

Revision 2 adds account-file/directory/genesis fixtures, startup image checks
and fork-local deployment setup. It preserves the earlier preview's other
features. Native Windows historical helpers remain unsupported; use WSL2.

[Downloads and tested installation instructions](https://github.com/xternet/anchor/releases/tag/svm-replay-0.2.0-preview.2)

Requires [SVM Replay 0.2.0](https://github.com/xternet/svm-replay/releases/tag/v0.2.0)
and the matching unofficial Surfpool preview. Missing historical data fails
explicitly; no silent current-state fallback is used.
