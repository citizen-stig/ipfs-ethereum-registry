require("@nomicfoundation/hardhat-toolbox");

/** @type import('hardhat/config').HardhatUserConfig */
module.exports = {
    defaultNetwork: "goerli",
    networks: {
        hardhat: {},
        ganache: {
            url: "http://127.0.0.1:8545",
            accounts: [
                "0xc95accbb87ea1c37aa0e7abce7a01d8121125734788843a1662a79219cce8fce",
                "0xa41bcd45af001dc30d6bc20fa03c2480de6b1779ed06456ba222602d7ab1983a",
                "0x94f7f96595e1abae9d5d1a3f505698a5666e86ce7c2146ae58b85e219a4ce120",
                "0x56df3c60b0445520c49962b9b31e3fbb252a19a817fa8156544d7ae0483ef015",
                "0x05e09ad9bb47e0fb3dc2306ecbb2438f82838b5b38253f6ffd2e0af2673976f4",
                "0x81a19f0ae7e7d9de5d59dac1b924f4eb47aa641f932c6f456eabd9ade7123d67",
                "0x6f38e5f1e2f5d84e8255a39c340064664adfaa3adb627da505c417f6990bff56",
                "0xb453cd5e4302766ce039fe7e5c6377aae1330053f5122a2d13e5f8ab1213de57",
                "0xa879884b5144ddd01dd39262bc21da2b087914ba8ce0e26982281ca56326e211",
                "0x1456697aedd53547909ece17479dd47ff6a30c34f1a186e31c9dd9676997c250"
            ]
        }
    },
    solidity: "0.8.18",
};
