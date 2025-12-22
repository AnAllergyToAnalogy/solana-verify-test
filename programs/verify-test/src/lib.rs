pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

#[allow(unused_imports)]
use solana_security_txt::security_txt;

#[cfg(not(feature = "no-entrypoint"))]
security_txt! {
    name: "Program Verify Test",
    project_url: "https://github.com/AnAllergyToAnalogy/solana-verify-test",
    contacts: "email:anallergytoanalogy@gmail.com",
    policy: "https://github.com/AnAllergyToAnalogy/solana-verify-test/blob/main/SECURITY.md",
    preferred_languages: "en",
    source_code: "https://github.com/AnAllergyToAnalogy/solana-verify-test"
}


pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("E6bPwe1P6M1QWQVvF7aH5pttrdGDRW9Kc45bEjq9e97Q");

#[program]
pub mod verify_test {
    use super::*;

    pub fn test_ix(ctx: Context<TestIx>) -> Result<()> {
        test_ix::handler(ctx)
    }
}
