<!-- omit in toc -->
# kubectl-df-pv

## Table of contents
- [Table of contents](#table-of-contents)
- [About The Project](#about-the-project)
- [Motivation](#motivation)
- [Testing setup](#testing-setup)
- [Getting Started](#getting-started)
  - [Prerequisites](#prerequisites)
  - [Setup](#setup)
  - [Usage](#usage)
    - [Available options](#available-options)
    - [Usage examples](#usage-examples)

## About The Project
df (disk free) command line program used to display the amount of available PV (Persistent Volume) space for Kubernetes.

## Motivation
Project was created, because kubectl command does not provide Kubernetes Persistent Volume disk usage statistics.

## Testing setup
Application was tested under below setup:
- Kubernetes cluster in version 1.22.2
- OpenEBS LVM Local PV Kubernetes storage in version 0.8.5

## Getting Started
Note: This project binary is not distributed in crates.io at the moment.

### Prerequisites
- Rust
- Cargo
- Kubernetes storage system which populates Persistent Volume metrics, for example: OpenEBS LVM Local PV

### Setup
Build and install basing on latest commit from main branch:
```bash
cargo install --git https://github.com/rsitko92/kubectl-df-pv.git
```

Build and install basing on specific commit:
```bash
cargo install --git https://github.com/rsitko92/kubectl-df-pv.git --rev 57ca198321ca2dfff30824f17e81f92c66035c55
```

### Usage

#### Available options

```bash
USAGE:
    kubectl-df-pv [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -f, --format <format>           [default: table]  [possible values: table, json]
    -n, --namespace <namespace>
```

#### Usage examples

```bash
vagrant@bullseye:/vagrant$ kubectl-df-pv
 PVC NAME   POD NAMESPACE       POD NAME   CAPACITY  USED      INODES  INODES USED  INODES FREE
 test-pvc   test-pod-namespace  test-pod   10.62MiB  204KiB    3072    11           3061
 test-pvc2  default             test-pod2  18.36MiB  10.31MiB  5136    11           5125
```

```bash
vagrant@bullseye:/vagrant$ kubectl-df-pv -n test-pod-namespace
 PVC NAME  POD NAMESPACE       POD NAME  CAPACITY  USED    INODES  INODES USED  INODES FREE
 test-pvc  test-pod-namespace  test-pod  10.62MiB  204KiB  3072    11           3061
```

```bash
vagrant@bullseye:/vagrant$ kubectl-df-pv -n test-pod-namespace -f json
[{"pvc_name":"test-pvc","pod_namespace":"test-pod-namespace","pod_name":"test-pod","capacity_bytes":11131904,"used_bytes":208896,"inodes_free":3061,"inodes":3072,"inodes_used":11}]
```
