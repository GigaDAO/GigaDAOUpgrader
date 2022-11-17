import * as anchor from "@project-serum/anchor";

// env consts
const IS_DEVNET = true;
const LOCAL_KEYPAIR_FPATH = "/home/alphaprime8/.config/solana/id.json";
const PROGRAM_ID = '7zytPdaZiXNjYQh1cStfAcFws7ZRhSLtUfhdoev9vp5G'; // can also load from file as done with localKeypair below

const MULTISIG_PDA_SEED = "multisig_pda_seed";

async function initProgram() {
    // INIT Web3 Connection Objects
    const localKeypair = anchor.web3.Keypair.fromSecretKey(Buffer.from(JSON.parse(require("fs").readFileSync(LOCAL_KEYPAIR_FPATH, {encoding: "utf-8",}))));
    const programId = new anchor.web3.PublicKey(PROGRAM_ID);
    let wallet = new anchor.Wallet(localKeypair);
    let opts = anchor.AnchorProvider.defaultOptions();
    const network = IS_DEVNET ? anchor.web3.clusterApiUrl('devnet') : anchor.web3.clusterApiUrl('mainnet-beta');
    let connection = new anchor.web3.Connection(network, opts.preflightCommitment);
    let provider = new anchor.AnchorProvider(connection, wallet, opts);
    let idl = await anchor.Program.fetchIdl(programId, provider);
    return new anchor.Program(idl, programId, provider);
}

async function initialize() {
    const program = await initProgram();

    let target_program_buffer = new anchor.web3.PublicKey("7mUcBh84gBpPpo8tCRWmxvsTSqnT79ooXFd3ozjmteZW");
    let target_program = new anchor.web3.PublicKey("7w9oX4fSFFW9YK7iWYqBUzEwXJHa3UY3wP4y8HvpaU2s");
    // let source_buffer = new anchor.web3.PublicKey("3rkWkQ1dzhVgdUSWqscBQqzBpB6nnzppbnnFaHPVuNwG"); // TODO close this...
    let source_buffer = new anchor.web3.PublicKey("HDmWtxcjDFxQjehJb4nygk8xKPgXKwjadSp1AngUE97B");
    let bft_loader_upgradeable = new anchor.web3.PublicKey("BPFLoaderUpgradeab1e11111111111111111111111")

    let [multisig_pda, _] = await anchor.web3.PublicKey.findProgramAddress(
        [Buffer.from(anchor.utils.bytes.utf8.encode(MULTISIG_PDA_SEED))],
        program.programId
    ); // EKLRkeyMnHsAvSfXKaRnhaRztjDqgFkw6jHWzhB9eRZh

    console.log("Got multisig_pda: ", multisig_pda.toString());

    const tx = await program.methods.upgrade()
        .accounts({
            targetProgramBuffer: target_program_buffer,
            targetProgram: target_program,
            sourceBuffer: source_buffer,
            signer: program.provider.publicKey,
            rent: anchor.web3.SYSVAR_RENT_PUBKEY,
            clock: anchor.web3.SYSVAR_CLOCK_PUBKEY,
            multisigPda: multisig_pda,
            systemProgram: anchor.web3.SystemProgram.programId,
            bpfLoader: bft_loader_upgradeable,
        })
        .rpc({skipPreflight: true});
    console.log("Your transaction signature", tx);

}

initialize()
    .then(()=>{
        console.log("done")
    })
