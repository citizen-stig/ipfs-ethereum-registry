const hre = require("hardhat");
const fs = require('fs');

async function main() {
    console.log("Going to wait for 'FileRegistered events");
    const FileRegistry = await hre.ethers.getContractFactory("FileRegistry");
    const contractAddress = fs.readFileSync('smart_contract_address.txt', 'utf8');
    const fileRegistry = await FileRegistry.attach(contractAddress);
    fileRegistry.on("FileRegistered", (contentId, owner) => {
        console.log("File Registered", contentId, owner);
    })
}

main();


