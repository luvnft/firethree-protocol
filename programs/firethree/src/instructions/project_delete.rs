use crate::errors::*;
use crate::Project;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct ProjectDelete<'info> {
    #[account(mut)]
    pub multisig: Signer<'info>,

    #[account(mut, close = multisig)]
    pub project: Account<'info, Project>,
}

impl ProjectDelete<'_> {
    fn validate(&self) -> Result<()> {
        Ok(())
    }

    #[access_control(ctx.accounts.validate())]
    pub fn project_delete(ctx: Context<ProjectDelete>) -> Result<()> {
        let project: &mut Account<Project> = &mut ctx.accounts.project;

        if project.authority != *ctx.accounts.multisig.key {
            return Err(FirethreeError::UnauthorizedToDeleteProject.into());
        }

        Ok(())
    }
}
