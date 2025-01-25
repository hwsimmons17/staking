use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult};

use crate::error::MplProjectNameError;
use crate::instruction::accounts::UpdateAccounts;
use crate::instruction::CreateArgs;
use crate::state::MyAccount;

pub fn update<'a>(accounts: &'a [AccountInfo<'a>], args: CreateArgs) -> ProgramResult {
    // Accounts.
    let ctx = UpdateAccounts::context(accounts)?;

    let mut my_account = MyAccount::load(ctx.accounts.address)?;

    // Guards.
    if my_account.authority != *ctx.accounts.authority.key {
        return Err(MplProjectNameError::InvalidAuthority.into());
    };

    my_account.data.field1 = args.arg1;
    my_account.data.field2 = args.arg2;

    my_account.save(ctx.accounts.address)
}
