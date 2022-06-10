# How to build and test this contract

1. Create nft_contract_id -> deploy out/nft-contract.wasm to nft_contract_id (`nearhub-market.nft.duongnh.testnet`)

   ```
   - ./build.sh
   - cd ..
   - near create-account nearhub-market.duongnh.testnet --masterAccount duongnh.testnet --initialBalance 20
   - near deploy --wasmFile out/market-contract.wasm --accountId nearhub-market.duongnh.testnet --initFunction new --initArgs '{"owner_id": "duongnh.testnet"}'
   ```

---

### Init contract

```
near call nearhub-market.duongnh.testnet new '{"owner_id": "duongnh.testnet"}' --accountId duongnh.testnet
```

### Buy NFTs

3. Deposit into Market Contract to cover storage

   ```
   near call nearhub-market.duongnh.testnet storage_deposit '{"account_id": "duongnh.testnet"}' --accountId duongnh.testnet --deposit 0.1
   ```

4. Call approve to transfer token (`zuongnh.testnet` gives approve to Market Contract: `nearhub-market.duongnh.testnet` with `price = 1 NEAR`)

   ```
   near call nearhub-nft.duongnh.testnet nft_approve '{"token_id": 0, "account_id": "nearhub-market.duongnh.testnet", "msg": "{\"sale_conditions\": \"1000000000000000000000000\"}"}' --deposit 0.01 --accountId duongnh.testnet
   ```

5. Update price of a NFT

   ```
   near call nearhub-market.duongnh.testnet update_price '{"nft_contract_id": "nearhub-nft.duongnh.testnet", "token_id": 0, "price": "5000000000000000000000000"}' --accountId duongnh.testnet --depositYocto 1
   ```

6. Offer buy an NFT

   ```
   near call nearhub-market.duongnh.testnet offer '{"nft_contract_id": "nearhub-nft.duongnh.testnet", "token_id": 0}' --accountId duongnh.testnet --deposit 0.01
   ```

### Enumeration

2. View total supply in `nearhub-market.duongnh.testnet`

   ```
   near view nearhub-market.duongnh.testnet get_supply_sales
   ```

3. View total supply for an account `duongnh.testnet`

   ```
   near view nearhub-market.duongnh.testnet get_supply_by_owner_id '{"account_id": "duongnh.testnet"}'
   ```

4. Get list Sales information on Market (Have pagination)

   ```
   near view nearhub-market.duongnh.testnet get_sales '{"from_index": "0", "limit": 10}'
   ```

5. Get list Sales information of an account `duongnh.testnet` (Have pagination)

   ```
   near view nearhub-market.duongnh.testnet get_sales_by_owner_id '{"from_index": "0", "limit": 10}'
   ```

6. Get list Sales information of a market (contract_id) `nearhub-market.duongnh.testnet` (Have pagination)
   ```
   near view nearhub-market.duongnh.testnet get_sales_by_contract_id '{"contract_id": "nearhub-nft.duongnh.testnet", "from_index": "0", "limit": 10}'
   ```
