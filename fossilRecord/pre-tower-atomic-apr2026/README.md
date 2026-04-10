# Pre-Tower-Atomic Deployment Artifacts (April 2026)

Archived during the Zero Port Tower Atomic transition. These files contain
TCP port defaults, `--port 0` arguments, and TCP-first transport modes that
predate the UDS-only Tower Atomic standard.

## What changed

- All primals now run UDS-only by default; TCP is opt-in via explicit `--port`
- `nucleus_launcher.sh` no longer passes `--port` to any primal
- `primal_launch_profiles.toml` no longer includes `extra_args = ["--port", "0"]`
- `deployment_matrix.toml` TCP-first topologies deprecated

## Files

- `nucleus_launcher.sh` -- had `--port 9200` for Songbird, missing provenance trio
- `primal_launch_profiles.toml` -- had `extra_args = ["--port", "0"]` for Songbird/ToadStool
- `deployment_matrix.toml` -- had `tower_tcp_first` as active topology
- `validate_composition.sh` -- TCP-era composition validation wrapper

See the modern versions in their original locations.
