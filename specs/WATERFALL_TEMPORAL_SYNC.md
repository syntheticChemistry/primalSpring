# waterFall — Temporal Sync Specification

**Owner**: primalSpring (spec) + biomeOS (runtime) + wateringHole (infrastructure)
**Status**: Specification — evolving from bash to Neural API
**Date**: May 29, 2026 (Wave 60)

---

## Problem Statement

The WaterFall pattern today is spatially correct but temporally naive. `--source auto`
selects a remote based on availability (Forgejo configured → use it), not on which
remote has the most recent commits. The result: a pull from a stale Forgejo mirror
returns zero changes while GitHub holds 4+ commits of upstream primal evolution.

The ecosystem needs **temporal sync**: a model where each gate independently discovers
which remote is ahead, pulls from the leader, and propagates to followers — without
centralized coordination, without ordering requirements, and even without connectivity.

---

## Sync Levels

Innermost to outermost. Each level is a complete sync boundary — higher levels
compose lower ones but never require them.

| Level | Scope | Boundary | Example |
|-------|-------|----------|---------|
| **local** | Working tree ↔ HEAD | Single repo | `git stash`, dirty check, uncommitted work |
| **gate** | All repos on one workstation | Filesystem | `cascade-pull` within eastGate |
| **membrane** | Gate ↔ VPS depot | SSH/HTTPS | `git push forgejo`, `git pull forgejo` |
| **ecosystem** | All membranes ↔ all remotes | Network + DAG | GitHub + Forgejo + future depots + USB |

A gate-level sync triggers N local-level syncs (one per repo). A membrane-level
sync is a gate-level sync followed by a push to the depot. An ecosystem-level sync
reconciles all known remotes.

---

## Temporal Patterns

For any repo, each remote has a HEAD. The relationship between local HEAD and
remote HEADs determines the sync pattern.

### converge

One remote is ahead. Others are behind or at parity.

```
origin/main    a1b2c3 ← leader (4 commits ahead)
forgejo/main   d4e5f6 ← follower (stale)
HEAD           d4e5f6 ← follower (matches forgejo)

Action: pull origin → push forgejo
Result: all three at a1b2c3
```

Safe to automate. No human decision needed. The DAG is linear between
the two positions.

### diverge

Multiple remotes have unique commits not present in the other.

```
origin/main    a1b2c3 (has commits X, Y)
forgejo/main   g7h8i9 (has commits P, Q)
HEAD           d4e5f6 (behind both)

Action: FLAG for quorumSignal review
```

Divergence means someone pushed different work to different remotes.
This is a merge decision, not an automation target. waterFall reports
the divergence; quorumSignal (or the human) resolves it.

### parity

All remotes at the same HEAD.

```
origin/main    a1b2c3
forgejo/main   a1b2c3
HEAD           a1b2c3

Action: no-op
```

Report clean. Update freshness timestamp.

---

## Temporal Measurement

Git refs are the temporal substrate. waterFall never needs wall-clock time,
logical clocks, or vector timestamps. The DAG contains all ordering
information.

For each repo × remote pair:

```
commits_ahead  = rev-list --count remote/main..HEAD
commits_behind = rev-list --count HEAD..remote/main
```

The **temporal position matrix** for a repo:

| Remote | Ahead | Behind | Pattern |
|--------|------:|-------:|---------|
| origin | 0 | 4 | behind — origin leads |
| forgejo | 0 | 0 | parity |

If any remote has both `ahead > 0` AND another remote has unique
commits → `diverge`. Otherwise the remote with the highest
`commits_ahead` relative to local is the leader.

---

## The Triad Cycle

The three Neural API domains operate as a continuous cycle:

```
  ┌──────────────────────────────────────────────────┐
  │                                                  │
  │   rP (action)                                    │
  │   Teams commit to nearest membrane               │
  │   rhizoCrypt → loamSpine → sweetGrass            │
  │   git commit + git push $nearest                 │
  │                                                  │
  ├──────────────────────────────────────────────────┤
  │                                                  │
  │   qS (sense)                                     │
  │   Gate fetches all remotes                       │
  │   Measures temporal position matrix              │
  │   Classifies: converge / diverge / parity        │
  │   Reports drift per repo                         │
  │                                                  │
  ├──────────────────────────────────────────────────┤
  │                                                  │
  │   wF (sync)                                      │
  │   Pulls from leader remote                       │
  │   Pushes to follower remotes                     │
  │   Updates freshness.toml                         │
  │   Publishes freshness to mesh                    │
  │                                                  │
  └──────────────────────────────────────────────────┘
```

No subsystem calls the others directly. They share one common language:
**git refs as temporal markers**. qS reads them, rP writes them, wF
propagates them. The DAG is the coordination substrate.

---

## Wave Model

There is no single sync event that resolves everything. Waves pass through
the ecosystem asynchronously:

```
Wave N    ironGate rP: rhizoCrypt team pushes dag.branch to GitHub
Wave N+1  eastGate qS: fetch all → origin +4, forgejo stale
Wave N+2  eastGate wF: pull origin → push forgejo (converge)
Wave N+3  southGate qS: fetch all → forgejo now current (parity)
Wave N+4  southGate wF: no-op
```

Each gate runs its own waves independently. No coordinator. No ordering.
A gate offline for a week comes back, runs one wave, and converges with
everything that happened. The wave period can be:

| Cadence | Use case |
|---------|----------|
| 15 min (systemd timer) | Active development gate |
| On-demand (manual) | Intermittent gates |
| Daily | Cold storage / NAS gates |
| On-connect (USB insert) | Airgapped sneakernet |

---

## Airgap Support

The temporal model is transport-agnostic. A USB drive with a bare repo
is just another remote:

```bash
git remote add usb file:///media/usb/ecoPrimals/primals/rhizoCrypt.git
```

qS senses it, wF syncs from it. The `converge` pattern works identically
whether the remote is:

- `ssh://git.primals.eco:2222/ecoPrimals/repo.git` (Forgejo)
- `https://github.com/ecoPrimals/repo.git` (GitHub)
- `file:///media/usb/repo.git` (sneakernet)
- `git://192.168.1.50/repo.git` (LAN depot)

A depot remote (NAS, VPS, USB) participates in temporal sync by being
git-reachable. No special protocol needed.

---

## Remote Topology

Each repo can have N remotes. The `ecosystem_manifest.toml` declares the
canonical remotes (origin, forgejo). Gates may add local remotes (usb,
lan-depot, project-vps).

### Source Priority for `--source temporal`

1. **Fetch all remotes** — `git fetch --all`
2. **Build temporal position matrix** — ahead/behind counts per remote
3. **Classify pattern** — converge / diverge / parity
4. **If converge**: pull from the remote with the highest ahead count
5. **If diverge**: skip, report to quorumSignal
6. **If parity**: no-op, update freshness
7. **Push to followers** — any remote behind local HEAD after pull

This replaces `--source auto` (which means "prefer forgejo remote name
if configured") with true temporal intelligence.

### Manifest Extensions

```toml
[sync]
default_source = "temporal"    # was "github"
freshness_cadence = "15min"
divergence_policy = "flag"     # flag | merge-ff | merge-rebase

[sync.remotes.forgejo]
url_template = "ssh://git.primals.eco:2222/ecoPrimals/{repo}.git"
priority = "membrane"          # membrane | ecosystem | depot

[sync.remotes.origin]
url_template = "https://github.com/ecoPrimals/{repo}.git"
priority = "ecosystem"

# Future: per-project depot
[sync.remotes.helixvision-depot]
url_template = "ssh://helix-vps.example.com:2222/repos/{repo}.git"
priority = "depot"
repos = ["gardens/helixVision"]
```

---

## Implementation Path

### Phase 1: Temporal cascade-pull (DONE → `membrane temporal.cascade`)

Evolve the bash script:

- `--source temporal` mode: fetch all → measure → pull leader → push followers
- Temporal position matrix output in `--check` mode
- `--publish-freshness` writes `freshness.toml` with per-repo per-remote HEADs
- Divergence detection and human-readable report

### Phase 2: Freshness Mesh (Wave 62+)

- `freshness.toml` evolves to include per-remote HEAD snapshots
- songbird `mesh.publish` broadcasts gate freshness to mesh
- Other gates can check freshness without fetching (O(1) drift detection)

### Phase 3: Neural API Ecosystem Signals (Wave 63+)

- `ecosystem.pull` signal graph calls `content.sync` with temporal source selection
- `ecosystem.check` calls `content.fetch_heads` + temporal position matrix
- `ecosystem.push` calls `content.push` to all follower remotes
- biomeOS replaces bash with `signal.dispatch("ecosystem.pull")`

### Phase 4: Cross-Gate Temporal Coordination (Wave 65+)

- Gate A can trigger `ecosystem.check` on Gate B via cross-gate executor
- Fleet-wide freshness dashboard from mesh-published snapshots
- Adaptive sync cadence: gates with high drift sync more frequently

---

## Relationship to Existing Specs

| Spec | Relationship |
|------|-------------|
| `NEURAL_API_EVOLUTION.md` | waterFall is the SYNC domain of the coordination triad |
| `CROSS_GATE_GRAPH_EXECUTOR.md` | Phase 4 temporal coordination requires cross-gate `graph.execute` |
| `ecosystem_manifest.toml` | SSOT for repo metadata, gate profiles, and remote URLs |
| `ecosystem_pull.toml` | Signal graph for the pull operation — needs temporal source logic |
| `membrane temporal.cascade` | Rust implementation (replaces bash `cascade-pull.sh`, fossilized Wave 66) |
| `freshness.toml` | Wave snapshot — evolves to per-remote temporal state |

---

*The ecosystem does not need a coordinator. It needs temporal awareness.
Each gate independently discovers what changed, pulls the leader, and
pushes to followers. The DAG is the only clock that matters.*
