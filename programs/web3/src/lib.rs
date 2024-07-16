use anchor_lang::prelude::*;

declare_id!("7ffMY6G9LXz8KArt5CS9APuJMQC547Q7sZ194SyQ1Rij");

#[program]
pub mod web3 {
    

    use anchor_lang::solana_program::{program::invoke, system_instruction};

    use super::*;

    //As name suggestes creates campaign
    pub fn create_campaign(
        ctx: Context<CreateCampaign>,
        title: String,
        description: String,
        target: u64,
        deadline: i64,
        image: String,
    ) -> Result<()> {
        let campaign = &mut ctx.accounts.campaign;//get campaign account

        let clock = Clock::get().unwrap();//get current blockchain time
        
        require!(deadline > clock.unix_timestamp, ErrorCode::InvalidDeadline); // check for invalid deadline

        //Creation of campaign
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
    
    //As the name suggests
    pub fn donate_to_campaign(ctx: Context<DonateToCampaign>, amount: u64) -> Result<()> {
        let campaign = &mut ctx.accounts.campaign;//campaign account
        let clock = Clock::get().unwrap();

        require!(clock.unix_timestamp < campaign.deadline, ErrorCode::DeadlinePassed);//Check if deadline has passed

        campaign.donators.push(ctx.accounts.donator.key());//Add donater to donaters list

        campaign.donations.push(amount); // donation amount to array of donations

        campaign.amount_collected += amount;//total amount collected

        //Lamport transfer from donator to campaign
        invoke(
            &system_instruction::transfer(
                &ctx.accounts.donator.key(),
                &ctx.accounts.campaign.key(),
                amount,
            ),
            &[
                //Accounts involved in transfer
                ctx.accounts.donator.to_account_info(),
                ctx.accounts.campaign.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
        )?;

        Ok(())
    }

    //All list of donaters
    pub fn get_donators(ctx: Context<GetDonators>) -> Result<DonatorsResponse> {
        let campaign = &ctx.accounts.campaign;
        Ok(DonatorsResponse {
            donators: campaign.donators.clone(),
            donations: campaign.donations.clone(),
        })
    }

    //All list of campaigns
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




