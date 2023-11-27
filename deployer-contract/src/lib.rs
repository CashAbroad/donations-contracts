#![no_std]

use num_integer::Roots;

use soroban_sdk::{
  contract, contracterror, contractimpl, contracttype, token, log,
  Address, Env, Vec, IntoVal, Val,
};

#[contracterror]
#[derive(Clone, Debug, Copy, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error{
  InvalidAuth = 1,
  KeyExpected = 2,
  ExpectedExtraValue = 3,
  InvalidAmount = 4,
  AlreadyWithdrawn = 5,
  InvalidAssociation = 6,
  InvalidTimestamp = 7,
  AlreadyInitialized = 8,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Association{
  name: Address,
  contribution: Vec<i64>,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FinalAssociation{
  name: Address,
  contribution: i128,
  has_withdrawn: bool,
}

impl Association{
  //The method to add a new contribution to a certain address
  fn add_contribution(&mut self, amount: i64){
    self.contribution.push_back(amount);
  }
  fn get_name(&self) -> &Address {
    &self.name
  }
  // a function to return the contribution of Association
  fn get_contribution(&self) -> &Vec<i64> {
    &self.contribution
  }
}

impl FinalAssociation{
  fn get_contribution(&self) -> i128 {
    self.contribution
  }
  fn set_contribution(&mut self, amount: i128){
    self.contribution = amount;
  }
}
#[contracttype]
pub enum StorageConst {
    AdminAddress,
    Associations,
    FinalAssociations,
    Deadline,
    RecipientsClaimed,
    AssetAdress,
    State,
    TotalAmount,
    ContractCallAddress,
}

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum State {
    Running = 0,
    Ended = 1,
}

/*Function to solve "the trait bound `State: TryFromVal<Env, soroban_sdk::Val>` is not satisfied
the following other types implement trait `TryFromVal<E, V>`:"*/

impl IntoVal<Env, Val> for State {
  fn into_val(&self, env: &Env) -> Val {
      (*self as u32).into_val(env)
  }
}

fn init_associations(e: &Env, associations: Vec<Address>) -> Vec<Association>{
  let mut associations_vec: Vec<Association> = Vec::new(&e);

  for association in associations.iter() {
      add_new_association(e, &mut associations_vec, &association);
  }
  associations_vec
}

fn add_new_association(e: &Env, associations: &mut Vec<Association>, name: &Address) {
  let new_assoc: Association = Association {
      name: name.clone(),
      contribution: Vec::new(&e),
  };
  associations.push_back(new_assoc);
}

fn get_associations_address(e: &Env, associations: Vec<Association>) -> Vec<Address>{
  // iterate trough this vector of associations and return the addresses as a new vector
  let mut addresses: Vec<Address> = Vec::new(&e);
  for association in associations.iter() {
      addresses.push_back(association.get_name().clone());
  }
  addresses
}

fn get_amount_per_association(e: &Env, association: &Vec<Association>, name: &Address) -> Vec<i64>{
  let mut amounts: Vec<i64> = Vec::new(&e);
  for assoc in association.iter() {
      if assoc.get_name() == name {
        amounts = assoc.get_contribution().clone();
      }
  }
  amounts
}

fn add_contribution(e: &Env, associations: &mut Vec<Association>, association: &Address, sender: &Address,amount: i64) -> Vec<Association>{
  let contract_token = e.current_contract_address();
  let amount_i128: i128 = amount as i128;
  let mut i:u32 = 0;
  for mut assoc in associations.iter() {
      if assoc.get_name() == association {
          assoc.add_contribution(amount);
          associations.set(i, assoc);
          transfer(e, sender, &contract_token, &amount_i128);
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

fn duplicate(e: &Env, associations: &Vec<FinalAssociation>, sender: &Address) -> Vec<FinalAssociation>{
  let contract_address = e.current_contract_address();
  for mut association in associations.iter() {
      let mut amount = association.get_contribution();
      transfer(e, sender, &contract_address, &amount);
      amount = amount * 2;
      association.set_contribution(amount);
  }
  associations.clone()
}

  /*
  Function to calculate the funding for each association.
  */
fn calculate_funding(e: &Env, associations: &mut Vec<Association>) -> Vec<FinalAssociation>{
  //Implement the quadratic funding for this function.
  let mut final_associations: Vec<FinalAssociation> = Vec::new(&e);
  let mut total_square_sum:i128 = 0;

  for association in associations.iter() {
      let mut sqrt_sum_for_association: i128 = 0;
      for contribution in association.get_contribution().iter() {
          sqrt_sum_for_association += (contribution as i128).sqrt();
      }
      total_square_sum += powi(sqrt_sum_for_association,2);
  }

  for association in associations.iter() {
      let total_amount: i128;
      total_amount = calculate_quadratic_funding_for_association(&association.get_contribution(), total_square_sum, get_total(&e));
      let final_assoc: FinalAssociation = FinalAssociation{
          name: association.get_name().clone(),
          contribution: total_amount,
          has_withdrawn: false,
      };
      final_associations.push_back(final_assoc);
  }
  final_associations
}
/*
Function to calculate the quadratic funding for a certain association.
*/
fn calculate_quadratic_funding_for_association(contributions: &Vec<i64>, total_square_sum: i128, total_funding: i128) -> i128 {
  let mut sqrt_sum_for_association:i128 = 0;
  for contribution in contributions.iter() {
      sqrt_sum_for_association += (contribution as i128).sqrt();
  }
  
  let project_square_sum: i128 = powi(sqrt_sum_for_association,2);
  let fraction_of_total: i128 = project_square_sum / total_square_sum;
  fraction_of_total * total_funding
}

fn powi(base: i128, exponent: i64) -> i128 {
  if exponent == 0 {
      return 1;
  }
  if exponent == 1 {
      return base;
  }

  let mut result = 1;
  let mut current_base = base;
  let mut current_exponent = exponent;

  while current_exponent > 0 {
      if current_exponent % 2 == 1 {
          result *= current_base;
      }
      current_base *= current_base;
      current_exponent /= 2;
  }

  result
}

fn withdraw(e: &Env) {
  let contract_transfer = get_contract_call_address(e);
  let total_amount = get_amount(e);
  transfer(e, &e.current_contract_address(), &contract_transfer, &total_amount);
}

fn get_ledger_timestamp(e: &Env) -> u64 {
  e.ledger().timestamp()
}

fn get_deadline(e: &Env) -> u64 {
  e.storage()
      .instance()
      .get::<_, u64>(&StorageConst::Deadline)
      .expect("not initialized yet")
}

fn get_associations(e: &Env) -> Vec<Association> {
  e.storage()
    .instance()
    .get::<_, Vec<Association>>(&StorageConst::Associations)
    .expect("not initialized yet")
}

fn get_final_associations(e: &Env) -> Vec<FinalAssociation> {
  e.storage()
    .instance()
    .get::<_, Vec<FinalAssociation>>(&StorageConst::FinalAssociations)
    .expect("not initialized yet")
}

fn get_token_address(e: &Env) -> Address {
  e.storage()
      .instance()
      .get::<_, Address>(&StorageConst::AssetAdress)
      .expect("not initialized yet")
}

fn get_admin_address(e: &Env) -> Address {
  e.storage()
      .instance()
      .get::<_, Address>(&StorageConst::AdminAddress)
      .expect("not initialized yet")
}

fn get_total(e: &Env) -> i128 {
  e.storage()
      .instance()
      .get::<_, i128>(&StorageConst::TotalAmount)
      .expect("not initialized yet")
}

fn get_state(e: &Env) -> State {
  let deadline: u64 = get_deadline(e);
  let current_timestamp: u64 = get_ledger_timestamp(e);

  if current_timestamp < deadline {
      return State::Running;
  };
  if current_timestamp >= deadline {
      return State::Ended;
  };
  State::Ended
}

fn get_recipients_claimed(e: &Env) -> bool {
  e.storage()
      .instance()
      .get::<_, bool>(&StorageConst::RecipientsClaimed)
      .expect("not initialized yet")
}

fn get_amount(e: &Env) -> i128 {
  e.storage()
      .instance()
      .get::<_, i128>(&StorageConst::TotalAmount)
      .expect("not initialized yet")
}

fn get_contract_call_address(e: &Env) -> Address {
  e.storage()
      .instance()
      .get::<_, Address>(&StorageConst::ContractCallAddress)
      .expect("not initialized yet")
}
#[contract]
pub struct VotingContract;

pub trait VotingTrait{
  /*
  Initialize the contract, it needs:
  admin: admin address for this contract, the only one who can modify.
  token_address: the token this contract will handle.
  association: a vector of addresses that will form up the associations.
  deadline: epoch timestamp 
  */
  fn init(
    env: Env,
    admin: Address,
    token_address: Address,
    associations: Vec<Address>,
    deadline: u64,
    contract_transfer: Address,
  ) -> Result<(), Error>;

  /*
  This function will be called by the admin to add a new association to the contract.
   */
  fn withdraw(
    env: Env,
    admin: Address,
  ) -> Result<(), Error>;


  fn deposit(
    env: Env,
    association: Address,
    sender: Address,
    amount: i64,
  ) -> Result<(), Error>;

  /*
  This function will be called by the admin to calculate the fundings.
  */
  fn calculate_funding(
    env: Env,
    admin: Address,
  ) -> Result<(), Error>;

  fn end_funding(
    env: Env,
    admin: Address,
  ) -> Result<(), Error>;

  // This function will be called by the admin to add a new association to the contract.
  fn add_association(
    env: Env,
    admin: Address,
    association: Address,
  ) -> Result<(), Error>;

  fn duplicate_quantities(
    env: Env,
    sender: Address,
  ) -> Result<(), Error>;
  //This function will be called by anyone to get the associations addresses.
  fn associations_addresses(
    env: Env
  ) -> Vec<Address>;
  
  //This function will be called by anyone to get the state of the contract.
  fn state(
    env: Env
  ) -> u32;

  //This function will be called by anyone to get the deadline of the contract.
  fn deadline(
    env: Env
  ) -> u64;

  //This function will be called by anyone to get the amounts per associations.
  fn associations_amounts(
    env: Env
  ) -> Vec<Association>;

  //This function will be calle by anyone to get the amount of a certain association.
  fn association_amount(
    env: Env,
    association: Address,
  ) -> Vec<i64>;

  fn total_amount(
    env: Env
  ) -> i128;
  
  fn total_final_associations(
    env: Env
  ) -> Vec<FinalAssociation>;
}

#[contractimpl]
impl VotingTrait for VotingContract {
  fn init(
    env: Env,
    admin: Address,
    token_address: Address,
    associations: Vec<Address>,
    deadline: u64,
    contract_transfer: Address,
  ) -> Result<(), Error> {
    admin.require_auth();
    if env.storage().instance().has(&StorageConst::AdminAddress) {
      log!(
        &env,
        "Something went wrong, the contract is already initizalized."
      );
      return Err(Error::AlreadyInitialized);
    }
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
    let associations_vec: Vec<Association> = init_associations(&env, associations);
    env.storage().instance().set(&StorageConst::AdminAddress, &admin);
    env.storage().instance().set(&StorageConst::AssetAdress, &token_address);
    env.storage().instance().set(&StorageConst::Deadline, &deadline);
    env.storage().instance().set(&StorageConst::Associations, &associations_vec);
    env.storage().instance().set(&StorageConst::TotalAmount, &total_amount);
    env.storage().instance().set(&StorageConst::ContractCallAddress, &contract_transfer);
    Ok(())
  }
  
  fn withdraw(
    env: Env,
    admin: Address,
  ) -> Result<(), Error> {
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
      return Err(Error::AlreadyWithdrawn);
    }
    withdraw(&env);
    env.storage().instance().set(&StorageConst::RecipientsClaimed, &true);
    Ok(())
  }

  fn calculate_funding(
    env: Env,
    admin: Address,
  ) -> Result<(), Error> {
    admin.require_auth();
    let admin_address: Address = get_admin_address(&env);
    if admin_address != admin {
      log!(
        &env,
        "Something went wrong, the admin address is not the same as the one who called the function."
      );
      return Err(Error::InvalidAuth);
    }
    let mut mutable_assoc: Vec<Association> = get_associations(&env);
    let mutable_final_assoc: Vec<FinalAssociation> = calculate_funding(&env, &mut mutable_assoc);
    env.storage().instance().set(&StorageConst::FinalAssociations, &mutable_final_assoc);
    Ok(())
  }

  fn end_funding(
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
    let final_amount = get_amount(&env);
    if final_amount < 0 {
      log!(
        &env,
        "Something went wrong, the final amount is less than 0."
      );
      return Err(Error::InvalidAmount);
    }
    let final_assoc = get_final_associations(&env);
    if final_assoc.len() > 0 {
      let deadline: u64 = env.ledger().timestamp();
      env.storage().instance().set(&StorageConst::Deadline, &deadline);
      env.storage().instance().set(&StorageConst::RecipientsClaimed, &false);
    }
   
    Ok(())
  }

  fn add_association(
    env: Env,
    admin: Address,
    association: Address,
  ) -> Result<(), Error> {
    admin.require_auth();
    let admin_address = get_admin_address(&env);
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
    let mut mutable_assoc: Vec<Association> = get_associations(&env);
    add_new_association(&env, &mut mutable_assoc, &association);
    env.storage().instance().set(&StorageConst::Associations, &mutable_assoc);
    Ok(())
  }

  fn deposit(
    env: Env,
    sender: Address,
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
    if amount <= 0 {
      log!(
        &env,
        "Something went wrong, the amount is 0 or less than 0."
      );
      return Err(Error::InvalidAmount);
    }
    sender.require_auth();
    let mut mutable_assoc: Vec<Association> = get_associations(&env);
    let mut total_amount: i128 = get_total(&env);
    total_amount += amount as i128;
    //add to storage
    let new_contributions:Vec<Association> = add_contribution(&env ,&mut mutable_assoc, &association, &sender,amount);
    env.storage().instance().set(&StorageConst::TotalAmount, &total_amount);
    env.storage().instance().set(&StorageConst::Associations, &new_contributions);
    Ok(())
  }
  //add a function to get the total amount.

  fn duplicate_quantities(
    env: Env,
    sender: Address,
  ) -> Result<(), Error>{
    sender.require_auth();
    let mut final_assocs = get_final_associations(&env);
    let assoc_duplicate = duplicate(&env, &mut final_assocs, &sender);
    env.storage().instance().set(&StorageConst::FinalAssociations, &assoc_duplicate);

    let mut amount = get_amount(&env);
    amount = amount * 2;
    env.storage().instance().set(&StorageConst::TotalAmount, &amount);
    Ok(())
  }
  fn associations_addresses(
    env: Env
  ) -> Vec<Address> {
    let associations: Vec<Association> = get_associations(&env);
    let addresses: Vec<Address> = get_associations_address(&env, associations);
    addresses
  }

  fn state(
    env: Env
  ) -> u32 {
    get_state(&env) as u32
  }

  fn deadline(
    env: Env
  ) -> u64 {
    get_deadline(&env)
  }

  fn associations_amounts(
    env: Env
  ) -> Vec<Association>{
    get_associations(&env)
  }

  fn association_amount(
    env: Env,
    association: Address,
  ) -> Vec<i64> {
    let associations: Vec<Association> = get_associations(&env);
    let assoc_amount: Vec<i64> = get_amount_per_association(&env, &associations, &association);
    assoc_amount
  }

  fn total_amount(
    env: Env
  ) -> i128 {
    get_amount(&env)
  }

  fn total_final_associations(
    env: Env
  ) -> Vec<FinalAssociation> {
    get_final_associations(&env)
  }
}
