import * as anchor from "@project-serum/anchor";
import { TOKEN_PROGRAM_ID } from "@solana/spl-token";

// env consts
const IS_DEVNET = true;
const LOCAL_KEYPAIR_FPATH = "/home/alphaprime8/.config/solana/id.json";
const PROGRAM_ID = 'GzMvD8AGSiRhHapNsJzUMoYR3pkbCg6vPnnopaeFZE7E'; // can also load from file as done with localKeypair below

const MULTISIG_PDA_SEED = "multisig_pda_seed";
const GIGS_VAULT_PDA_SEED = "gigs_vault_pda_seed";
const PROPOSAL_PDA_SEED = "proposal_pda_seed";

const ProposalType = {
    UpgradeProgram: { upgradeProgram: {} },
    SetAuthority: { setAuthority: {} },
};

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
    let [multisig_pda, _] = await anchor.web3.PublicKey.findProgramAddress(
        [Buffer.from(anchor.utils.bytes.utf8.encode(MULTISIG_PDA_SEED))],
        program.programId
    ); // CKk2EQ6ybz6qMxMAVgDxdRksjLAQgLTfW47t9LwERW3z

    let [gigsVault, _b2] = await anchor.web3.PublicKey.findProgramAddress(
        [Buffer.from(anchor.utils.bytes.utf8.encode(GIGS_VAULT_PDA_SEED))],
        program.programId
    );

    let [proposalPda, _b3] = await anchor.web3.PublicKey.findProgramAddress(
        [Buffer.from(anchor.utils.bytes.utf8.encode(PROPOSAL_PDA_SEED))],
        program.programId
    );

    let gigsMint = new anchor.web3.PublicKey("58enXRckAspQsKqKr9ct7Hu69ope2uj5fFdmFuXNxmpc");

    let approval_threshold = new anchor.BN(1000);
    let proposal_minimum = new anchor.BN(500);

    const tx = await program.methods.initialize(approval_threshold, proposal_minimum)
        .accounts({
            signer: program.provider.publicKey,
            multisigPda: multisig_pda,
            proposal: proposalPda,
            gigsMint: gigsMint,
            gigsVault: gigsVault,
            systemProgram: anchor.web3.SystemProgram.programId,
            tokenProgram: TOKEN_PROGRAM_ID,
            rent: anchor.web3.SYSVAR_RENT_PUBKEY,
        })
        .rpc();
    console.log("Your transaction signature", tx);
}

async function propose() {
    const program = await initProgram();
    let [proposalPda, _b3] = await anchor.web3.PublicKey.findProgramAddress(
        [Buffer.from(anchor.utils.bytes.utf8.encode(PROPOSAL_PDA_SEED))],
        program.programId
    );
    let [gigsVault, _b2] = await anchor.web3.PublicKey.findProgramAddress(
        [Buffer.from(anchor.utils.bytes.utf8.encode(GIGS_VAULT_PDA_SEED))],
        program.programId
    );

    let ballot = anchor.web3.Keypair.generate();
    let gigsMint = new anchor.web3.PublicKey("58enXRckAspQsKqKr9ct7Hu69ope2uj5fFdmFuXNxmpc");
    let senderGigsAta = new anchor.web3.PublicKey("4DMKoc2UZgenrdG7Sxk2b3JusmcQ6WREAgUTenDTj2Tb");

    let proposal_type = ProposalType.SetAuthority;
    let target_buffer = new anchor.web3.PublicKey("4jQUaj3oR5H3vgLNtKXvf2V8v5rubeLcsqZD1rfT1rwb");
    let source_buffer = program.provider.publicKey;
    let new_authority = program.provider.publicKey; // TODO change to owner 1?

    let amount = new anchor.BN(500);

    // @ts-ignore
    const tx = await program.methods.propose(proposal_type, target_buffer, source_buffer, new_authority, amount)
        .accounts({
            signer: program.provider.publicKey,
            proposal: proposalPda,
            ballot: ballot.publicKey,
            gigsMint: gigsMint,
            gigsVault: gigsVault,
            senderGigsAta: senderGigsAta,
            systemProgram: anchor.web3.SystemProgram.programId,
            tokenProgram: TOKEN_PROGRAM_ID,
            rent: anchor.web3.SYSVAR_RENT_PUBKEY,
        })
        .signers([ballot])
        .rpc();
    console.log("Your transaction signature", tx);
}

async function cast_ballot() {
    const program = await initProgram();

    let [gigsVault, _b2] = await anchor.web3.PublicKey.findProgramAddress(
        [Buffer.from(anchor.utils.bytes.utf8.encode(GIGS_VAULT_PDA_SEED))],
        program.programId
    );
    let [proposalPda, _b3] = await anchor.web3.PublicKey.findProgramAddress(
        [Buffer.from(anchor.utils.bytes.utf8.encode(PROPOSAL_PDA_SEED))],
        program.programId
    );
    let gigsMint = new anchor.web3.PublicKey("58enXRckAspQsKqKr9ct7Hu69ope2uj5fFdmFuXNxmpc");
    let senderGigsAta = new anchor.web3.PublicKey("4DMKoc2UZgenrdG7Sxk2b3JusmcQ6WREAgUTenDTj2Tb");

    let ballot = anchor.web3.Keypair.generate();
    let proposal_id = new anchor.BN(1);
    let amount = new anchor.BN(501);

    // @ts-ignore
    const tx = await program.methods.castBallot(proposal_id, amount)
        .accounts({
            signer: program.provider.publicKey,
            ballot: ballot.publicKey,
            proposal: proposalPda,
            gigsMint: gigsMint,
            gigsVault: gigsVault,
            senderGigsAta: senderGigsAta,
            systemProgram: anchor.web3.SystemProgram.programId,
            tokenProgram: TOKEN_PROGRAM_ID,
            rent: anchor.web3.SYSVAR_RENT_PUBKEY,
        })
        .signers([ballot])
        .rpc();

    console.log("Your transaction signature", tx);
}

async function execute_set_authority() {
    const program = await initProgram();
    let target_buffer = new anchor.web3.PublicKey("4jQUaj3oR5H3vgLNtKXvf2V8v5rubeLcsqZD1rfT1rwb");
    let bft_loader_upgradeable = new anchor.web3.PublicKey("BPFLoaderUpgradeab1e11111111111111111111111")
    let [multisig_pda, _] = await anchor.web3.PublicKey.findProgramAddress(
        [Buffer.from(anchor.utils.bytes.utf8.encode(MULTISIG_PDA_SEED))],
        program.programId
    );
        let [gigsVault, _b2] = await anchor.web3.PublicKey.findProgramAddress(
        [Buffer.from(anchor.utils.bytes.utf8.encode(GIGS_VAULT_PDA_SEED))],
        program.programId
    );
    let gigsMint = new anchor.web3.PublicKey("58enXRckAspQsKqKr9ct7Hu69ope2uj5fFdmFuXNxmpc");
    let senderGigsAta = new anchor.web3.PublicKey("4DMKoc2UZgenrdG7Sxk2b3JusmcQ6WREAgUTenDTj2Tb");

    let [proposalPda, _b3] = await anchor.web3.PublicKey.findProgramAddress(
        [Buffer.from(anchor.utils.bytes.utf8.encode(PROPOSAL_PDA_SEED))],
        program.programId
    );

    const tx = await program.methods.executeSetAuthority()
        .accounts({
            signer: program.provider.publicKey,
            targetProgramBuffer: target_buffer,
            multisigPda: multisig_pda,
            proposal: proposalPda,
            newAuthority: program.provider.publicKey,
            systemProgram: anchor.web3.SystemProgram.programId,
            bpfLoader: bft_loader_upgradeable,
        })
        .rpc({skipPreflight: true});
    console.log("Your transaction signature", tx);

}

async function close_ballot() {
    const program = await initProgram();

    let [multisigPda, _] = await anchor.web3.PublicKey.findProgramAddress(
        [Buffer.from(anchor.utils.bytes.utf8.encode(MULTISIG_PDA_SEED))],
        program.programId
    );
    let [proposalPda, _b3] = await anchor.web3.PublicKey.findProgramAddress(
        [Buffer.from(anchor.utils.bytes.utf8.encode(PROPOSAL_PDA_SEED))],
        program.programId
    );
    let [gigsVault, _b2] = await anchor.web3.PublicKey.findProgramAddress(
        [Buffer.from(anchor.utils.bytes.utf8.encode(GIGS_VAULT_PDA_SEED))],
        program.programId
    );

    let gigsMint = new anchor.web3.PublicKey("58enXRckAspQsKqKr9ct7Hu69ope2uj5fFdmFuXNxmpc");
    let senderGigsAta = new anchor.web3.PublicKey("4DMKoc2UZgenrdG7Sxk2b3JusmcQ6WREAgUTenDTj2Tb");
    // let ballot = new anchor.web3.PublicKey("Asn7gbJ5gqaGy2UrK9fqgqv7vmoKoYad942gkdcweoZG");
    let ballot = new anchor.web3.PublicKey("9GpVR7Y8o8odx3oLZdmJ88Hn68DWsbUSW1dkMuGJBsNt");

    const tx = await program.methods.closeBallot()
        .accounts({
            signer: program.provider.publicKey,
            multisigPda: multisigPda,
            ballot: ballot,
            proposal: proposalPda,
            gigsMint: gigsMint,
            gigsVault: gigsVault,
            senderGigsAta: senderGigsAta,
            systemProgram: anchor.web3.SystemProgram.programId,
            tokenProgram: TOKEN_PROGRAM_ID,
            rent: anchor.web3.SYSVAR_RENT_PUBKEY,
        })
        .rpc();

    console.log("Your transaction signature", tx);

}



// async function execute_upgrade() {
//     const program = await initProgram();
//
//     let target_program_buffer = new anchor.web3.PublicKey("7mUcBh84gBpPpo8tCRWmxvsTSqnT79ooXFd3ozjmteZW");
//     let target_program = new anchor.web3.PublicKey("7w9oX4fSFFW9YK7iWYqBUzEwXJHa3UY3wP4y8HvpaU2s");
//     let source_buffer = new anchor.web3.PublicKey("3rkWkQ1dzhVgdUSWqscBQqzBpB6nnzppbnnFaHPVuNwG");
//     let bft_loader_upgradeable = new anchor.web3.PublicKey("BPFLoaderUpgradeab1e11111111111111111111111")
//
//     let [multisig_pda, _] = await anchor.web3.PublicKey.findProgramAddress(
//         [Buffer.from(anchor.utils.bytes.utf8.encode(MULTISIG_PDA_SEED))],
//         program.programId
//     ); // EKLRkeyMnHsAvSfXKaRnhaRztjDqgFkw6jHWzhB9eRZh
//
//     console.log("Got multisig_pda: ", multisig_pda.toString());
//
//     const tx = await program.methods.upgrade()
//         .accounts({
//             targetProgramBuffer: target_program_buffer,
//             targetProgram: target_program,
//             sourceBuffer: source_buffer,
//             signer: program.provider.publicKey,
//             rent: anchor.web3.SYSVAR_RENT_PUBKEY,
//             clock: anchor.web3.SYSVAR_CLOCK_PUBKEY,
//             multisigPda: multisig_pda,
//             systemProgram: anchor.web3.SystemProgram.programId,
//             bpfLoader: bft_loader_upgradeable,
//         })
//         .rpc({skipPreflight: true});
//     console.log("Your transaction signature", tx);
//
// }

close_ballot()
    .then(()=>{
        console.log("done")
    })
