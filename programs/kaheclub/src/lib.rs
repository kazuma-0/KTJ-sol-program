use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_program;

declare_id!("AByC8vWQntfskq889DFGq6nc9EXVGi725X3em2Nnfjis");

#[program]
pub mod kaheclub {
    use super::*;

    pub fn send_tweet(ctx: Context<SendTweet>, topic: String, content: String, tag: String) -> Result<()>{
        let tweet: &mut Account<Tweet> = &mut ctx.accounts.tweet;
        let author: &Signer = &ctx.accounts.author;
        let clock: Clock = Clock::get().unwrap();

        if topic.chars().count() > 50{
            return Err(error!(ErrorCode::TopicTooLong))
        }

        if content.chars().count() > 280{
            return Err(error!(ErrorCode::ContentTooLong))
        }
        if tag.chars().count() > 10 {
            return Err(error!(ErrorCode::TagTooLong))
        }

        tweet.author = *author.key;
        tweet.timestamp = clock.unix_timestamp;
        tweet.topic = topic;
        tweet.content = content;
        tweet.tag = tag;

        Ok(())
    }

    pub fn delete_tweet(_ctx: Context<DeleteTweet>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SendTweet<'info>{
    #[account(init, payer= author, space= Tweet::LEN)]
    pub tweet: Account<'info, Tweet>,
    #[account(mut)]
    pub author: Signer<'info>,
    #[account(address = system_program::ID)]
    pub system_program: Program<'info,System>,
}

#[derive(Accounts)]
pub struct DeleteTweet<'info> {
    #[account(mut, has_one = author, close = author)]
    pub tweet: Account<'info, Tweet>,
    pub author: Signer<'info>,
}


#[account]
pub struct Tweet {
    pub author: Pubkey,
    pub timestamp: i64,
    pub topic: String,
    pub content: String,
    pub tag: String,
}


const DISCRIMINATOR_LENGTH: usize = 8;
const PUBLIC_KEY_LENGTH: usize = 32;
const TIMESTAMP_LENGTH: usize = 8;
const STRING_LENGTH_PREFIX: usize = 4; // stores the size of the string
const TOPIC_LENGTH: usize = 50 * 4; // 50 characters
const MAX_CONTENT_LENGTH: usize = 400 * 4; // 280 chars max
const TAG_LENGTH: usize = 10 * 4;

impl Tweet {
    const LEN: usize =
        DISCRIMINATOR_LENGTH
            + PUBLIC_KEY_LENGTH
            + TIMESTAMP_LENGTH
            + STRING_LENGTH_PREFIX + TOPIC_LENGTH
            + STRING_LENGTH_PREFIX + MAX_CONTENT_LENGTH
            + STRING_LENGTH_PREFIX + TAG_LENGTH;
}

#[error_code]
pub enum ErrorCode{
    #[msg("The provided topic should be 50 characters long maximum")]
    TopicTooLong,
    #[msg("The provided content should be 400 characters long maximum")]
    ContentTooLong,
    #[msg("The provided tag should be 400 characters long maximum")]
    TagTooLong
}