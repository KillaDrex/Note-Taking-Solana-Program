import * as web3 from '@solana/web3.js';
import * as borsh from '@project-serum/borsh';
import * as anchor from "@project-serum/anchor";
const { BN } = anchor.default; // use of default is needed in .mjs extension
import Dotenv from 'dotenv';
import bs58 from 'bs58';
Dotenv.config();

const note = {
    id: new BN(0),
    title: "Introduction Part 2",
    body: "This note was edited."
};

main();

async function main() {
    // create container for instruction data
    const noteInstructionLayout = borsh.struct([
        borsh.u8('variant'),
        borsh.u16('id'),
        borsh.str('title'),
       borsh.str('body')
    ]);
    // encode instruction data
    let buffer = Buffer.alloc(1000);
    noteInstructionLayout.encode({
        variant: 1,
        id: note.id,
        title: note.title,
        body: note.body
    },
    buffer 
    );
    buffer = buffer.slice(0, noteInstructionLayout.getSpan(buffer) );

    // payer
    const payer = web3.Keypair.fromSecretKey(bs58.decode(process.env.PRIVATE_KEY));

    // get program address (pda)
    const [pda, bump_seed] = await web3.PublicKey.findProgramAddress(
        [payer.publicKey.toBuffer(), note.id.toBuffer('le', 2)], new web3.PublicKey(process.env.PROGRAM_ID)
    )
    // create transaction
    const transaction = new web3.Transaction();
    const instruction = new web3.TransactionInstruction({
        keys: [
            {
                pubkey: payer.publicKey,
                isSigner: true,
                isWritable: false
            },
            {
                pubkey: pda,
                isSigner: false,
                isWritable: true
            },
            {
                pubkey: web3.SystemProgram.programId,
                isSigner: false,
                isWritable: false
            }
        ],
        programId: new web3.PublicKey(process.env.PROGRAM_ID),
        data: buffer
    });
    transaction.add(instruction);

    // connection
    const connection = new web3.Connection(web3.clusterApiUrl("devnet"));
    
    const transactionSignature = await web3.sendAndConfirmTransaction(connection, transaction, [payer])

    // print link to transaction
    console.log(`Transaction: https://explorer.solana.com/tx/${transactionSignature}?cluster=devnet`)
}