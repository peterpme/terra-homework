import { client, wallets } from "../library.js";

let address = wallets.wallet1.key.accAddress;
const [balance] = await client.bank.balance(address);
