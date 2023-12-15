use anchor_lang::prelude::*;

declare_id!("61FzjtyHgwuB9H99YV1X7L6oporpWXNA8ugMf4816Y1t");

#[program]
pub mod voter {
    use super::*;



    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let party_jkl = &mut ctx.accounts.party_jkl;
        if party_jkl.votes >= 0{
            party_jkl.votes= party_jkl.votes;
        }
        msg!("party_jkl initialized with {} votes",party_jkl.votes);
        Ok(())
    }

        pub fn upvote(ctx: Context<UpVote>)-> Result<()>{
        let party_jkl = &mut ctx.accounts.party_jkl;
        party_jkl.votes +=1;
        msg!("votes updated : {}",party_jkl.votes);
        Ok(())
    }

}

#[derive(Accounts)]
pub struct CheckVote<'info>{
    #[account(mut)]
    pub party_jkl:Account<'info, JKL>
}


#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer=user, space=8+8)]
    pub party_jkl: Account<'info, JKL>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}


#[derive(Accounts)]
pub struct UpVote<'info> {
    #[account(mut)]
    pub party_jkl: Account<'info, JKL>,
}

#[account]
pub struct JKL{
    pub votes: u64
}