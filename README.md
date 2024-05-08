[![](https://img.shields.io/crates/v/melbootstrap)](https://crates.io/crates/melbootstrap)
![](https://img.shields.io/crates/l/melbootstrap)
The `melbootstrap` crate contains bootstrap information for Mel clients. In particular, it provides

- Hardcoded **bootstrap nodes** for joining the replica peer-to-peer network
- Hardcoded **checkpoints**: known block hashes / heights for both the testnet and mainnet. This is required for correct functionality due to ["weak subjectivity"](https://blog.ethereum.org/2014/11/25/proof-stake-learned-love-weak-subjectivity/). In short, without a somewhat recent (within 70 days / 200,000 blocks) known block height, blocks cannot be validated in a trustworthy manner.
