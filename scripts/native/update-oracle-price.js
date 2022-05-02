import { client, wallets } from "../library.js";

import {
  MsgExecuteContract,
  MnemonicKey,
  Coins,
  LCDClient,
} from "@terra-money/terra.js";

const contract = "terra140nuhszq4qhmvqgv57fsh8m9p6lw305002nvec";
const wallet = wallets.lunaHomework;

const msg = new MsgExecuteContract(
  // Address of wallet that is signing the transaction
  wallet.key.accAddress,
  // Address of CW20 contrac:
  contract,
  // ExecuteMsg payload
  {
    update_price: {
      price: 69,
    },
  }
);

const tx = await wallet.createAndSignTx({ msgs: [msg] });
const result = await client.tx.broadcast(tx);

console.log(result);
