use anchor_lang::prelude::*;

declare_id!("7ffMY6G9LXz8KArt5CS9APuJMQC547Q7sZ194SyQ1Rij");

#[program]
pub mod web3 {
    

    use super::*;

    pub fn create_campaign(
        ctx: Context<CreateCampaign>,
        title: String,
        description: String,
        target: u64,
        deadline: i64,
        image: String,
    ) -> Result<()> {
        let campaign = &mut ctx.accounts.campaign;
        let clock = Clock::get().unwrap();
        
        require!(deadline > clock.unix_timestamp, ErrorCode::InvalidDeadline);

        campaign.owner = ctx.accounts.owner.key();
        campaign.title = title;
        campaign.description = description;
        campaign.target = target;
        campaign.deadline = deadline;
        campaign.amount_collected = 0;
        campaign.image = image;
        campaign.donators = Vec::new();
        campaign.donations = Vec::new();

        Ok(())
    }
    
    pub fn donate_to_campaign(ctx: Context<DonateToCampaign>, amount: u64) -> Result<()> {
        let campaign = &mut ctx.accounts.campaign;
        let clock = Clock::get().unwrap();

        require!(clock.unix_timestamp < campaign.deadline, ErrorCode::DeadlinePassed);

        campaign.donators.push(ctx.accounts.donator.key());
        campaign.donations.push(amount);
        campaign.amount_collected += amount;

        **ctx.accounts.campaign.to_account_info().try_borrow_mut_lamports()? -= amount;
        **ctx.accounts.donator.to_account_info().try_borrow_mut_lamports()? += amount;

        Ok(())
    }

    pub fn get_donators(ctx: Context<GetDonators>) -> Result<DonatorsResponse> {
        let campaign = &ctx.accounts.campaign;
        Ok(DonatorsResponse {
            donators: campaign.donators.clone(),
            donations: campaign.donations.clone(),
        })
    }

    pub fn get_all_campaigns(ctx: Context<GetAllCampaigns>) -> Result<AllCampaignsResponse> {
        let state = &ctx.accounts.state;
        Ok(AllCampaignsResponse {
            campaigns: state.campaigns.clone(),
        })
    }
}


#[derive(Accounts)]
pub struct CreateCampaign<'info> {
    #[account(init, payer = owner, space = 8 + 256)]
    pub campaign: Account<'info, Campaign>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct DonateToCampaign<'info> {
    #[account(mut)]
    pub campaign: Account<'info, Campaign>,
    #[account(mut)]
    pub donator: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct GetDonators<'info> {
    #[account(mut)]
    pub campaign: Account<'info, Campaign>,
}

#[derive(Accounts)]
pub struct GetAllCampaigns<'info> {
    #[account(mut)]
    pub state: Account<'info, State>,
}

#[account]
pub struct State {
    pub campaigns: Vec<Pubkey>,
}

#[account]
pub struct Campaign {
    pub owner: Pubkey,
    pub title: String,
    pub description: String,
    pub target: u64,
    pub deadline: i64,
    pub amount_collected: u64,
    pub image: String,
    pub donators: Vec<Pubkey>,
    pub donations: Vec<u64>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct AllCampaignsResponse {
    pub campaigns: Vec<Pubkey>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("The deadline should be a date in the future.")]
    InvalidDeadline,
    #[msg("The deadline has passed.")]
    DeadlinePassed,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct DonatorsResponse {
    pub donators: Vec<Pubkey>,
    pub donations: Vec<u64>,
}




