## Add a new network for the futurenet
soroban config network add --global futurenet \
  --rpc-url https://rpc-futurenet.stellar.org:443 \
  --network-passphrase "Test SDF Future Network ; October 2022"

## Or Add a new network for the testnet
soroban config network add --global testnet \
  --rpc-url https://soroban-testnet.stellar.org:443 \
  --network-passphrase "Test SDF Network ; September 2015"

## add a new address with the private key 
soroban config identity add --global --secret-key $(name) ##(Here you can add the cashabroad, and sender address)

##Then you got to write the private key
S....2IFME

## build the contract.
soroban contract build

## Deploy the contract to the futurenet.
soroban contract deploy --wasm target/wasm32-unknown-unknown/release/deployer_contract.wasm --source cashabroad --network testnet

## Invoke the init function.
soroban contract invoke --id ___ --source cashabroad --network testnet -- init --admin GCFHD6NQ7DB75I4T2LQPUT2W336KRUMZYVOYIHIVF63IBUL4EZRNYOZK --token_address CB64D3G7SM2RTH6JSGG34DDTFTQ5CFDKVDZJZSODMCX4NJ2HV2KN7OHT --deadline 1694649599 --associations '[ "GBXKU3C7KKKBJAH6FCXV6BXZ4ZZXUWP37XM3HNS32FXM2LCRQV4HLUBK", "GBRADTPI7RK666QPUWP5TPVE27TCZERRHBUF6OMX3LV4ZGZN7Z5USNEM" ]'

## Invoke the add_association function.
soroban contract invoke --id ___ --source cashabroad --network testnet -- add_association --admin GCFHD6NQ7DB75I4T2LQPUT2W336KRUMZYVOYIHIVF63IBUL4EZRNYOZK --association GAFIA4AE62FMOZ642IKVSQUM3XF7AREWNOBNIY4FTNBKTJPPJIKKLVUV

## Invoke the deposit function.}
soroban contract invoke --id ____ --source cashabroad --network testnet -- deposit --association  GBXKU3C7KKKBJAH6FCXV6BXZ4ZZXUWP37XM3HNS32FXM2LCRQV4HLUBK --amount 200000000

## Invoke the withdraw function.
soroban contract invoke --id ___ --source cashabroad --network testnet -- withdraw --admin GCFHD6NQ7DB75I4T2LQPUT2W336KRUMZYVOYIHIVF63IBUL4EZRNYOZK 

##Invoke the reset deadline function.
soroban contract invoke --id ___ --source cashabroad --network testnet -- reset_deadline --admin GCFHD6NQ7DB75I4T2LQPUT2W336KRUMZYVOYIHIVF63IBUL4EZRNYOZK 

## Invoke the get associations function.
soroban contract invoke --id ___ --source cashabroad --network testnet -- get_associations 

## Invoke the get total function.
soroban contract invoke --id ___ --source cashabroad --network testnet -- total