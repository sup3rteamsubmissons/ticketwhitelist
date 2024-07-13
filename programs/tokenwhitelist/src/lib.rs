use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, TokenAccount, Transfer};

declare_id!("8BNEYJRtpxyTKQEMTQDZYj4hAie7tqdeEJ59DXhehGV9");

#[program]
pub mod whitelist_token_sale {
    use super::*;

    pub fn initialize_sale(ctx: Context<InitializeSale>, price: u64, max_per_wallet: u64) -> Result<()> {
        let sale = &mut ctx.accounts.sale;
        sale.price = price;
        sale.max_per_wallet = max_per_wallet;
        sale.token_mint = ctx.accounts.token_mint.key();
        sale.seller = ctx.accounts.seller.key();
        Ok(())
    }

    pub fn add_to_whitelist(ctx: Context<AddToWhitelist>) -> Result<()> {
        let whitelist = &mut ctx.accounts.whitelist;
        whitelist.is_whitelisted = true;
        Ok(())
    }

    pub fn buy_tokens(ctx: Context<BuyTokens>, amount: u64) -> Result<()> {
        let sale = &ctx.accounts.sale;
        let user_record = &mut ctx.accounts.user_record;
        let user_wallet = &ctx.accounts.user_wallet;
        let token_account = &ctx.accounts.token_account;

        require!(amount > 0, CustomError::InvalidAmount);
        require!(user_record.amount_purchased + amount <= sale.max_per_wallet, CustomError::ExceedsMaxPurchase);
        require!(user_wallet.to_account_info().lamports() >= amount * sale.price, CustomError::InsufficientFunds);

        if !ctx.accounts.whitelist.is_whitelisted {
            return Err(CustomError::NotWhitelisted.into());
        }

        // Transfer tokens from seller to user
        let cpi_accounts = Transfer {
            from: ctx.accounts.seller_token_account.to_account_info(),
            to: token_account.to_account_info(),
            authority: ctx.accounts.sale.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let seeds = &[sale.to_account_info().key.as_ref(), &[sale.bump]];
        let signer = &[&seeds[..]];
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
        token::transfer(cpi_ctx, amount)?;

        // Update user's purchase record
        user_record.amount_purchased += amount;

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(price: u64, max_per_wallet: u64)]
pub struct InitializeSale<'info> {
    #[account(init, payer = seller, space = 8 + 8 + 8 + 32 + 32 + 1, seeds = [b"sale"], bump)]
    pub sale: Account<'info, Sale>,
    pub token_mint: Account<'info, Mint>,
    #[account(mut)]
    pub seller: Signer<'info>,
    #[account(mut)]
    pub seller_token_account: Account<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, anchor_spl::token::Token>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct AddToWhitelist<'info> {
    #[account(init, payer = user, space = 8 + 1, seeds = [user.key().as_ref()], bump)]
    pub whitelist: Account<'info, Whitelist>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
#[instruction(amount: u64)]
pub struct BuyTokens<'info> {
    #[account(mut)]
    pub sale: Account<'info, Sale>,
    #[account(mut)]
    pub user_wallet: Signer<'info>,
    #[account(init_if_needed, payer = user_wallet, space = 8 + 8, seeds = [user_wallet.key().as_ref(), b"user_record"], bump)]
    pub user_record: Account<'info, UserRecord>,
    #[account(mut)]
    pub token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub seller_token_account: Account<'info, TokenAccount>,
    pub whitelist: Account<'info, Whitelist>,
    pub token_program: Program<'info, anchor_spl::token::Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[account]
pub struct Sale {
    pub price: u64,
    pub max_per_wallet: u64,
    pub token_mint: Pubkey,
    pub seller: Pubkey,
    pub bump: u8,
}

#[account]
pub struct Whitelist {
    pub is_whitelisted: bool,
    pub bump: u8,
}

#[account]
pub struct UserRecord {
    pub amount_purchased: u64,
    pub bump: u8,
}

#[error_code]
pub enum CustomError {
    #[msg("Invalid amount.")]
    InvalidAmount,
    #[msg("Exceeds maximum purchase limit per wallet.")]
    ExceedsMaxPurchase,
    #[msg("Insufficient funds.")]
    InsufficientFunds,
    #[msg("User is not whitelisted.")]
    NotWhitelisted,
}
