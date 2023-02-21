use crate::*;

#[derive(AnchorSerialize, AnchorDeserialize, Default, Clone, Debug, PartialEq)]
pub struct UpdatePocketParams {
    status: PocketStatus
}

#[derive(Accounts)]
pub struct UpdatePocketContext<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds = [POCKET_SEED, pocket.id.as_bytes().as_ref()],
        bump = pocket.bump,
    )]
    pub pocket: Account<'info, Pocket>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,

    #[account(address = sysvar::rent::ID)]
    pub rent: Sysvar<'info, Rent>,
}

impl<'info> UpdatePocketContext<'info> {
    pub fn execute(&mut self, params: UpdatePocketParams) -> Result<()> {
        let pocket = &mut self.pocket;

        match params.status {
            PocketStatus::Active => {
                assert_eq!(pocket.is_able_to_restart(), true, "Cannot restart the pocket");
            },

            PocketStatus::Paused => {
                assert_eq!(pocket.is_able_to_pause(), true, "Cannot pause the pocket");
            },

            PocketStatus::Closed => {
                assert_eq!(pocket.is_able_to_close(), true, "Cannot close the pocket");
            },

            PocketStatus::Withdrawn => {
                assert_eq!(1, 0, "Invalid input");
            }
        }

        pocket.status = params.status;
        Ok(())
    }
}