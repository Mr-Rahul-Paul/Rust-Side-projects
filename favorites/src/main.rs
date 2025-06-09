use anchor_lang::prelude::*;

declare_id!("DzT9SQ7kUnD5pmHubXqE4D1QcNamw2FWLxz8nrkpqJ3R");
// will be filled automatically
pub const ANCHOR_DISCRIMINATOR_SIZE: usize = 8  ; 
// 8bytes + whatever we are saving 

#[program] // this makes anchor set each fn as a Solana instruction  
pub mod favorites {
    use super::*  ; 

    pub fn set_favorites(
        context: Context<SetFavorites>,
        number: u64 ,
        color: String ,
        hobbies: Vec<String>,
    ) -> Result <()> {
        // same as console log
        msg!("hello or greetings {}", context.program_id);
        let user_public_key = context.accounts.user.key();
        msg!("user key = {user_public_key} number = {number} color = {color} , hobbies = {hobbies:?}");

        context.accounts.favorites.set_inner(Favorites {
            number, 
            color,
            hobbies
        });

        Ok(()) 
    }
}

#[account]
#[derive(InitSpace)] //total space used by all items inside 
pub struct Favorites {
    pub number : u64, 

    #[max_len(50)] // need to speicfy each angle differently 
    pub color : String ,

    #[max_len(5 ,50)]
    pub hobbies: Vec<String>, 
}
// wen the call setFavorites fucntion , they need to speicfy the accounts
// list of acc that they are gonna modify on the blockchain 


// the lifetime is saying , the item will live for lifetime of a Solana account info object
#[derive(Accounts)]
pub struct SetFavorites<'info> { // struct for accounts for Fav instruction handler
// its tradition is to name strcut name the same as the fucntion itself
    #[account(mut)] // the person who signs the instructions to set fav will pay to store info 
    pub user : Signer<'info>,

    #[account(
        init_if_needed , 
        payer = user, 
        space = ANCHOR_DISCRIMINATOR_SIZE + Favorites::INIT_SPACE,
        seeds = [b"favorites", user.key().as_ref()],
        bump  
    )] 
    pub favorites : Account<'info , Favorites>,
    // its a token progam ? 
    pub system_program: Program<'info, System>,
}
