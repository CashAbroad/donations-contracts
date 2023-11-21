#![no_std]

use soroban_sdk::{
  contract, contracterror, contractimpl, contracttype, token, log,
  Address, Env, Vec,
};

#[contracterror]
#[derive(Clone, Debug, Copy, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error{
  InvalidAuth = 1,
  KeyExpected = 2,
  ExpectedExtraValue = 3,
  InvalidAmount = 4,
  InvalidTimestamp = 5,
  InvalidAssociation = 6,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FinalAssociation{
  name: Address,
  contribution: i128,
  has_withdrawn: bool,
}

impl FinalAssociation{
  fn get_name(&self) -> &Address {
    &self.name
  }
  // a function to return the contribution of Association
  fn get_contribution(&self) -> &i128 {
    &self.contribution
  }
  fn set_contribution(&mut self, contribution: i128){
    self.contribution = contribution;
  }
  fn get_has_withdrawn(&self) -> &bool {
    &self.has_withdrawn
  }
  fn set_has_withdrawn(&mut self, has_withdrawn: bool){
    self.has_withdrawn = has_withdrawn;
  }
}

#[contracttype]
pub enum StorageConst {
  AdminAddress,
  FinalAssociations,
  Deadline,
  RecipientsClaimed,
  AssetAdress,
  ClaimMonth,
  TotalAmount,
  DeadlineWithdraw,
}

fn init_associations(e: &Env, associations: Vec<Address>) -> Vec<FinalAssociation>{
  let mut associations_vec: Vec<FinalAssociation> = Vec::new(&e);

  for association in associations.iter() {
      add_new_association(&mut associations_vec, &association);
  }
  associations_vec
}

fn add_new_association(associations: &mut Vec<FinalAssociation>, name: &Address) {
  let new_assoc: FinalAssociation = FinalAssociation {
      name: name.clone(),
      contribution: 0,
      has_withdrawn: false,
  };
  associations.push_back(new_assoc);
}

fn add_contribution(associations: &mut Vec<FinalAssociation>, association: &Address, amount: i64) -> Vec<FinalAssociation>{
  let amount_i128: i128 = amount as i128;
  let mut i:u32 = 0;
  for mut assoc in associations.iter() {
      if assoc.get_name() == association {
          let amount_assoc = assoc.get_contribution();
          let new_amount = amount_assoc + amount_i128;
          assoc.set_contribution(new_amount);
          associations.set(i, assoc);
      }
      i += 1;
  }
  associations.clone()
}

fn withdraw(e: &Env, associations: &mut Vec<FinalAssociation>) {
  let claim_month = get_claim_month(&e);
  if claim_month == 12{
    return;
  }
  for mut association in associations.iter(){
    let has_withdrawn: bool = *association.get_has_withdrawn();
    if has_withdrawn == true {
      continue;
    }
    let mut total_amount: i128;
    total_amount  = *association.get_contribution() as i128;
    total_amount = total_amount / 12;
    transfer(e, &e.current_contract_address(), association.get_name(), &total_amount);
    association.set_has_withdrawn(true);
  }

}

fn reset_withdraw(associations: &mut Vec<FinalAssociation>) -> Vec<FinalAssociation> {
  let mut i = 0;
  for mut association in associations.iter(){
    let has_withdrawn: bool = *association.get_has_withdrawn();
    if has_withdrawn == true {
      association.set_has_withdrawn(false);
      associations.set(i, association);
    }
    i += 1;
  }
  associations.clone()
}

fn transfer(e: &Env, from: &Address, to: &Address, amount: &i128) {
  let token_contract_id: &Address = &get_token_address(e);
  let client = token::Client::new(e, token_contract_id);
  client.transfer(from, to, amount);
}

fn get_admin_address(e: &Env) -> Address {
  e.storage()
      .instance()
      .get::<_, Address>(&StorageConst::AdminAddress)
      .expect("not initialized yet")
}

fn get_token_address(e: &Env) -> Address {
  e.storage()
      .instance()
      .get::<_, Address>(&StorageConst::AssetAdress)
      .expect("not initialized yet")
}

fn get_ledger_timestamp(e: &Env) -> u64 {
  e.ledger().timestamp()
}

fn get_associations(e: &Env) -> Vec<FinalAssociation> {
  e.storage()
    .instance()
    .get::<_, Vec<FinalAssociation>>(&StorageConst::FinalAssociations)
    .expect("not initialized yet")
}

fn get_deadline(e: &Env) -> u64 {
  e.storage()
      .instance()
      .get::<_, u64>(&StorageConst::Deadline)
      .expect("not initialized yet")
}

fn get_total(e: &Env) -> i128 {
  e.storage()
      .instance()
      .get::<_, i128>(&StorageConst::TotalAmount)
      .expect("not initialized yet")
}

fn get_recipients_claimed(e: &Env) -> bool {
  e.storage()
      .instance()
      .get::<_, bool>(&StorageConst::RecipientsClaimed)
      .expect("not initialized yet")
}

fn get_final_associations(e: &Env) -> Vec<FinalAssociation> {
  e.storage()
    .instance()
    .get::<_, Vec<FinalAssociation>>(&StorageConst::FinalAssociations)
    .expect("not initialized yet")
}

fn get_deadline_withdraw(e: &Env) -> u64 {
  e.storage()
      .instance()
      .get::<_, u64>(&StorageConst::DeadlineWithdraw)
      .expect("not initialized yet")
}

fn get_claim_month(e: &Env) -> u32 {
  e.storage()
      .instance()
      .get::<_, u32>(&StorageConst::ClaimMonth)
      .expect("not initialized yet")
}

#[contract]
pub struct DistributionContract;

pub trait DistributionTrait{
  fn init( 
    env: Env,
    admin: Address,
    token_address: Address,
    associations: Vec<Address>,
    deadline: u64,
  ) -> Result<(), Error>;

  fn add_association(
    env: Env,
    association: Address,
    admin: Address,
  ) -> Result<(), Error>;

  fn deposit(
    env: Env,
    association: Address,
    amount: i64,
  ) -> Result<(), Error>;

  fn reset_deadline(
    env: Env,
    admin: Address,
  ) -> Result<(), Error>;

  fn withdraw(
    env: Env,
    admin: Address,
  ) -> Result<(), Error>;

  fn associations(
    env: Env,
  ) -> Vec<FinalAssociation>;

  fn total(
    env: Env,
   ) -> i128;
  
  fn deadline(
    env: Env,
  ) -> u64;
  
}

#[contractimpl]
impl DistributionTrait for DistributionContract{
  fn init( 
    env: Env,
    admin: Address,
    token_address: Address,
    associations: Vec<Address>,
    deadline: u64,
  ) -> Result<(), Error>{
    admin.require_auth();
    let act_ledger = get_ledger_timestamp(&env);
    if deadline < act_ledger{
      log!(
        &env,
        "Something went wrong, the deadline is before the current deadline."
      );
      return Err(Error::InvalidTimestamp);
    }
    if associations.len() < 1 {
      log!(
        &env,
        "Something went wrong, the length of the associations is less than 1."
      );
      return Err(Error::InvalidAssociation);
    }

    let total_amount: i128 = 0;
    let month: u32 = 0;
    let associations_vec: Vec<FinalAssociation> = init_associations(&env, associations);
    env.storage().instance().set(&StorageConst::AdminAddress, &admin);
    env.storage().instance().set(&StorageConst::AssetAdress, &token_address);
    env.storage().instance().set(&StorageConst::Deadline, &deadline);
    env.storage().instance().set(&StorageConst::FinalAssociations, &associations_vec);
    env.storage().instance().set(&StorageConst::TotalAmount, &total_amount);
    env.storage().instance().set(&StorageConst::DeadlineWithdraw, &deadline);
    env.storage().instance().set(&StorageConst::ClaimMonth, &month);
    Ok(())
  }

  fn add_association(
    env: Env,
    association: Address,
    admin: Address,
  ) -> Result<(), Error>{
    admin.require_auth();
    let admin_address: Address = get_admin_address(&env);
    if admin_address != admin {
      log!(
        &env,
        "Something went wrong, the admin address is not the same as the one who called the function."
      );
      return Err(Error::InvalidAuth);
    }
    let deadline = get_deadline(&env);
    if deadline < get_ledger_timestamp(&env){
      log!(
        &env,
        "Something went wrong, the deadline is before the current deadline."
      );
      return Err(Error::InvalidTimestamp);
    }
    let mut associations: Vec<FinalAssociation> = get_associations(&env);
    add_new_association( &mut associations, &association);
    env.storage().instance().set(&StorageConst::FinalAssociations, &associations);
    Ok(())
  }

  fn deposit(
    env: Env,
    association: Address,
    amount: i64,
  ) -> Result<(), Error>{
    let deadline = get_deadline(&env);
    if deadline < get_ledger_timestamp(&env){
      log!(
        &env,
        "Something went wrong, the deadline is before the current deadline."
      );
      return Err(Error::InvalidTimestamp);
    }
    let mut mutable_assoc: Vec<FinalAssociation> = get_associations(&env);
    let mut total_amount: i128 = get_total(&env);
    total_amount += amount as i128;
    //add to storage
    let new_contributions:Vec<FinalAssociation> = add_contribution(&mut mutable_assoc, &association, amount);
    env.storage().instance().set(&StorageConst::TotalAmount, &total_amount);
    env.storage().instance().set(&StorageConst::FinalAssociations, &new_contributions);
    Ok(())
  }

  fn withdraw(
    env: Env,
    admin: Address,
  ) -> Result<(), Error>{
    admin.require_auth();
    let admin_address: Address = get_admin_address(&env);
    if admin_address != admin {
      log!(
        &env,
        "Something went wrong, the admin address is not the same as the one who called the function."
      );
      return Err(Error::InvalidAuth);
    }
    let recipients_claimed: bool = get_recipients_claimed(&env);
    if recipients_claimed{
      log!(
        &env,
        "Something went wrong, the recipients have already claimed their funds."
      );
      return Err(Error::ExpectedExtraValue);
    }
    let mut mutable_assoc: Vec<FinalAssociation> = get_final_associations(&env);
    let mut claim_month = get_claim_month(&env);
    claim_month += 1;
    withdraw(&env, &mut mutable_assoc);
    env.storage().instance().set(&StorageConst::RecipientsClaimed, &true);
    env.storage().instance().set(&StorageConst::ClaimMonth, &claim_month);
    Ok(())
  }

  fn reset_deadline(
    env: Env,
    admin: Address,
  ) -> Result<(), Error>{
    admin.require_auth();
    let admin_address: Address = get_admin_address(&env);
    if admin_address != admin {
      log!(
        &env,
        "Something went wrong, the admin address is not the same as the one who called the function."
      );
      return Err(Error::InvalidAuth);
    }
    let recipients_claimed: bool = get_recipients_claimed(&env);
    let mut deadline_withdraw: u64 = get_deadline_withdraw(&env);
    
    if recipients_claimed && deadline_withdraw > get_ledger_timestamp(&env){
      log!(
        &env,
        "Something went wrong, the recipients have not claimed their funds yet."
      );
      return Err(Error::ExpectedExtraValue);
    }
    let final_assoc = reset_withdraw(&mut get_final_associations(&env));
    deadline_withdraw += 2629743;
    env.storage().instance().set(&StorageConst::FinalAssociations, &final_assoc);
    env.storage().instance().set(&StorageConst::RecipientsClaimed, &false);
    env.storage().instance().set(&StorageConst::DeadlineWithdraw, &deadline_withdraw);
    Ok(())
  }

  fn associations(
    env: Env,
  ) -> Vec<FinalAssociation>{
    get_associations(&env)
  }

  fn total(
    env: Env,
   ) -> i128{
    get_total(&env)
   }
  
  fn deadline(
    env: Env,
  ) -> u64{
    get_deadline(&env)
  }
}