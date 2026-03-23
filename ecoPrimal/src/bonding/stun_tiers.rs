// SPDX-License-Identifier: AGPL-3.0-or-later

//! STUN tier config parsing and sovereignty-first escalation validation.
//!
//! Parses biomeOS `config/stun/multi_tier.toml` and validates the
//! sovereignty-first NAT traversal strategy:
//!   Tier 1: Genetic Lineage Relay (highest trust, family-only)
//!   Tier 2: Self-Hosted STUN (your infrastructure)
//!   Tier 3: Public STUN (address discovery only, community servers)
//!   Tier 4: Rendezvous (future, gaming platforms)

use std::path::Path;

use serde::Deserialize;

/// Parsed STUN multi-tier configuration.
#[derive(Debug, Clone)]
pub struct StunTierConfig {
    /// Whether the STUN system is enabled.
    pub enabled: bool,
    /// Escalation strategy (sovereignty-first, fastest-first, lineage-only).
    pub strategy: String,
    /// Per-tier timeout in seconds.
    pub tier_timeout_secs: u64,
    /// Tier 1: Genetic lineage relay config.
    pub lineage: LineageTier,
    /// Tier 2: Self-hosted STUN servers.
    pub user_provided: Vec<StunServer>,
    /// Tier 3: Public community STUN servers.
    pub public_stun: PublicStunTier,
    /// Tier 4: Rendezvous (future).
    pub rendezvous_enabled: bool,
    /// Advanced settings.
    pub advanced: AdvancedSettings,
}

/// Tier 1: Lineage relay configuration.
#[derive(Debug, Clone)]
pub struct LineageTier {
    /// Whether lineage relay is active.
    pub enabled: bool,
    /// Whether lineage paths take priority over faster non-lineage routes.
    pub prefer_lineage: bool,
    /// Maximum relay hops allowed through the lineage mesh.
    pub max_lineage_hops: u32,
    /// Bandwidth cap in Mbps for relay traffic.
    pub relay_bandwidth_limit_mbps: u64,
    /// Maximum simultaneous relay connections.
    pub max_concurrent_relays: u32,
}

/// A single STUN server entry (Tier 2 or Tier 3).
#[derive(Debug, Clone)]
pub struct StunServer {
    /// Host and port (e.g. `"192.168.1.144:3478"`).
    pub address: String,
    /// Transport protocol (`"udp"`, `"tcp"`, `"tls"`).
    pub protocol: String,
    /// Lower values are tried first within the same tier.
    pub priority: u32,
    /// Whether this server is currently active.
    pub enabled: bool,
    /// Whether the server identity has been cryptographically verified.
    pub verified: bool,
    /// Whether a human has audited this server for sovereignty compliance.
    pub vetted: bool,
    /// Free-form annotation for operational context.
    pub comment: String,
}

/// Tier 3: Public STUN configuration.
#[derive(Debug, Clone)]
pub struct PublicStunTier {
    /// Whether public STUN is available at all.
    pub enabled: bool,
    /// If true, only used when Tiers 1-2 fail.
    pub use_as_fallback_only: bool,
    /// Cycle through servers to avoid fingerprinting.
    pub rotate_servers: bool,
    /// Community STUN server list.
    pub servers: Vec<StunServer>,
}

/// Advanced NAT traversal settings.
#[derive(Debug, Clone)]
pub struct AdvancedSettings {
    /// Try multiple tiers simultaneously (faster, less sovereign).
    pub parallel_attempts: bool,
    /// Auto-switch to direct connection when NAT is resolved.
    pub auto_upgrade_to_direct: bool,
    /// Privacy-related settings for relay and metadata handling.
    pub privacy: PrivacySettings,
}

/// Privacy settings for sovereignty-first relay behavior.
#[derive(Debug, Clone)]
pub struct PrivacySettings {
    /// Require Dark Forest gating before relay handshake.
    pub dark_forest_gating: bool,
    /// Strip non-essential metadata from relay headers.
    pub minimal_metadata: bool,
}

// --- Raw TOML structs for deserialization ---

#[derive(Debug, Deserialize)]
struct RawStunConfig {
    #[serde(default)]
    general: RawGeneral,
    #[serde(default)]
    lineage: RawLineage,
    #[serde(default)]
    user_provided: Vec<RawServer>,
    #[serde(default)]
    public_stun: RawPublicStun,
    #[serde(default)]
    rendezvous: RawRendezvous,
    #[serde(default)]
    advanced: RawAdvanced,
}

#[derive(Debug, Default, Deserialize)]
struct RawGeneral {
    #[serde(default)]
    enabled: bool,
    #[serde(default)]
    strategy: Option<String>,
    #[serde(default)]
    tier_timeout_secs: Option<u64>,
}

#[derive(Debug, Default, Deserialize)]
struct RawLineage {
    #[serde(default)]
    enabled: bool,
    #[serde(default)]
    prefer_lineage: bool,
    #[serde(default)]
    max_lineage_hops: Option<u32>,
    #[serde(default)]
    relay_bandwidth_limit_mbps: Option<u64>,
    #[serde(default)]
    max_concurrent_relays: Option<u32>,
}

#[derive(Debug, Default, Deserialize)]
struct RawServer {
    #[serde(default)]
    address: String,
    #[serde(default)]
    protocol: String,
    #[serde(default)]
    priority: u32,
    #[serde(default)]
    enabled: bool,
    #[serde(default)]
    verified: bool,
    #[serde(default)]
    vetted: bool,
    #[serde(default)]
    comment: String,
}

#[derive(Debug, Default, Deserialize)]
struct RawPublicStun {
    #[serde(default)]
    enabled: bool,
    #[serde(default)]
    use_as_fallback_only: bool,
    #[serde(default)]
    rotate_servers: bool,
    #[serde(default)]
    servers: Vec<RawServer>,
}

#[derive(Debug, Default, Deserialize)]
struct RawRendezvous {
    #[serde(default)]
    enabled: bool,
}

#[derive(Debug, Default, Deserialize)]
struct RawAdvanced {
    #[serde(default)]
    parallel_attempts: bool,
    #[serde(default)]
    auto_upgrade_to_direct: bool,
    #[serde(default)]
    privacy: RawPrivacy,
}

#[derive(Debug, Default, Deserialize)]
struct RawPrivacy {
    #[serde(default)]
    use_dark_forest_gating: bool,
    #[serde(default)]
    minimal_metadata: bool,
}

fn convert_server(raw: &RawServer) -> StunServer {
    StunServer {
        address: raw.address.clone(),
        protocol: raw.protocol.clone(),
        priority: raw.priority,
        enabled: raw.enabled,
        verified: raw.verified,
        vetted: raw.vetted,
        comment: raw.comment.clone(),
    }
}

/// Load and parse the STUN multi-tier config from a TOML file.
///
/// # Errors
///
/// Returns a description if reading or parsing fails.
pub fn load_stun_config(path: &Path) -> Result<StunTierConfig, String> {
    let contents = std::fs::read_to_string(path)
        .map_err(|e| format!("failed to read {}: {e}", path.display()))?;
    let raw: RawStunConfig = toml::from_str(&contents)
        .map_err(|e| format!("failed to parse {}: {e}", path.display()))?;

    Ok(StunTierConfig {
        enabled: raw.general.enabled,
        strategy: raw
            .general
            .strategy
            .unwrap_or_else(|| "sovereignty-first".to_owned()),
        tier_timeout_secs: raw.general.tier_timeout_secs.unwrap_or(5),
        lineage: LineageTier {
            enabled: raw.lineage.enabled,
            prefer_lineage: raw.lineage.prefer_lineage,
            max_lineage_hops: raw.lineage.max_lineage_hops.unwrap_or(3),
            relay_bandwidth_limit_mbps: raw.lineage.relay_bandwidth_limit_mbps.unwrap_or(100),
            max_concurrent_relays: raw.lineage.max_concurrent_relays.unwrap_or(10),
        },
        user_provided: raw.user_provided.iter().map(convert_server).collect(),
        public_stun: PublicStunTier {
            enabled: raw.public_stun.enabled,
            use_as_fallback_only: raw.public_stun.use_as_fallback_only,
            rotate_servers: raw.public_stun.rotate_servers,
            servers: raw.public_stun.servers.iter().map(convert_server).collect(),
        },
        rendezvous_enabled: raw.rendezvous.enabled,
        advanced: AdvancedSettings {
            parallel_attempts: raw.advanced.parallel_attempts,
            auto_upgrade_to_direct: raw.advanced.auto_upgrade_to_direct,
            privacy: PrivacySettings {
                dark_forest_gating: raw.advanced.privacy.use_dark_forest_gating,
                minimal_metadata: raw.advanced.privacy.minimal_metadata,
            },
        },
    })
}

/// Validate the sovereignty-first escalation order.
///
/// Returns a list of issues. Empty = valid.
#[must_use]
pub fn validate_sovereignty_first(config: &StunTierConfig) -> Vec<String> {
    let mut issues = Vec::new();

    if config.strategy != "sovereignty-first"
        && config.strategy != "lineage-only"
        && !config.public_stun.use_as_fallback_only
    {
        issues.push("public STUN should be fallback-only in sovereignty-first strategy".to_owned());
    }

    if config.strategy == "sovereignty-first" {
        if !config.lineage.enabled {
            issues.push("sovereignty-first requires lineage relay (Tier 1) enabled".to_owned());
        }
        if !config.lineage.prefer_lineage {
            issues.push("sovereignty-first requires prefer_lineage = true".to_owned());
        }
        if config.advanced.parallel_attempts {
            issues.push(
                "sovereignty-first should use sequential attempts (parallel_attempts = false)"
                    .to_owned(),
            );
        }
    }

    if config.strategy == "lineage-only" && config.public_stun.enabled {
        issues.push("lineage-only strategy should not enable public STUN".to_owned());
    }

    for server in &config.user_provided {
        if server.enabled && !server.verified {
            issues.push(format!(
                "Tier 2 server {} is enabled but not verified",
                server.address
            ));
        }
    }

    let corporate_patterns = ["google", "cloudflare", "twilio"];
    for server in &config.public_stun.servers {
        if server.enabled {
            let addr_lower = server.address.to_lowercase();
            for corp in &corporate_patterns {
                if addr_lower.contains(corp) {
                    issues.push(format!(
                        "Tier 3 server {} appears to be corporate ({corp}) — sovereignty-first prefers community servers",
                        server.address
                    ));
                }
            }
        }
    }

    if !config.advanced.privacy.dark_forest_gating {
        issues.push("Dark Forest gating should be enabled for relay security".to_owned());
    }

    issues
}

/// Return the ordered tier escalation path based on the strategy.
#[must_use]
pub fn escalation_order(config: &StunTierConfig) -> Vec<&'static str> {
    match config.strategy.as_str() {
        "lineage-only" => vec!["Tier 1: Lineage Relay"],
        "fastest-first" => vec![
            "Tier 3: Public STUN",
            "Tier 2: Self-Hosted STUN",
            "Tier 1: Lineage Relay",
            "Tier 4: Rendezvous",
        ],
        _ => {
            let mut tiers = Vec::new();
            if config.lineage.enabled {
                tiers.push("Tier 1: Lineage Relay");
            }
            if !config.user_provided.is_empty() {
                tiers.push("Tier 2: Self-Hosted STUN");
            }
            if config.public_stun.enabled {
                tiers.push("Tier 3: Public STUN");
            }
            if config.rendezvous_enabled {
                tiers.push("Tier 4: Rendezvous");
            }
            tiers
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    fn write_temp(content: &str) -> tempfile::NamedTempFile {
        let mut f = tempfile::NamedTempFile::new().unwrap();
        f.write_all(content.as_bytes()).unwrap();
        f
    }

    fn default_privacy() -> PrivacySettings {
        PrivacySettings {
            dark_forest_gating: true,
            minimal_metadata: true,
        }
    }

    fn default_advanced() -> AdvancedSettings {
        AdvancedSettings {
            parallel_attempts: false,
            auto_upgrade_to_direct: true,
            privacy: default_privacy(),
        }
    }

    fn default_lineage() -> LineageTier {
        LineageTier {
            enabled: true,
            prefer_lineage: true,
            max_lineage_hops: 3,
            relay_bandwidth_limit_mbps: 100,
            max_concurrent_relays: 10,
        }
    }

    fn empty_public_stun(enabled: bool) -> PublicStunTier {
        PublicStunTier {
            enabled,
            use_as_fallback_only: true,
            rotate_servers: false,
            servers: Vec::new(),
        }
    }

    #[test]
    fn parse_minimal_config() {
        let f = write_temp(
            r#"
[general]
enabled = true
strategy = "sovereignty-first"
tier_timeout_secs = 5

[lineage]
enabled = true
prefer_lineage = true

[public_stun]
enabled = true
use_as_fallback_only = true

[rendezvous]
enabled = false

[advanced]
parallel_attempts = false
[advanced.privacy]
use_dark_forest_gating = true
minimal_metadata = true
"#,
        );
        let config = load_stun_config(f.path()).unwrap();
        assert!(config.enabled);
        assert_eq!(config.strategy, "sovereignty-first");
        assert!(config.lineage.enabled);
        assert!(config.lineage.prefer_lineage);
        assert!(!config.advanced.parallel_attempts);
    }

    #[test]
    fn sovereignty_first_validates_clean() {
        let config = StunTierConfig {
            enabled: true,
            strategy: "sovereignty-first".to_owned(),
            tier_timeout_secs: 5,
            lineage: default_lineage(),
            user_provided: vec![StunServer {
                address: "192.168.1.144:3478".to_owned(),
                protocol: "udp".to_owned(),
                priority: 1,
                enabled: true,
                verified: true,
                vetted: false,
                comment: "Tower LAN".to_owned(),
            }],
            public_stun: PublicStunTier {
                enabled: true,
                use_as_fallback_only: true,
                rotate_servers: true,
                servers: vec![StunServer {
                    address: "stun.nextcloud.com:3478".to_owned(),
                    protocol: "udp".to_owned(),
                    priority: 1,
                    enabled: true,
                    verified: false,
                    vetted: true,
                    comment: "Nextcloud community".to_owned(),
                }],
            },
            rendezvous_enabled: false,
            advanced: default_advanced(),
        };
        let issues = validate_sovereignty_first(&config);
        assert!(issues.is_empty(), "issues: {issues:?}");
    }

    #[test]
    fn catches_parallel_in_sovereignty_first() {
        let config = StunTierConfig {
            enabled: true,
            strategy: "sovereignty-first".to_owned(),
            tier_timeout_secs: 5,
            lineage: default_lineage(),
            user_provided: Vec::new(),
            public_stun: empty_public_stun(false),
            rendezvous_enabled: false,
            advanced: AdvancedSettings {
                parallel_attempts: true,
                ..default_advanced()
            },
        };
        let issues = validate_sovereignty_first(&config);
        assert!(issues.iter().any(|i| i.contains("parallel")));
    }

    #[test]
    fn catches_corporate_stun() {
        let config = StunTierConfig {
            enabled: true,
            strategy: "sovereignty-first".to_owned(),
            tier_timeout_secs: 5,
            lineage: default_lineage(),
            user_provided: Vec::new(),
            public_stun: PublicStunTier {
                enabled: true,
                use_as_fallback_only: true,
                rotate_servers: false,
                servers: vec![StunServer {
                    address: "stun.google.com:19302".to_owned(),
                    protocol: "udp".to_owned(),
                    priority: 1,
                    enabled: true,
                    verified: false,
                    vetted: false,
                    comment: "Google STUN".to_owned(),
                }],
            },
            rendezvous_enabled: false,
            advanced: default_advanced(),
        };
        let issues = validate_sovereignty_first(&config);
        assert!(issues.iter().any(|i| i.contains("google")));
    }

    #[test]
    fn escalation_order_sovereignty_first() {
        let config = StunTierConfig {
            enabled: true,
            strategy: "sovereignty-first".to_owned(),
            tier_timeout_secs: 5,
            lineage: default_lineage(),
            user_provided: vec![StunServer {
                address: "192.168.1.144:3478".to_owned(),
                protocol: "udp".to_owned(),
                priority: 1,
                enabled: true,
                verified: true,
                vetted: false,
                comment: "Tower".to_owned(),
            }],
            public_stun: PublicStunTier {
                enabled: true,
                use_as_fallback_only: true,
                rotate_servers: false,
                servers: Vec::new(),
            },
            rendezvous_enabled: false,
            advanced: default_advanced(),
        };
        let order = escalation_order(&config);
        assert_eq!(
            order,
            vec![
                "Tier 1: Lineage Relay",
                "Tier 2: Self-Hosted STUN",
                "Tier 3: Public STUN",
            ]
        );
    }

    #[test]
    fn escalation_order_lineage_only() {
        let config = StunTierConfig {
            enabled: true,
            strategy: "lineage-only".to_owned(),
            tier_timeout_secs: 5,
            lineage: default_lineage(),
            user_provided: Vec::new(),
            public_stun: empty_public_stun(false),
            rendezvous_enabled: false,
            advanced: default_advanced(),
        };
        let order = escalation_order(&config);
        assert_eq!(order, vec!["Tier 1: Lineage Relay"]);
    }
}
