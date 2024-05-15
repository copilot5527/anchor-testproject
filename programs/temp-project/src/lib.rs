use anchor_lang::prelude::*;

declare_id!("G3buTvmMGr4guzRftVvy7YX5s24WARiqEPg2dbo8TPSV");

#[program]
pub mod temp_project {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
