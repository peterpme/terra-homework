import { client, wallets } from "../library.js";

import {
  MsgExecuteContract,
  MnemonicKey,
  Coins,
  LCDClient,
} from "@terra-money/terra.js";

const cw20Contract = "terra1wjm8lkjasaktdmw95h6g03m8m2c45es844df2v";
const wallet = wallets.lunaHomework;

const msg = new MsgExecuteContract(
  // Address of wallet that is signing the transaction
  wallet.key.accAddress,
  // Address of CW20 contrac:
  cw20Contract,
  // ExecuteMsg payload
  {
    mint: {
      // Address of wallet or contract that is getting the tokens
      recipient: "terra1320he3k82kflf9tf729xszudwwznvsyy8x2aqw",
      // Amount of tokens to transfer, in microunits
      amount: "6969696969",
    },
  }
);

const tx = await wallet.createAndSignTx({ msgs: [msg] });
const result = await client.tx.broadcast(tx);

console.log(result);
