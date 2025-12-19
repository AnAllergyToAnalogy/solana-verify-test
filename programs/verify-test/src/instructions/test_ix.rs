use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct TestIx {}

pub fn handler(ctx: Context<TestIx>) -> Result<()> {
    /// Test ix description.
    msg!("This is the test ix: {:?}", ctx.program_id);
    Ok(())
}
