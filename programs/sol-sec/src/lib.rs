use anchor_lang::prelude::*;
use std::time::{Duration, SystemTime};
use crypto_hash::{Algorithm, hex_digest};
use base64::encode;
use anchor_lang::solana_program::{self,system_program, sysvar::rent::Rent,};

// This is your program's public key and it will update
// automatically when you build the project.
declare_id!("11111111111111111111111111111111");

#[program]
mod hello_anchor {
    use super::*;
           
    pub fn create_proposal(ctx: Context<Create_Proposal>,
        pool_prize: u64,
        days_b4_contest : u64,
        high_rish_value: u8,
        medium_risk_value: u8,
        title: String,
        prize_pool: u64) -> Result<()>{

        // to be converted to sol    
        let min_amount = 10000;
        require!(pool_prize>min_amount);

        //a minimum time is to be decided upon after acception
          
        
        let start_time = ctx.accounts.clock.unix_timestamp +  days_b4_contest* 24 * 60 * 60;
        let end_date = start_time + contest_duration * 24 * 60 * 60;   
          
        let judge_cut = (10*pool_prize)/100;
        let DAO_cut = (5*pool_prize)/100;

        // Variable cuts for vulnerabilities
        //should add up to 100
        //value taken in percent/100  
        let high_risk_vulnerability_percent = high_risk_value;
        let high_risk_pool = high_risk_vulnerability_percent*pool_prize;
        let medium_risk_vulnerability_percent = medium_risk_value; 
        let medium_risk_pool = medium_risk_vulnerability_percent*pool_prize;
        let gas_report_and_low_risk_cut = (85/100 - high_risk_vulnerability_percent -  medium_risk_vulnerability_percent)*pool_prize;

        let data1 = pool_prize.as_bytes();
        let data2 = title.as_bytes();
        let data3 = start_time.as_bytes();
        let data4 = end_time.as_bytes();

        let combined_data = [data1, data2, data3, data4].concat();
        let hash_output = hex_digest(Algorithm::SHA256, &combined_data);
        let encoded_hash = encode(&hash_output);
        //stake 25 % of the pool prize 
        // how to implement
        let stake_account = &mut ctx.accounts.stake_account;
        
        stake_account.stake = 25*pool_prize/100;
        stake_account.staker = ctx.accounts.authority.key();
        stake_account.bump = *ctx.bumps.get("stake_account").unwrap();
        stake_account.proposal_id = encoded_hash;

        // where to put the stake ? which account will have it ?
        // ans = PDA account 


        let proposal = Proposal {
        authority: ctx.accounts.authority.clone(),
        governance_token_account: ctx.accounts.governance_token_account.clone(),
        proposal_account: ctx.accounts.proposal_account.clone(),
        title,
        start_date = start_time,
        end_date = end_time,
        prize_pool,
        proposal_id = encoded_hash,// created from contest info 
        proposal_eligible=false,
        success=true,
        };

        Ok(());
      }

    pub fn vote_for_proposal(ctx: Context<Vote_For_Proposal>,vote_type: VoteType) -> Result<()>{
    
        // where to define voting_end variable
    if(voting_end == true){
        Ok(())
    }     

    let vote_account = &mut ctx.accounts.vote_account;
    let governance_token_account = &mut ctx.accounts.governance_token_account;
    let voter = &ctx.accounts.voter;

    // Check if the governance token is owned by the program.
    if governance_token_account.owner != *ctx.program_id {
        return Err(ErrorCode::NotProgramToken.into());
    }

    // Check if the voter has enough tokens to cast a vote.
    let voter_balance = governance_token_account.balance;
    if voter_balance == 0 {
        return Err(ErrorCode::InsufficientTokens.into());
    }

    
    let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?;
    let voting_period = Duration::from_secs(2 * 24 * 60 * 60); // 2 days
    let voting_start_time = proposal.voting_start_time; // how to ensure correct proposal is selected
    let voting_end_time = voting_start_time + voting_period;
        
    if current_time < voting_start_time || current_time > voting_end_time {
        return Err(ErrorCode::VotingPeriodOver.into());
    }
            match vote_type {
            VoteType::YES => {
                msg!("Voted for YES 🤝");
                 let vote_weight = voter_balance;
                 vote_account.yes += vote_weight; 
            },
            VoteType::NO => {
                msg!("Voted for NO 🤞");
                 let vote_weight = voter_balance;
                 vote_account.no += vote_weight;
            },
        
        let end = false;
        if(current_time > voting_end_time){
            end = true;
        }
        
        if(end == true){
            voting_verdict();
            voting_end= true;
        }
        };
        Ok(())

          // delegation of power ??
      }

      pub fn voting_verdict(ctx: <Voting_Verdict>) -> Result<()>{
        
        // need to create proposal id with proposal attribute
        if(yes_votes > (66*total_votes_casted)/100 && total_votes_casted > 10*totalsupply/100){
          accounts.proposal_eligible = true ;
        }else{
          accounts.proposal_eligible = false ;
        }
          Ok(());
      }
     
      pub fn start_contest(ctx: Context<Start_Contest>) -> Result<()>{
          // bind the proposal id with specific user so as to act as owner of proposal
          /// already binded with proposal with authority
          //option to trigger the contest needed
          let stake_left = 75*prize_pool/100;
          stake_account.stake = stake_account.stake + stake_left;
          //add codebase
          Ok(())
      }

      pub fn apply_for_judge(ctx: Context<Apply_For_Judge>,
        name:String ,
        email:String ,
        proposal_id: String , // contest id of that protocol
        ) -> Result<()>{

          let data1 = name.as_bytes();
          let data2 = email.as_bytes();
          let data3 = proposal_id.as_bytes();
          
          let combined_data = [data1, data2, data3].concat();
          let hash_output = hex_digest(Algorithm::SHA256, &combined_data);
          let encoded_hash = encode(&hash_output);
          
          // add contestants using PDA , it can also act as a mapping
          let candidate = new Candidate {
             name,
             email,
             proposal_id,
             votes=0;
             candidate_id = encoded_hash, 
          };
           
          Ok(())
      }

    pub fn vote_for_judge(ctx: Context<Vote_For_Judge>,choice1: String,choice2: String,choice3: String) -> Result<()>{
    
    let vote_account = &mut ctx.accounts.vote_account;
    let governance_token_account = &mut ctx.accounts.governance_token_account;
    let voter = &ctx.accounts.voter;

    // Check if the governance token is owned by the program.
    if governance_token_account.owner != *ctx.program_id {
        return Err(ErrorCode::NotProgramToken.into());
    }

    // Check if the voter has enough tokens to cast a vote.
    let voter_balance = governance_token_account.balance;
    if voter_balance == 0 {
        return Err(ErrorCode::InsufficientTokens.into());
    }

    //no proposal struct yet
    let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?;
    let voting_period = Duration::from_secs(2 * 24 * 60 * 60); // 2 days
    let voting_start_time = proposal.voting_start_time;
    let voting_end_time = voting_start_time + voting_period;
        
    if current_time < voting_start_time || current_time > voting_end_time {
        return Err(ErrorCode::VotingPeriodOver.into());
    }
        // voting judges based on priority 
        // not properly implemented right now
         let vote_weight = voter_balance;
          choice1.votes += 5*vote_weight;
          choice2.votes += 3*vote_weight;
          choice3.votes += 1*vote_weight;
          
        Ok(())
       
      }
       
      // where to store the judges ??
      //need only till the contest ends
      pub fn get_candidates(_ctx: Context<GetCandidates>) -> ProgramResult<Vec<String>> {
        let accounts = _ctx.accounts;
    
        
        let mut candidate_votes: Vec<(Candidate, u32)> = accounts
            .candidate_list
            .to_account_infos()
            .into_iter()
            .map(|info| (Candidate::unpack(&info.data.borrow()).unwrap(), info.data.borrow().len() as u32))
            .collect();
    
        // Sort the list of candidates based on the number of votes they have received, in descending order.
        candidate_votes.sort_by(|a, b| b.1.cmp(&a.1));
    
        let mut judge: Vec<<String>> = vec![None; 3];
    
        for (i, (candidate, _)) in candidate_votes.iter().enumerate().take(3) {
            judge[i] = Some(candidate.name);
        }
    
        Ok(judge.into_iter().flatten().collect())
    }
    
     
         #[access_control(SubmitReport::accounts(&ctx, &report_hash, &proposal_id))]
         pub fn submit_report(ctx: Context<Submit_Report>, report_hash: u8 ,proposal_id: String) -> Result<()>{
         
           let contest_data= new ContestDataInner {
            hash,
            proposal_id,
           };

           // how to store the data on blockchain
           let contest_data_account = &mut ctx.accounts.data;
           contest_data_account.hash = contest_data.hash;
           contest_data_account.proposal_id = contest_data.proposal_id;

           Ok(())
         }

      pub fn propose_report(ctx:Context<Propose_Report>,
         proposal_id: String, 
         high_risk_rewardees: Vec<Vec<String>> ,
         medium_risk_rewardees: Vec<Vec<String>> ,
         report_rewardees: Vec<Vec<String>>  ,
        ) -> Result<()>{

        let contestWinners = new WinnerDataInner {
         proposal_id,
         high_risk_rewardees,
         medium_risk_rewardees,
         report_rewardees, 
        };
        
        let contest_winners_account = & mut ctx.accounts.data;
        contest_data_account.proposal_id = contestWinners.proposal_id;
        contest_data_account.high_risk_rewardees = contestWinners.high_risk_rewardees;
        contest_data_account.medium_risk_rewardees = contestWinners.medium_risk_rewardees;
        contest_data_account.report_rewardees = contestWinners.report_rewardees;

        Ok(())

      }
    
      pub fn vote_for_slash(ctx:Context<Vote_For_Slash>) -> Result<()>{

          // Everytime the contest ends , this function also opens 
          // need overwhelming mojority to slash their tokens 
          // discussion and proof expected to be done off chain
          // if a malicious activity by judges found by a member , slash the stakes of the judge and dont give their prize pool
          // prize pool distributed to dao

          let vote_account = &mut ctx.accounts.vote_account;
          let governance_token_account = &mut ctx.accounts.governance_token_account;
          let voter = &ctx.accounts.voter;
      
          // Check if the governance token is owned by the program.
          if governance_token_account.owner != *ctx.program_id {
              return Err(ErrorCode::NotProgramToken.into());
          }
      
          // Check if the voter has enough tokens to cast a vote.
          let voter_balance = governance_token_account.balance;
          if voter_balance == 0 {
              return Err(ErrorCode::InsufficientTokens.into());
          }
          
          let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?;
          let voting_period = Duration::from_secs(2 * 24 * 60 * 60); // 2 days
          let voting_start_time = proposal.voting_start_time; // not implemented here too
          let voting_end_time = voting_start_time + voting_period;
        
          if current_time < voting_start_time || current_time > voting_end_time {
          return Err(ErrorCode::VotingPeriodOver.into());
          }
          
          match vote_type {
            VoteType::YES => {
                msg!("Voted for YES 🤝");
                 let vote_weight = voter_balance;
                 vote_account.yes += vote_weight; 
            },
            VoteType::NO => {
                msg!("Voted for NO 🤞");
                 let vote_weight = voter_balance;
                 vote_account.no += vote_weight;
            },
        };

        Ok(())
      }


      pub fn close_contest(ctx:Context<Close_Contest>) -> Result<()>{
        
        // check result of vote_for_slash
       
       // distribute funds
       // for now distribute equally 

       // close contest 
         Ok(())
     }
    

}

#[derive(Accounts)]
pub struct Proposal {
    //proposal by the protocol
    pub authority: Signer<'info>,
    pub governance_token_account: Account<'info, GovernanceTokenAccount>,
    pub proposal_account: Account<'info, ProposalAccount>,
    pub title: String,
    pub start_date: u64,
    pub end_date: u64,
    pub prize_pool: u64,
    pub proposal_id: String,
    pub proposal_eligible: bool,
    pub success: bool,
}

#[derive(Accounts)]
pub struct Candidate {
    pub name: String,
    pub email: String,
    pub proposal_id: u64,
    pub votes: u64,
    pub candidate_id: String,
}
 
pub struct GetCandidates<'info> {
    // to get the list of candidates
    #[account(address = "")]
    pub candidate_list: AccountInfo<'info>,
}

pub struct Create_Proposal<'info> {
 clock: Sysvar<'info, Clock>,
 #[account(
    mut,
)]
pub authority: Signer<'info>,

#[account(
    init,
    payer = authority,
    space = 256,
    seeds = [b"stake".as_ref(),authority.key().as_ref()], 
    bump
)]
pub stake_account: Box<Account<'info, StakeAccount>>,

pub system_program: Program<'info, System>,
pub rent: Sysvar<'info, Rent>,

}
                          

pub struct Vote_For_Proposal<'info> {
   
    #[account(has_one = governance_token)]
    governance_token_account: Account<'info, GovernanceToken>,
    // Storing Votes in global account
    #[account(mut)] 
    pub vote_account: Account<'info, VoteBank>,
    pub signer: Signer<'info>,

}

pub struct Get_Votes<'info>{
    #[account(has_one = governance_token)]
    governance_token_account: Account<'info, GovernanceToken>,
    #[account(has_one = vote_account)]
    vote_account: Account<'info, VoteBank>,
}

pub struct Start_Contest<'info> {
    #[account(
        mut,
    )]
    pub authority: Signer<'info>,
    
    #[account(
        init,
        payer = authority,
        space = 256,
        seeds = [b"stake".as_ref(),authority.key().as_ref()], 
        bump
    )]
    pub stake_account: Box<Account<'info, StakeAccount>>,
    
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
    
}

pub struct Apply_For_Judge<'info> {
    
}
pub struct Vote_For_Judge<'info> {
    
}
pub struct Submit_Report<'info> {
    
}
pub struct Vote_For_Slash<'info> {
    
}

#[derive(Accounts)]
pub struct WinnerData<'info> {
    #[account(init, payer = user, space = 64)]
    pub data: Account<'info, WinnerDataInner>,
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Default, BorshSerialize, BorshDeserialize)]
pub struct WinnerDataInner {
    pub proposal_id: String,
    #[account(mut)]
    pub high_risk_rewardees: Vec<Vec<String>> ,
    pub medium_risk_rewardees: Vec<Vec<String>> ,
    pub report_rewardees: Vec<Vec<String>>  ,
}

#[derive(Accounts)]
pub struct ContestData<'info> {
    #[account(init, payer = user, space = 64)]
    pub data: Account<'info, ContestDataInner>,
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Default, BorshSerialize, BorshDeserialize)]
pub struct ContestDataInner {
    pub hash: u8,
    pub proposal_id: String,
}

#[account]
pub struct GovernanceToken {
    pub owner: Pubkey,
    pub balance: u64,
}

#[derive(Default)]
pub struct VoteBank {
    yes: u64, // 8 bytes in size
    no: u64, // 8 bytes in size
}

#[derive(Default)]
pub struct StakeAccount {
    pub staker = Pubkey, 
    pub proposal_id = String,
    pub bump: u8
    #[account(mut)]
    pub stake : u64,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub enum VoteType {
    YES,
    NO
}

#[error]
pub enum ErrorCode {
    #[msg("The proposal is no longer in its voting period")]
    VotingPeriodOver,
    #[msg("Insufficient tokens")]
    InsufficientTokens,
    #[msg("Not a program-owned governance token")]
    NotProgramToken,
}