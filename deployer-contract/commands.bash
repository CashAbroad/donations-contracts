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
soroban contract deploy --wasm /Users/alberto/Documents/Cash-abroad/soroban-contracts/target/wasm32-unknown-unknown/release/deployer_contract.wasm --source cashabroad --network testnet

## Invoke the init function.
soroban contract invoke --id CDQIB6DDVBIPH2VK5LLUCSN3HM2L2Q4TNSRGPMD4ZFQOB4YLLKE4NTLQ --source cashabroad --network testnet -- init --admin cashabroad --token_address CB64D3G7SM2RTH6JSGG34DDTFTQ5CFDKVDZJZSODMCX4NJ2HV2KN7OHT --deadline 1701028905 --associations '[ "GBXKU3C7KKKBJAH6FCXV6BXZ4ZZXUWP37XM3HNS32FXM2LCRQV4HLUBK", "GBRADTPI7RK666QPUWP5TPVE27TCZERRHBUF6OMX3LV4ZGZN7Z5USNEM" ]'

## Calculate funding 
soroban contract invoke --id CDQIB6DDVBIPH2VK5LLUCSN3HM2L2Q4TNSRGPMD4ZFQOB4YLLKE4NTLQ --source cashabroad --network testnet -- calculate_funding --admin cashabroad

##Invoke the withdraw function
soroban contract invoke --id CDQIB6DDVBIPH2VK5LLUCSN3HM2L2Q4TNSRGPMD4ZFQOB4YLLKE4NTLQ --source cashabroad --network testnet -- withdraw --admin cashabroad

soroban contract invoke --id CDQIB6DDVBIPH2VK5LLUCSN3HM2L2Q4TNSRGPMD4ZFQOB4YLLKE4NTLQ --source cashabroad --network testnet -- end_funding --admin cashabroad

## invoke the add_association function
soroban contract invoke --id CDQIB6DDVBIPH2VK5LLUCSN3HM2L2Q4TNSRGPMD4ZFQOB4YLLKE4NTLQ --source cashabroad --network testnet -- add_association --admin cashabroad --association GB4XTGTAZFH57VXWQETJ5RDVLUCQ7QN4SKELZTMWBU6TB6YQZFVQWFLA

## Add funds to an asociation
soroban contract invoke --id CDQIB6DDVBIPH2VK5LLUCSN3HM2L2Q4TNSRGPMD4ZFQOB4YLLKE4NTLQ --source sender --network testnet -- deposit --sender sender --association GB4XTGTAZFH57VXWQETJ5RDVLUCQ7QN4SKELZTMWBU6TB6YQZFVQWFLA --amount 1000

## get the state of the contract: 0 means running and 1 means ended
soroban contract invoke --id CDQIB6DDVBIPH2VK5LLUCSN3HM2L2Q4TNSRGPMD4ZFQOB4YLLKE4NTLQ --source cashabroad --network testnet -- state  

## Get the associations
soroban contract invoke --id CDQIB6DDVBIPH2VK5LLUCSN3HM2L2Q4TNSRGPMD4ZFQOB4YLLKE4NTLQ --source cashabroad --network testnet -- associations_addresses  

## Get the association balance
soroban contract invoke --id CDQIB6DDVBIPH2VK5LLUCSN3HM2L2Q4TNSRGPMD4ZFQOB4YLLKE4NTLQ --source cashabroad --network testnet -- association_amount --association GB4XTGTAZFH57VXWQETJ5RDVLUCQ7QN4SKELZTMWBU6TB6YQZFVQWFLA

## Get the association balances
soroban contract invoke --id CDQIB6DDVBIPH2VK5LLUCSN3HM2L2Q4TNSRGPMD4ZFQOB4YLLKE4NTLQ --source cashabroad --network testnet -- associations_amounts
 
 ## Get the total amount
soroban contract invoke --id CDQIB6DDVBIPH2VK5LLUCSN3HM2L2Q4TNSRGPMD4ZFQOB4YLLKE4NTLQ --source cashabroad --network testnet -- total_amount  

soroban contract invoke --id CDQIB6DDVBIPH2VK5LLUCSN3HM2L2Q4TNSRGPMD4ZFQOB4YLLKE4NTLQ --source cashabroad --network testnet -- total_final_associations