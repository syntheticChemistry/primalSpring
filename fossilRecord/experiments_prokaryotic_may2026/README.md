# Prokaryotic Experiment Fossil Record

Snapshot of 20 absorbed experiment crate sources — the prokaryotic era
predecessors to the eukaryotic UniBin scenario modules.

Date: May 2026 (Interstadial transition)

## Absorbed Experiments

| Experiment | Scenario Module | Track |
|-----------|----------------|-------|
| exp001_tower_atomic | s_tower_atomic | atomic-composition |
| exp002_node_atomic | s_node_atomic | atomic-composition |
| exp003_nest_atomic | s_nest_atomic | atomic-composition |
| exp004_full_nucleus | s_full_nucleus | atomic-composition |
| exp006_startup_ordering | s_startup_ordering | atomic-composition |
| exp010_sequential_graph | s_sequential_graph | graph-execution |
| exp030_covalent_bond | s_covalent_bond | bonding |
| exp031_ionic_bond | s_ionic_bond | bonding |
| exp033_gate_failure | s_gate_failure | security |
| exp040_cross_spring_data_flow | s_cross_spring_data_flow | cross-spring |
| exp050_compute_triangle | s_compute_triangle | transport |
| exp051_socket_discovery_sweep | s_socket_discovery | transport |
| exp054_bearer_token_auth | s_bearer_token_auth | security |
| exp060_biomeos_tower_deploy | s_biomeos_tower_deploy | biomeos-deploy |
| exp075_biomeos_neural_api_live | s_biomeos_neural_api | biomeos-deploy |
| exp081_deployment_matrix_sweep | s_deployment_matrix | infrastructure |
| exp094_composition_parity | s_composition_parity | lifecycle |
| exp098_cellular_deployment | s_cellular_deployment | infrastructure |
| exp108_token_federation | s_token_federation | security |
| exp109_composition_lifecycle | s_composition_lifecycle | lifecycle |

## Context

These experiment crates were standalone binaries that validated specific
NUCLEUS composition patterns. Their validation logic has been absorbed into
`ecoPrimal/src/validation/scenarios/` as library modules, callable from the
UniBin's `validate` subcommand.

The original experiment crates remain as workspace members during the
transition period. Full removal happens in the next stadial wave when all
remaining experiments are also absorbed.
