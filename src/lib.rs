use std::{env, net::{SocketAddr, ToSocketAddrs}};

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

static CUSTOM_BOOTSTRAP: Lazy<Vec<(NetID, Checkpoint)>> = Lazy::new(|| {
    match env::var("MELBOOTSTRAP") {
        Ok(bootstrap) => bootstrap.split(',').filter_map(|s| {
            let mut split = s.splitn(2, ':');
            Some((
                split.next()?.parse::<NetID>().ok()?,
                split.next()?.parse::<Checkpoint>().ok()?,
            ))
        }).collect(),
        Err(_) => vec![],
    }
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
    let custom_bootstrap = CUSTOM_BOOTSTRAP.iter().find(|(id, _)| *id == network);
    if let Some((_, checkpoint)) = custom_bootstrap {
        return Some(checkpoint.clone());
    }

    match network {
        NetID::Mainnet => Some(
            "3210987:ad52471737a6024eca60a5b6e0ab6e699b8a653ec3851cb329bd6459e96e131c"
                .parse()
                .unwrap(),
        ),
        NetID::Testnet => Some(
            "2:37dc9fdefd1b58b332ac70bf2e78dd6aaa0980de5168be786f136d0fedf228a1"
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
