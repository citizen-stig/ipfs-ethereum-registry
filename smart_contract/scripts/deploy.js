const hre = require("hardhat");
const fs = require('fs');

async function main() {
    const FileRegistry = await hre.ethers.getContractFactory("FileRegistry");
    const fileRegistry = await FileRegistry.deploy();

    await fileRegistry.deployed();

    console.log(`Registry has been deployed to ${fileRegistry.address}`);
    fs.writeFile('smart_contract_address.txt', fileRegistry.address, err => {
        if (err) {
            console.error(err);
        }
    });
}

// We recommend this pattern to be able to use async/await everywhere
// and properly handle errors.
main().catch((error) => {
    console.error(error);
    process.exitCode = 1;
});
