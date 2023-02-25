use std::net::{SocketAddr, ToSocketAddrs};

use once_cell::sync::Lazy;

use melstructs::{Checkpoint, NetID};

/// DNS seed
static MAINNET_DNS_SEEDS: Lazy<Vec<SocketAddr>> = Lazy::new(|| {
    "mainnet.p2p-bootstrap.melproject.org:41814"
        .to_socket_addrs()
        .map(|iter| iter.collect::<Vec<_>>())
        .unwrap_or_default()
});

/// DNS seed
static TESTNET_DNS_SEEDS: Lazy<Vec<SocketAddr>> = Lazy::new(|| {
    "testnet.p2p-bootstrap.melproject.org:41814"
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
pub fn checkpoint_height(network: NetID) -> Option<Checkpoint> {
    match network {
        NetID::Mainnet => Some(
            "1959796:3790ffdfd3fb5e46432732a5433b18d2e9847d6e80b229f85a139c81aead75af"
                .parse()
                .unwrap(),
        ),
        NetID::Testnet => Some(
            "72:f019035a08513da87eaec538e9f685b0c4ec72013bbfccb45ecf57d89d9d0efa"
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
