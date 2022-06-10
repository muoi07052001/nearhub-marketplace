# How to build and test this contract

1. Create nft_contract_id -> deploy out/nft-contract.wasm to nft_contract_id (`nearhub-nft.duongnh.testnet`)

   ```
   - ./build.sh
   - cd ..
   - near create-account nearhub-nft.duongnh.testnet --masterAccount duongnh.testnet --initialBalance 20
   - near deploy --wasmFile out/nft-contract.wasm --accountId nearhub-nft.duongnh.testnet --initFunction new_default_metadata --initArgs '{"owner_id": "duongnh.testnet"}'
   ```

---

### Init contract

```
near call nearhub-nft.duongnh.testnet new_default_metadata '{"owner_id": "duongnh.testnet"}' --accountId duongnh.testnet
```

---

### Collection

2. Create a Collection:

   ```
   near call nearhub-nft.duongnh.testnet create_collection '{"collection_name": "Game", "market_fee": 5.0, "data": {"name": "Zuong game", "img": ""}}' --deposit 0.1 --accountId duongnh.testnet
   ```

3. View the total number of Collections of the Contract:

   ```
   near view nearhub-nft.duongnh.testnet collection_total_supply
   ```

4. View the total number of Collections for a account: `duongnh.testnet`

   ```
   near view nearhub-nft.duongnh.testnet collection_supply_for_owner '{"account_id": "duongnh.testnet"}'
   ```

5. View list of Collections (with pagination) of the Contract: (`from_index`: String, `limit`: integer)

   ```
   near view nearhub-nft.duongnh.testnet get_all_collections '{"from_index": "0", "limit": 10}'
   ```

6. View list of Collections (with pagination) of 1 account: `duongnh.testnet` (`from_index`: String, `limit`: integer)

   ```
   near view nearhub-nft.duongnh.testnet get_all_collections_for_owner '{"account_id": "duongnh.testnet", "from_index": "0", "limit": 10}'
   ```

7. View list of Collections by Collection Name (All the Collections that has Name contains the `search_string`)
   ```
   near view nearhub-nft.duongnh.testnet get_collections_by_name '{"search_string": "GA"}'
   ```

---

### Schema

9. Create a Schema:

   ```
   near call nearhub-nft.duongnh.testnet create_schema '{"collection_name": "Game", "schema_name": "Weapon", "schema_format": [{"attribute_name": "name", "attribute_type": "string"}]}' --deposit 0.1 --accountId duongnh.testnet
   ```

10. View the total number of Schemas of the Contract:

```
near view nearhub-nft.duongnh.testnet schema_total_supply
```

11. View the total number of Schemas of a Collection:

```
near view nearhub-nft.duongnh.testnet schema_supply_by_collection '{"collection_name": "Game"}'
```

12. View list of Schemas (with pagination) of the Contract: (`from_index`: String, `limit`: integer)

    ```
    near view nearhub-nft.duongnh.testnet get_all_schemas '{"from_index": "0", "limit": 10}'
    ```

13. View list of Schemas (with pagination) of 1 Collection: `duongnh.testnet` (`from_index`: String, `limit`: integer)

    ```
    near view nearhub-nft.duongnh.testnet get_all_schemas_by_collection '{"collection_name": "Game", "from_index": "0", "limit": 10}'
    ```

14. View list of Schemas by Schema Name (All the Schemas that has Name contains the `search_string`)
    ```
    near view nearhub-nft.duongnh.testnet get_schemas_by_name '{"search_string": "Wea"}'
    ```

---

### Template

15. Create a Template:

```
near call nearhub-nft.duongnh.testnet create_template '{"collection_name": "Game", "schema_id": 0, "transferable": true, "burnable": true, "max_supply": 10, "issued_supply": 3, "immutable_data": {"name": "Lightsaber", "img": "", "extra_immutable_data": "{\"attack\": \"10\"}"}}' --deposit 0.1 --accountId duongnh.testnet
```

16. View the total number of Templates of the Contract:

```
near view nearhub-nft.duongnh.testnet template_total_supply
```

17. View the total number of Templates of a Collection:

```
near view nearhub-nft.duongnh.testnet template_supply_by_collection '{"collection_name": "Game"}'
```

18. View list of Templates (with pagination) of the Contract: (`from_index`: String, `limit`: integer)

```
near view nearhub-nft.duongnh.testnet get_all_templates '{"from_index": "0", "limit": 10}'
```

19. View list of Templates (with pagination) of 1 Collection: `duongnh.testnet` (`from_index`: String, `limit`: integer)

```
near view nearhub-nft.duongnh.testnet get_all_templates_by_collection '{"collection_name": "Game", "from_index": "0", "limit": 10}'
```

20. Get Template by Template Id

```
near view nearhub-nft.duongnh.testnet get_template_by_id '{"template_id": 0}'
```

---

## NFT

21. Mint an NFT:

```
near call nearhub-nft.duongnh.testnet nft_mint '{"collection_name": "Game", "schema_id": 0, "template_id": 0, "metadata":{"title": "ZUONG SABER", "description": "Zuong saber", "media": "https://bafkreibhsxpr4qbjqure75n6q6ywulozmb6e2tnedloq6v5em24f6nhmgm.ipfs.dweb.link/"}, "receiver_id": "duongnh.testnet"}' --deposit 0.1 --accountId duongnh.testnet
```

22. View the token (NFT) we just minted

```
near view nearhub-nft.duongnh.testnet nft_token '{"token_id": 0}'
```

23. View total NFT supply in the NFTContract

```
near view nearhub-nft.duongnh.testnet nft_total_supply
```

24. View total NFT supply of an account `duongnh.testnet`

```
near view nearhub-nft.duongnh.testnet nft_supply_for_owner '{"account_id": "duongnh.testnet"}'
```

25. View list of NFTs of an account `duongnh.testnet`

```
near view nearhub-nft.duongnh.testnet nft_tokens_for_owner '{"account_id": "duongnh.testnet", "from_index": "0", "limit": 10}'
```

---

### Approval

7. Transfer NFT from `duongnh.tesnet` to `zuongnh.testnet`

   ```
   near call nearhub-nft.duongnh.testnet nft_transfer '{"receiver_id": "zuongnh.testnet", "token_id": 0, "approval_id": 0}' --accountId duongnh.testnet --depositYocto 1
   ```

8. `zuongnh.testnet` add approval for `duongnh.testnet` to transfer his token

   ```
   near call nearhub-nft.duongnh.testnet nft_approve '{"token_id": 0, "account_id": "duongnh.testnet"}' --deposit 0.01 --accountId zuongnh.testnet
   ```

9. View the token we just add approval

   ```
   near view nearhub-nft.duongnh.testnet nft_token '{"token_id": 0}'
   ```

10. Use `duongnh.testnet` to transfer token back to `duongnh.testnet` (although token's owner is `zuongnh.testnet`, but since `duongnh.testnet` has been approved to transfer the NFT so it can transfer the NFT)

```
near call nearhub-nft.duongnh.testnet nft_transfer '{"receiver_id": "duongnh.testnet", "token_id": 0, "approval_id": 0}' --accountId duongnh.testnet --depositYocto 1
```

---

## Lootbox

26. Create a Lootbox: (Belongs to a Collection)

```
near call nearhub-nft.duongnh.testnet create_lootbox '{"lootbox_name": "Random Weapon", "description": "This will gives a lot of weapons", "collection_name": "Game", "unlock_time": 0, "display_data": "", "config": [{"total_odds": 100, "outcomes": [{"template_id": 0, "odds": 50}, {"template_id": 1, "odds": 100}]}, {"total_odds": 100, "outcomes": [{"template_id": 0, "odds": 30}, {"template_id": 1, "odds": 100}]}]}' --deposit 0.1 --accountId duongnh.testnet

near call nearhub-nft.duongnh.testnet create_lootbox '{"lootbox_name": "Random Weapon", "description": "This will gives a lot of weapons", "collection_name": "Game", "unlock_time": 0, "display_data": "", "config": [{"total_odds": 100, "outcomes": [{"template_id": 0, "odds": 50}, {"template_id": 1, "odds": 100}]}, {"total_odds": 100, "outcomes": [{"template_id": 0, "odds": 30}, {"template_id": 1, "odds": 100}]}, {"total_odds": 200, "outcomes": [{"template_id": 1, "odds": 30}, {"template_id": 2, "odds": 100}, {"template_id": 3, "odds": 200}]}]}' --deposit 0.1 --accountId duongnh.testnet
```

27. View the total number of Lootboxes of the Contract:

```
near view nearhub-nft.duongnh.testnet lootbox_total_supply
```

28. View the total number of Lootboxes of a Collection:

```
near view nearhub-nft.duongnh.testnet lootbox_supply_by_collection '{"collection_name": "Game"}'
```

29. View list of Lootboxes (with pagination) of the Contract: (`from_index`: String, `limit`: integer)

```
near view nearhub-nft.duongnh.testnet get_all_lootboxes '{"from_index": "0", "limit": 10}'
```

30. View list of Lootboxes (with pagination) of 1 Collection: `Game` (`from_index`: String, `limit`: integer)

```
near view nearhub-nft.duongnh.testnet get_all_lootboxes_by_collection '{"collection_name": "Game", "from_index": "0", "limit": 10}'
```

31. Unbox a Lootbox

```
near call nearhub-nft.duongnh.testnet unbox_lootbox '{"lootbox_id": 0}' --deposit 0.1 --accountId duongnh.testnet
```

---

### Drop Sale

32. Create a Drop Sale for some Templates

```
near call nearhub-nft.duongnh.testnet create_drop '{"collection_name": "Game", "template_ids": [0, 1], "price": 1.50, "price_type": "NEAR", "is_public": true, "max_supply": 10, "account_limit": 1, "account_limit_cooldown": "2", "start_time": "0", "end_time": "0", "display_data": ""}' --deposit 0.1 --accountId duongnh.testnet
```

16. View the total number of Drop Sales of the Contract:

```
near view nearhub-nft.duongnh.testnet drop_total_supply
```

17. View the total number of Drop Sales of a Collection:

```
near view nearhub-nft.duongnh.testnet drop_supply_by_collection '{"collection_name": "Game"}'
```

18. View list of Drop Sales (with pagination) of the Contract: (`from_index`: String, `limit`: integer)

```
near view nearhub-nft.duongnh.testnet get_all_drops '{"from_index": "0", "limit": 10}'
```

19. View list of Drop Sales (with pagination) of 1 Collection: `duongnh.testnet` (`from_index`: String, `limit`: integer)

```
near view nearhub-nft.duongnh.testnet get_all_drops_by_collection '{"collection_name": "Game", "from_index": "0", "limit": 10}'
```

20. Get Drop Sale by Drop Id

```
near view nearhub-nft.duongnh.testnet get_drop_by_id '{"drop_id": 0}'
```


33. Let all the users of the Contract to purchase this Drop Sale
```
near call nearhub-nft.duongnh.testnet drop_add_approval_for_all '{"drop_id": 0}' --accountId duongnh.testnet
```

34. Add 1 account to Drop Sale's Whitelist -> They can purchase the Drop Sale
```
near call nearhub-nft.duongnh.testnet drop_add_whitelist_account '{"drop_id": 0, "account_id": "duongnh.testnet"}' --accountId duongnh.testnet
```

35. Check if an account in the Drop Sale's Whitelist
```
near view nearhub-nft.duongnh.testnet drop_is_approved '{"drop_id": 0, "approved_account_id": "duongnh.testnet", "approval_id": 0}'
```

36. Remove an account from the Drop Sale's Whitelist
```
near call nearhub-nft.duongnh.testnet drop_revoke '{"drop_id": 0, "account_id": "duongnh.testnet"}' --accountId duongnh.testnet --depositYocto 1
```

37. Remove all accounts from the Drop Sale's Whitelist
```
near call nearhub-nft.duongnh.testnet drop_revoke_all '{"drop_id": 0}' --accountId duongnh.testnet --depositYocto 1
```