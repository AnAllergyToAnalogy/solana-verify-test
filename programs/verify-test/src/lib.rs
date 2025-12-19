pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("DSieW9K3zXmTtmv6y5QBUquRiquE8HysUDxbcqgk2c44");

#[program]
pub mod verify_test {
    use super::*;

    pub fn test_ix(ctx: Context<TestIx>) -> Result<()> {
        test_ix::handler(ctx)
    }
}
