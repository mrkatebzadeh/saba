# Saba Application-Aware Bandwidth Allocation Scheme

[![CI](https://github.com/mrkatebzadeh/saba-controller/workflows/CI/badge.svg)](https://github.com/mrkatebzadeh/saba-controller/actions)
[![Coverage Status](https://coveralls.io/repos/github/mrkatebzadeh/saba-controller/badge.svg?branch=main&t=KEn620)](https://coveralls.io/github/mrkatebzadeh/saba-controller?branch=main)

## Overview

- **Offline profiler** – ingests application traces to build sensitivity tables that capture how throughput responds to bandwidth for application-aware allocation.
- **Controller** – consumes the sensitivity table to allocate bandwidth across flows and programs network queues according to the Saba policy.
- **Saba library & interface** – a connection manager that applications link against to expose their intents and receive the controller’s allocation decisions.

## Workspace layout

This repository is organized as a Cargo workspace with the following members:

- `saba-core` – shared models, clustering logic, client RPC code, and protobuf bindings.
- `saba-controller` – the controller daemon that enforces Saba’s bandwidth allocation policy.
- `saba-client` – the client/interface daemon that applications run locally to talk to the controller.
- `saba-profiler` – the offline profiler that builds sensitivity tables from application traces.

## Installation

### Cargo

* Install the Rust toolchain by following the [official guide](https://www.rust-lang.org/tools/install).
* Build or install the workspace members you need:
  * `cargo install --path saba-controller`
  * `cargo install --path saba-client`
  * `cargo install --path saba-profiler`

## License

Licensed under MIT license ([LICENSE](LICENSE) or http://opensource.org/licenses/MIT)

## Cite this work

```bibtex
@inproceedings{katebzadeh_saba,
  author    = {M.R. Siavash Katebzadeh and Paolo Costa and Boris Grot},
  title     = {Saba: Rethinking Datacenter Network Allocation from Application's Perspective},
  booktitle = {Proceedings of the Eighteenth European Conference on Computer Systems},
  series    = {EuroSys '23},
  year      = {2023},
  publisher = {Association for Computing Machinery},
  address   = {New York, NY, USA},
  pages     = {623--638},
  isbn      = {9781450394871},
  doi       = {10.1145/3552326.3587450},
  url       = {https://doi.org/10.1145/3552326.3587450},
  keywords  = {datacenter networks, bandwidth allocation, max-min fairness},
  location  = {Rome, Italy}
}
```

## Contacts

This implementation is a research prototype that shows the feasibility of smart bandwidth allocation and has been tested on a cluster equipped with _Mellanox MT27700 ConnectX-4_ HCAs and a _Mellanox SX6012_ IB switch. It is NOT production quality code. If you have any questions, please raise issues on Github or contact the authors below.

[M.R. Siavash Katebzadeh](http://mr.katebzadeh.xyz) (mr@katebzadeh@xyz)
<!-- markdown-toc end -->
