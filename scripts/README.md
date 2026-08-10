# scripts/ — Lab Automation & Cascade

Shell scripts for multi-gate lab validation scenarios and eastGate cascade ops.

**Status**: Lab scripts reference pre-`nucleus_launcher` invocation patterns
(standalone `beardog server`, `songbird server`). The canonical launch path
is now `nucleus_launcher` or `primalspring nucleus`.

| Script | Purpose |
|--------|---------|
| `temporal_cascade.sh` | eastGate temporal cascade — depot refresh + binary deployment |
| `chaos-inject.sh` | Inject network partitions and process kills into running labs |
| `lan_covalent_lab.sh` | 2-gate LAN mesh validation (covalent bond proving) |
| `pixel_cross_arch_lab.sh` | aarch64 cross-architecture deployment to Pixel |
| `validate_composition.sh` | Run experiment suite against live compositions |
| `validate_local_lab.sh` | Local-gate full validation sweep |

Lab scripts are preserved as tooling. When multi-gate automated testing
resumes, they'll be evolved to use `nucleus_launcher` patterns.
