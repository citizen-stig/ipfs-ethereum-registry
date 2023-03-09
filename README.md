# Prerequisites

1. [Docker](https://docs.docker.com/get-docker/) with compose functionality
2. [npm](https://docs.npmjs.com/cli/v7/configuring-npm/install)
3. [Rust toolchain](https://rustup.rs/)

# Preparation

1. Prepare dependencies by running `make install`
2. Start ipfs node and ganache cli by running `docker-compose up`
3. Deploy smart contract by running `make deploy-smart-contract`. Address of deployed smart contract is printed at the
   end. It will be needed at later stages

# How to run

```
 ./target/debug/registrar --path file.txt --eth-key=c95accbb87ea1c37aa0e7abce7a01d8121125734788843a1662a79219cce8fce --contract-address=0xd0520dba92cfd1fed6d9dd6bf3ecd546c65ce759
```

Different private keys are available, they can be found in the output of ganache-cli docker container.

(!) Please note, that smart contract forbids registering same file more than once, program will return error in that
case

Additionally, to check that smart contract has processed registration properly, script for monitoring events can be run
in separate console

```
make events_watcher
```

Program can be pointed to different IPFS or Blockchain urls, if these environment variables are specifies:

* `IPFS_URL`, default is "http://127.0.0.1:5001"
* `BLOCKCHAIN_URL`, default is ""http://localhost:8545"