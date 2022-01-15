use std::net::{SocketAddr, ToSocketAddrs};

use once_cell::sync::Lazy;
use themelio_nodeprot::TrustedHeight;
use themelio_structs::NetID;

/// DNS seed
static MAINNET_DNS_SEEDS: Lazy<Vec<SocketAddr>> = Lazy::new(|| {
    "mainnet-bootstrap.themelio.org:11814"
        .to_socket_addrs()
        .map(|iter| iter.collect::<Vec<_>>())
        .unwrap_or_default()
});

/// DNS seed
static TESTNET_DNS_SEEDS: Lazy<Vec<SocketAddr>> = Lazy::new(|| {
    "testnet-bootstrap.themelio.org:11814"
        .to_socket_addrs()
        .map(|iter| iter.collect::<Vec<_>>())
        .unwrap_or_default()
});

/// Obtains bootstrap nodes for a given NetID.
pub fn bootstrap_routes(network: NetID) -> Vec<SocketAddr> {
    match network {
        NetID::Mainnet => MAINNET_DNS_SEEDS.clone(),
        NetID::Testnet => TESTNET_DNS_SEEDS.clone(),
        _ => vec![],
    }
}

/// Obtains a checkpoint for a given NetID.
pub fn checkpoint_height(network: NetID) -> Option<TrustedHeight> {
    match network {
        NetID::Mainnet => Some(
            "413096:7ecd81b20ab0ce678b9de7078b833f41d23856df5323a93abd409149b23a4bcd"
                .parse()
                .unwrap(),
        ),
        NetID::Testnet => Some(
            "400167:bf8a7194dcef69eb3a0c9a3664d58156f68ca4092306ce04eda08bfe794db940"
                .parse()
                .unwrap(),
        ),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_bootstrap() {
        assert!(!bootstrap_routes(NetID::Mainnet).is_empty());
        assert!(!bootstrap_routes(NetID::Testnet).is_empty());
    }
}
