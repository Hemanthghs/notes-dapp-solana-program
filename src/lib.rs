use anchor_lang::prelude::*;

// This is your program's public key and it will update
// automatically when you build the project.
declare_id!("nJiToJCPGNjxQ3Q6ySWgLqEX1AybLhaJu6niQdBxosK");

#[program]
pub mod notes_dapp {
    use super::*;

    // handlers
    pub fn create_note(ctx: Context<CreateNote>, title: String, content: String) -> Result<()> {
        let note = &mut ctx.accounts.note;
        let clock = Clock::get()?;

        require!(title.len() <= 100, NotesError::TitleTooLong);
        require!(content.len() <= 1000, NotesError::ContentTooLong);
        require!(!title.trim().is_empty(), NotesError::TitleEmpty);
        require!(!content.trim().is_empty(), NotesError::ContentEmpty);

        note.author = ctx.accounts.author.key();
        note.title = title.clone();
        note.content = content.clone();
        note.created_at = clock.unix_timestamp;
        note.last_updated = clock.unix_timestamp;

        msg!(
            "Note created! Title: {}, Author: {}, Created At: {}",
            note.title,
            note.author,
            note.created_at
        );

        Ok(())
    }

    pub fn update_note(
        ctx: Context<UpdateNote>,
        content: String,
    ) -> Result<()> {
        let note = &mut ctx.accounts.note;
        let clock = Clock::get()?;

        require!(note.author == ctx.accounts.author.key(), NotesError::Unauthorized);
        require!(content.len() <= 1000, NotesError::ContentTooLong);
        require!(!content.trim().is_empty(), NotesError::ContentEmpty);

        note.content = content.clone();
        note.last_updated = clock.unix_timestamp;

        msg!("Note {} update!", note.title);

        Ok(())
    }

    pub fn delete_note(ctx: Context<DeleteNote>) -> Result<()> {
        let note = &ctx.accounts.note;

        require!(note.author == ctx.accounts.author.key(), NotesError::Unauthorized);

        msg!("Note {} deleted!", note.title);
        Ok(())

    }
}

#[derive(Accounts)]
#[instruction(title: String)]
pub struct CreateNote<'info> {
    #[account(
        init,
        payer = author,
        space = 8 + Note::INIT_SPACE,
        seeds = [b"note", author.key().as_ref(), title.as_bytes()], // we will create a key for the data or account
        bump,
    )]
    pub note: Account<'info, Note>, // account:Note -> account discriminator

    #[account(mut)]
    pub author: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateNote<'info> {
    #[account(
        mut,
        seeds = [b"note", author.key().as_ref(), note.title.as_bytes()],
        bump,
    )]
    pub note: Account<'info, Note>,

    pub author: Signer<'info>,
}

#[derive(Accounts)]
pub struct DeleteNote<'info> {
    #[account(
        mut,
        seeds = [b"note", author.key().as_ref(), note.title.as_bytes()],
        bump,
        close = author,
    )]
    pub note: Account<'info, Note>,

    #[account(mut)]
    pub author: Signer<'info>
}

#[account]
#[derive(InitSpace)]
pub struct Note {
    pub author: Pubkey,
    #[max_len(100)]
    pub title: String,
    #[max_len(1000)]
    pub content: String,
    pub created_at: i64, // Signed int 64 - unix
    pub last_updated: i64,
}

#[error_code]
pub enum NotesError {
    #[msg("Title cannot be longer than 100 chars")]
    TitleTooLong,
    #[msg("Content cannot be longer than 1000 chars")]
    ContentTooLong,
    #[msg("Title cannot be empty")]
    TitleEmpty,
    #[msg("Content cannot be empty")]
    ContentEmpty,
    #[msg("Unauthorized")]
    Unauthorized,
}
