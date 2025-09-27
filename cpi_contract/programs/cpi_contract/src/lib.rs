use anchor_lang::prelude::*;

declare_id!("7WfUBfVcK9RucEhTy2yiJJEmMozdB2vBNsu3xwD7AHqo");

#[program]
pub mod cpi_contract {
    use anchor_lang::solana_program::program::invoke;
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let program_id = ctx.accounts.cpiprogram.key();

        let accounts = vec![
            AccountMeta {
                is_signer: true,
                is_writable: true,
                pubkey: ctx.accounts.signer.key(),
            },
            AccountMeta {
                is_signer: true,
                is_writable: true,
                pubkey: ctx.accounts.account.key(),
            },
            AccountMeta {
                is_signer: false,
                is_writable: false,
                pubkey: ctx.accounts.system_program.key(),
            },
        ];

        let xi = anchor_lang::solana_program::instruction::Instruction {
            program_id,
            accounts,
            data: vec![0],
        };
        invoke(
            &xi,
            &[
                ctx.accounts.signer.to_account_info(),
                ctx.accounts.account.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ]
        )?;
        Ok(())
    }
    pub fn double(ctx: Context<Modify>) -> Result<()> {
        let program_id = ctx.accounts.cpi_program.key();
        let accounts = vec![
            AccountMeta {
                is_signer: true,
                is_writable: true,
                pubkey: ctx.accounts.signer.key(),
            },
            AccountMeta {
                is_signer: true,
                is_writable: true,
                pubkey: ctx.accounts.account.key(),
            },
            AccountMeta{
                is_signer:false,
                is_writable:false,
                pubkey:ctx.accounts.system_program.key()
            }
          
        ];


        let ix = anchor_lang::solana_program::instruction::Instruction {
            program_id,
            accounts,
            data: vec![1],
        };
        invoke(
            &ix,
            &[
                ctx.accounts.signer.to_account_info(),
                ctx.accounts.account.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
        )?;

        Ok(())
    }
    pub fn halve(ctx: Context<Modify>) -> Result<()> {
        let program_id = ctx.accounts.cpi_program.key();
        let accounts = vec![
            AccountMeta {
                is_signer: true,
                is_writable: true,
                pubkey: ctx.accounts.signer.key(),
            },
            AccountMeta {
                is_signer: true,
                is_writable: true,
                pubkey: ctx.accounts.account.key(),
            },
            AccountMeta{
                is_signer:false,
                is_writable:false,
                pubkey:ctx.accounts.system_program.key()
            }
          
        ];


        let ix = anchor_lang::solana_program::instruction::Instruction {
            program_id,
            accounts,
            data: vec![2],
        };
        invoke(
            &ix,
            &[
                ctx.accounts.signer.to_account_info(),
                ctx.accounts.account.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
        )?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut,signer)]
    /// CHECK:is safe
    pub account: AccountInfo<'info>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
    /// CHECK: This is safe because we are only reading the program id
    pub cpiprogram: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct Modify<'info> {
    #[account(mut,signer)]
    /// CHECK:is  not unsafe
    pub account: AccountInfo<'info>,
    #[account(mut)]
    pub signer: Signer<'info>,
     pub system_program: Program<'info, System>,
    /// CHECK: This is safe because we are only reading the program id
    pub cpi_program: AccountInfo<'info>,
}

#[account]
pub struct Counter {
    count: u64,
}


//  let data = cpi_program::instruction::Double {}.data();