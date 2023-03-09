
run-ipfs-node:
	ipfs daemon

run-ganache:
	ganache-cli -m "buffalo they brave owner merit oak buddy brown chapter program nut text"

deploy-smart-contract:
	cd smart_contract && npx hardhat run --network ganache scripts/deploy.js
