# How to build and test this contract

1. Create nft_contract_id -> deploy out/nft-contract.wasm to nft_contract_id (`nearhub.nft.duongnh.testnet`)

   ```
   - ./build.sh
   - cd ..
   - near create-account nearhub.nft.duongnh.testnet --masterAccount nft.duongnh.testnet --initialBalance 20
   - near deploy --wasmFile out/nft-contract.wasm --accountId nearhub.nft.duongnh.testnet --initFunction new_default_metadata --initArgs '{"owner_id": "duongnh.testnet"}'
   ```

---

### Init contract

   ```
   near call dev-1654311163374-36226532432210 new_default_metadata '{"owner_id": "duongnh.testnet"}' --accountId duongnh.testnet
   ```

### Collection

2. Create a Collection:

   ```
   near call dev-1654311163374-36226532432210 create_collection '{"collection_name": "Game", "market_fee": 5.0, "data": {"name": "Zuong game", "img": ""}}' --accountId duongnh.testnet
   ```

3. View the total number of Collections of the Contract:

   ```
   near view dev-1654311163374-36226532432210 collection_total_supply
   ```

4. View the total number of Collections for a account: `duongnh.testnet`

   ```
   near view dev-1654311163374-36226532432210 collection_supply_for_owner '{"account_id": "duongnh.testnet"}'
   ```

5. View list of Collections (with pagination) of the Contract: (`from_index`: String, `limit`: integer)

   ```
   near view dev-1654311163374-36226532432210 get_all_collections '{"from_index": "0", "limit": 10}'
   ```

6. View list of Collections (with pagination) of 1 account: `duongnh.testnet` (`from_index`: String, `limit`: integer)

   ```
   near view dev-1654311163374-36226532432210 get_all_collections_for_owner '{"account_id": "duongnh.testnet", "from_index": "0", "limit": 10}'
   ```

7. Search Collection by collection id
   ```
   near view dev-1654311163374-36226532432210 get_collection_by_id '{"collection_id": 0}'
   ```

8. View list of Collections by Collection Name (All the Collections that has Name contains the `search_string`)
    ```
    near view dev-1654311163374-36226532432210 get_collections_by_name '{"search_string": "GA"}'
    ```

---

### Schema

9. Create a Schema:

   ```
   near call dev-1654311163374-36226532432210 create_schema '{"collection_id": 0, "schema_name": "Weapon", "schema_format": [{"attribute_name": "name", "attribute_type": "string"}]}' --accountId duongnh.testnet
   ```

10. View the total number of Schemas of the Contract:

   ```
   near view dev-1654311163374-36226532432210 schema_total_supply
   ```

11. View the total number of Schemas of a Collection:

   ```
   near view dev-1654311163374-36226532432210 schema_supply_by_collection '{"collection_id": 0}'
   ```

12. View list of Schemas (with pagination) of the Contract: (`from_index`: String, `limit`: integer)

    ```
    near view dev-1654311163374-36226532432210 get_all_schemas '{"from_index": "0", "limit": 10}'
    ```

13. View list of Schemas (with pagination) of 1 Collection: `duongnh.testnet` (`from_index`: String, `limit`: integer)

    ```
    near view dev-1654311163374-36226532432210 get_all_schemas_by_collection '{"collection_id": 0, "from_index": "0", "limit": 10}'
    ```

14. View list of Schemas by Schema Name (All the Schemas that has Name contains the `search_string`)
    ```
    near view dev-1654311163374-36226532432210 get_schemas_by_name '{"search_string": "Wea"}'
    ```

---

### Template

15. Create a Template:

   ```
   near call dev-1654311163374-36226532432210 create_template '{"collection_id": 0, "schema_id": 0, "transferable": true, "burnable": true, "max_supply": 10, "issued_supply": 3, "immutable_data": {"name": "Lightsaber", "img": "", "extra_immutable_data": "{\"attack\": \"10\"}"}}' --accountId duongnh.testnet
   ```

16. View the total number of Templates of the Contract:

   ```
   near view dev-1654311163374-36226532432210 template_total_supply
   ```

17. View the total number of Templates of a Collection:

   ```
   near view dev-1654311163374-36226532432210 template_supply_by_collection '{"collection_id": 0}'
   ```

18. View list of Templates (with pagination) of the Contract: (`from_index`: String, `limit`: integer)

    ```
    near view dev-1654311163374-36226532432210 get_all_templates '{"from_index": "0", "limit": 10}'
    ```

19. View list of Templates (with pagination) of 1 Collection: `duongnh.testnet` (`from_index`: String, `limit`: integer)

    ```
    near view dev-1654311163374-36226532432210 get_all_templates_by_collection '{"collection_id": 0, "from_index": "0", "limit": 10}'
    ```

20. Get Template by Template Id
    ```
    near view dev-1654311163374-36226532432210 get_template_by_id '{"template_id": 0}'
    ```

## NFT
21. Mint an NFT:
   ```
   near call dev-1654311163374-36226532432210 nft_mint '{"collection_id": 0, "schema_id": 0, "template_id": 0, "metadata":{"title": "ZUONG SABER", "description": "Zuong saber", "media": "https://bafkreibhsxpr4qbjqure75n6q6ywulozmb6e2tnedloq6v5em24f6nhmgm.ipfs.dweb.link/"}, "receiver_id": "duongnh.testnet"}' --deposit 0.1 --accountId duongnh.testnet
   ```

22. View the token (NFT) we just minted

   ```
   near view dev-1654311163374-36226532432210 nft_token '{"token_id": 0}'
   ```

23. View total NFT supply in the NFTContract

   ```
   near view dev-1654311163374-36226532432210 nft_total_supply
   ```

24. View total NFT supply of an account `duongnh.testnet`

   ```
   near view dev-1654311163374-36226532432210 nft_supply_for_owner '{"account_id": "duongnh.testnet"}'
   ```

25. View list of NFTs of an account `duongnh.testnet`

   ```
   near view dev-1654311163374-36226532432210 nft_tokens_for_owner '{"account_id": "duongnh.testnet", "from_index": "0", "limit": 10}'
   ```