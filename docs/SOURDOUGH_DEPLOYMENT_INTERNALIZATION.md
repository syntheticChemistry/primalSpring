# sourDough Deployment Internalization Roadmap

> Contract between primalSpring (validates) and sourDough (implements tooling).
> Updated: 2026-05-14

## Purpose

plasmidBin currently owns deployment automation as shell scripts (`build-primal.sh`,
`harvest.sh`, `deploy_membrane.sh`, `validate_composition.sh`, etc.). These
patterns work but are fragile, lack type safety, and cannot be validated by
primalSpring's Rust-tier scenarios.

sourDough's mission is to absorb these patterns into a typed Rust CLI so that
every primal onboarding and deployment step is:

1. **Reproducible** — deterministic builds, checksums, composition graphs
2. **Validatable** — primalSpring can import sourDough crates and test them
3. **Composable** — new primals onboard via `sourdough scaffold`, not copy-paste

## Pattern → Subcommand Mapping

| Pattern | Current Owner | sourDough Target | Phase |
|---|---|---|---|
| `build-primal.sh --all` | plasmidBin shell | `sourdough harvest --all` | v0.4.0 |
| `harvest.sh` checksum + stage | plasmidBin shell | `sourdough harvest --release` | v0.4.0 |
| Binary signing (Ed25519) | sourDough v0.2.0 (module exists, CLI unwired) | `sourdough sign` | v0.3.0 |
| ecoBin validation (static/stripped/musl) | `doctor.sh` heuristic | `sourdough validate ecobin` | v0.3.0 |
| genomeBin self-extractor | planned in whitePaper | `sourdough package` | v0.5.0 |
| Deploy + verify cycle | `deploy_membrane.sh` + manual | `sourdough deploy --target membrane` | v0.6.0 |
| BTSP relay scaffold | `sourdough-core` pattern | scaffold default in v0.2.0+ | v0.2.x |
| Hardened systemd template | `membrane/*.service` files | `sourdough scaffold --systemd` | v0.3.0 |
| Composition validation | `validate_composition.sh` | `sourdough validate composition` | v0.4.0 |
| Triple-first binary layout | `fetch.sh` + `harvest.sh` convention | `sourdough layout --triple-first` | v0.3.0 |

## Phase Roadmap

### v0.2.x (Current — scaffold + BTSP)

- BTSP relay scaffold is the default `sourdough scaffold` output
- Ed25519 signing module exists in `sourdough-core` but is not wired to CLI
- No deployment or composition awareness yet

### v0.3.0 — Sign + Validate + Scaffold

- `sourdough sign <binary>` — Ed25519 detached signatures, reads key from age-encrypted store
- `sourdough validate ecobin <path>` — checks static linking, musl, stripped, size budget
- `sourdough validate composition <name>` — replaces `validate_composition.sh` Phase 1+2
- `sourdough scaffold --systemd <primal>` — generates hardened `.service` unit from template
- `sourdough layout --triple-first` — enforces `primals/<triple>/<name>` convention

### v0.4.0 — Harvest + Release

- `sourdough harvest --all` — cross-compile all primals per `sources.toml`
- `sourdough harvest --release` — checksum, stage, tag, push to GitHub Releases
- Asset carry-forward logic (currently in `auto-harvest.yml`) moves into Rust
- `sourdough validate composition` gains Phase 3 live health probes

### v0.5.0 — Package (genomeBin)

- `sourdough package` — creates self-extracting genomeBin archives
- Embeds manifest, checksums, and signature in the archive header
- Supports offline deployment to air-gapped gates

### v0.6.0 — Deploy

- `sourdough deploy --target membrane` — full deploy+verify cycle
- Reads composition from `manifest.toml`, fetches binaries, provisions services
- Post-deploy smoke test built in (replaces `--validate` flag)
- Multi-target support: `membrane`, `gate`, `nest` topologies

## Integration with primalSpring

Once sourDough subcommands exist, primalSpring gains new validation scenarios:

- `s_sourdough_sign_verify` — round-trip sign+verify with test key
- `s_sourdough_composition_parity` — sourDough composition matches manifest.toml
- `s_sourdough_ecobin_compliance` — sourDough ecobin checks agree with doctor.sh

These scenarios import sourDough crates directly (Rust-tier validation), replacing
the current shell-exec bridge.

## Deprecation Path

| plasmidBin Script | Deprecated By | Remove After |
|---|---|---|
| `build-primal.sh` | `sourdough harvest` | v0.5.0 |
| `harvest.sh` | `sourdough harvest --release` | v0.5.0 |
| `validate_composition.sh` | `sourdough validate composition` | v0.4.0 |
| `deploy_membrane.sh` | `sourdough deploy` | v0.7.0 |
| `doctor.sh` (ecoBin checks) | `sourdough validate ecobin` | v0.4.0 |
| `fetch.sh` | `sourdough fetch` | v0.5.0 |

Shell scripts remain functional until sourDough reaches parity. Both paths are
tested in CI during the overlap window. primalSpring's `s_sourdough_*` scenarios
gate the deprecation — a shell script is only removed when its sourDough
replacement passes all validation.

## Open Questions

1. **Key management**: Should `sourdough sign` use age-encrypted keys (current pattern)
   or integrate with BearDog's BTSP credential store?
2. **CI ownership**: Does `auto-harvest.yml` call `sourdough harvest` or does sourDough
   provide its own GitHub Action?
3. **Remote deploy transport**: SSH (current) vs BTSP-tunneled commands?
