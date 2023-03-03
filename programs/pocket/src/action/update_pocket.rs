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
        constraint = pocket.owner == signer.key() @ PocketError::OnlyOwner,
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
                assert_eq!(pocket.is_able_to_restart(), true, "COULD_NOT_RESTART_POCKET");
            },

            PocketStatus::Paused => {
                assert_eq!(pocket.is_able_to_pause(), true, "COULD_NOT_PAUSE_POCKET");
            },

            PocketStatus::Closed => {
                assert_eq!(pocket.is_able_to_close(), true, "COULD_NOT_CLOSE_POCKET");
            },

            PocketStatus::Withdrawn => {
                assert_eq!(1, 0, "INVALID_INPUT");
            }
        }

        pocket.status = params.status;

        pocket_emit!(
            PocketUpdated {
                actor: self.signer.key(),
                pocket_address: pocket.key(),
                status: pocket.status,
                memo: String::from("USER_UPDATED_POCKET")
            }
        );

        Ok(())
    }
}