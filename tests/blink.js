const anchor = require('@project-serum/anchor');
const { SystemProgram } = anchor.web3;

describe('whitelist_token_sale', () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.local();
  anchor.setProvider(provider);

  const program = anchor.workspace.WhitelistTokenSale;

  it('Initializes the sale!', async () => {
    const sale = anchor.web3.Keypair.generate();
    const tokenMint = anchor.web3.Keypair.generate();
    const seller = provider.wallet.publicKey;
    const sellerTokenAccount = anchor.web3.Keypair.generate();

    await program.rpc.initializeSale(new anchor.BN(1000), new anchor.BN(10), {
      accounts: {
        sale: sale.publicKey,
        tokenMint: tokenMint.publicKey,
        seller: seller,
        sellerTokenAccount: sellerTokenAccount.publicKey,
        systemProgram: SystemProgram.programId,
        tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      },
      signers: [sale],
    });

    const saleAccount = await program.account.sale.fetch(sale.publicKey);
    console.log('Sale account:', saleAccount);
  });

  it('Adds a user to the whitelist!', async () => {
    const user = provider.wallet.publicKey;
    const whitelist = anchor.web3.Keypair.generate();

    await program.rpc.addToWhitelist({
      accounts: {
        whitelist: whitelist.publicKey,
        user: user,
        systemProgram: SystemProgram.programId,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      },
      signers: [whitelist],
    });

    const whitelistAccount = await program.account.whitelist.fetch(whitelist.publicKey);
    console.log('Whitelist account:', whitelistAccount);
  });

  it('Buys tokens!', async () => {
    const sale = anchor.web3.Keypair.generate();
    const userWallet = provider.wallet.publicKey;
    const userRecord = anchor.web3.Keypair.generate();
    const tokenAccount = anchor.web3.Keypair.generate();
    const sellerTokenAccount = anchor.web3.Keypair.generate();
    const whitelist = anchor.web3.Keypair.generate();

    await program.rpc.buyTokens(new anchor.BN(5), {
      accounts: {
        sale: sale.publicKey,
        userWallet: userWallet,
        userRecord: userRecord.publicKey,
        tokenAccount: tokenAccount.publicKey,
        sellerTokenAccount: sellerTokenAccount.publicKey,
        whitelist: whitelist.publicKey,
        tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      },
      signers: [userRecord],
    });

    const userRecordAccount = await program.account.userRecord.fetch(userRecord.publicKey);
    console.log('User record account:', userRecordAccount);
  });
});