[![](https://img.shields.io/crates/v/themelio-bootstrap)](https://crates.io/crates/themelio-bootstrap)
![](https://img.shields.io/crates/l/themelio-bootstrap)
The `themelio-bootstrap` crate contains bootstrap information for Themelio clients. In particular, it provides

- Hardcoded **bootstrap nodes** for joining the auditor peer-to-peer network
- Hardcoded **checkpoints**: known block hashes / heights for both the testnet and mainnet. This is required for correct functionality due to ["weak subjectivity"](https://blog.ethereum.org/2014/11/25/proof-stake-learned-love-weak-subjectivity/). In short, without a somewhat recent (within 70 days / 200,000 blocks) known block height, blocks cannot be validated in a trustworthy manner.
