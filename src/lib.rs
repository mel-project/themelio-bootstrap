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
            "1000388:d02bda605c12374714524b49411535a5e31e59fd7c0deba8727512dbbfaa6b2d"
                .parse()
                .unwrap(),
        ),
        NetID::Testnet => Some(
            "3737:0ef0a61a67fb56f9b05e38a90b4a9f2d565963ee1e1b749f84bbc43c02e9f306"
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
