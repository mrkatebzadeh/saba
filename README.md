-------------------------------------------------------------------------------

# SaBA: Smart Bandwidth Allocator

[![CI](https://github.com/mrkatebzadeh/Saba/workflows/CI/badge.svg)](https://github.com/mrkatebzadeh/Saba/actions)
-------------------------------------------------------------------------------

**Table of Contents**

- [SaBA](#saba)
- [Prerequisites](#prerequisites)
- [Install](#install)
- [Configuration](#configuration)
- [Running Tests](#running-tests)
- [Contacts](#contacts)

## SaBA ##

This package provides a smart/application-aware bandwidth allocator and consists of following components:
- saba_ibverbs
- saba_socket
- Controller
- test


## Prerequisites ##

Before you install SaBA, you must have the following libraries:

- cmake
- rdma-core libibverbs1 librdmacm1 libibmad5 libibumad3 librdmacm1 ibverbs-providers rdmacm-utils infiniband-diags libfabric1 ibverbs-utils libibverbs-dev

## Install ##

Clone the repository:
```
git clone https://github.com/ease-lab/Saba.git
```
Then you can simply make the package:
```
cd Saba
cmake -S . -B build
cmake --build build
```
To prevent any component from being compiled, use `SABA_NO_X=YES`, where **X** is the name of component.
```
cd Saba
SABA_NO_IBVERBS=YES cmake -S . -B build
cmake --build build
```

## Configuration ##

**TODO**

## Running Tests ##
**TODO**

## Contacts ##

This implementation is a research prototype that shows the feasibility of smart bandwidth allocation and has been tested on a cluster equipped with _Mellanox MT27700 ConnectX-4_ HCAs and a _Mellanox SX6012_ IB switch. It is NOT production quality code. If you have any questions, please raise issues on Github or contact the authors below.

[M.R. Siavash Katebzadeh](http://mr.katebzadeh.xyz) (m.r.katebzadeh@ed.ac.uk)
<!-- markdown-toc end -->

 

