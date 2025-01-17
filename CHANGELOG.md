# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.1] - 2024-02-01

### Added

- Upgraded `polkadot-sdk` dependencies from `polkadotv1.3.0` -> `polkadotv1.6.0` (#96)

## [0.1.0] - 2023-12-21

### Added

- Fork cumulus parachain template (#11)
- Configured pallet-multisig (#13)
- Configured pallet-proxy (#20)
- Configured pallet-utility (#36)
- Integration tests (#24)
- Set up Docs (#51)
- Docs for pallet-proxy (#57)
- Docs for aura-ext (#59)
- Docs for parachain-system (#63)
- Docs for collator-selection (#63) 
- Docs for pallet-multisig (#64)
- Docs for pallet-transaction-payment (#53)
- Docs for pallet-message-queue (#58)
- Docs for weights & fees (#66)
- Docs for xcmp-queue (#73)
- Docs for balances (#72)
- Docs for xcm-executor (#77)
- Docs for pallet-xcm (#76)

### Fixed

- Fix runtime build (#40)
- Add pallet index to multisig (#45)
- Update LICENSE (#50)
- Fix CI to verify runtime builds (#46)
- Don't run Rust CI on Doc Changes (#54)
