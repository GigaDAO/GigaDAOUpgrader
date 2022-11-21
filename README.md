# GigaDAOUpgrader

Mainnet Program ID: `GzMvD8AGSiRhHapNsJzUMoYR3pkbCg6vPnnopaeFZE7E`

This Solana on-chain program is implemented in anchorlang framework and serves to gate control of key smart contracts
and treasury programs behind the GIGS governance token. 

### Voting

This program is capable of executing two types of governance proposals:

1) Upgrade Program
2) Set Authority

In other words, this is a program for controlling other programs. The governance process is straight forward.

1) A tokenholder calls `propose`, specifying the type of proposal and its parameters.
2) Other tokenholders `cast_ballot`, thereby locking GIGS into that proposal.
3) Any voter can call `close_ballot` at any time to retrieve their GIGS.
4) If a proposal reaches the approval threshold of 110M GIGS, then the apt `execute` instruction can be called.

### Upgrade Authority

This contract is ***not upgradeable***. This means it is written in stone, i.e. Code Is Law.

### Verification

This program was deployed using `anchor build --verifiable`, meaning that anyone can download this repo,
build it locally (using anchor's docker verified build routine), and confirm it matches the on-chain binary.

Prerequisites:
- solana-cli v1.11.10
- anchor-cli v0.25.0
- node v18.8.0
- yarn
- docker

After installing the prerequisites, clone this repo, install with `yarn install`, followed by:

`anchor verify -p gdupgrader GzMvD8AGSiRhHapNsJzUMoYR3pkbCg6vPnnopaeFZE7E`



