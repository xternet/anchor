# Unofficial historical testing preview

This branch contains the combined SVM Replay integration, not an upstream
Anchor release. Proposed upstream PRs remain separate and unmerged.

The current source adds credential-safe confirmation diagnostics, private fork
identity files and one disposable default signing key per fork. Funding remains
explicit. It also handles closing providers that never opened a WebSocket on
older supported clients. Existing fixture support and normal testing remain.
Native Windows historical helpers remain unsupported; use WSL2.

[Earlier snapshot downloads](https://github.com/xternet/anchor/releases/tag/svm-replay-0.2.0-preview.2)
are pinned to that tag and do not include these latest source changes. Use this
branch for source review/builds; updated binary publication is separate.

Requires [SVM Replay 0.2.0](https://github.com/xternet/svm-replay/releases/tag/v0.2.0)
and the matching unofficial Surfpool preview. Missing historical data fails
explicitly; no silent current-state fallback is used.

Never deliberately use a real funded wallet at a recent historical boundary:
leaked signed bytes may remain valid on mainnet until their blockhash expires.
The fork does not submit transactions to mainnet.
