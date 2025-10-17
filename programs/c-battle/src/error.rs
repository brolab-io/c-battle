use anchor_lang::prelude::*;

#[error_code]
pub enum BattleErrorCode {
    Unauthorized,
    #[msg("Custom error message")]
    CustomError,
}
