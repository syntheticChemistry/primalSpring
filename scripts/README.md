# scripts/ — Lab Automation

Shell scripts for multi-gate lab validation scenarios. These scripts
invoke primals in standalone mode for chaos testing and cross-architecture
validation.

**Status**: Functional but reference pre-`nucleus_launcher` invocation
patterns (standalone `beardog server`, `songbird server`). The canonical
launch path is now `nucleus_launcher` or `primalspring nucleus`.

| Script | Purpose |
|--------|---------|
| `chaos-inject.sh` | Inject network partitions and process kills into running labs |
| `lan_covalent_lab.sh` | 2-gate LAN mesh validation (covalent bond proving) |
| `pixel_cross_arch_lab.sh` | aarch64 cross-architecture deployment to Pixel |
| `validate_composition.sh` | Run experiment suite against live compositions |
| `validate_local_lab.sh` | Local-gate full validation sweep |

These scripts are preserved as lab tooling. When multi-gate automated
testing resumes, they'll be evolved to use `nucleus_launcher` patterns.
