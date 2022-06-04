// // Lấy danh sách Collections, Schemas, Templates, NFT, ...
// use crate::*;

// #[near_bindgen]
// impl NFTContract {
//     // Lấy tổng số token đang có trong contract
//     pub fn nft_total_supply(&self) -> U128 {
//         // Đếm tổng số lượng id đang có trong token_metadata_by_id
//         U128(self.token_metadata_by_id.len() as u128)
//     }

//     // Lấy thông tin 1 token dưới dạng JsonToken
//     pub fn nft_token(&self, token_id: TokenId) -> Option<JsonToken> {
//         let token = self.tokens_by_id.get(&token_id);

//         if let Some(token) = token {
//             let metadata = self.token_metadata_by_id.get(&token_id).unwrap();

//             Some(JsonToken {
//                 owner_id: token.owner_id,
//                 token_id,
//                 collection_id: token.collection_id,
//                 collection_name: token.collection_name,
//                 schema_id: token.schema_id,
//                 schema_name: token.schema_name,
//                 template_id: token.template_id,
//                 metadata,
//                 approved_account_ids: token.approved_account_ids,
//             })
//         } else {
//             None
//         }
//     }

//     // Lấy danh sách token (có pagination)
//     pub fn nft_tokens(&self, from_index: Option<U128>, limit: Option<u64>) -> Vec<JsonToken> {
//         let collection_keys = self.token_metadata_by_id.keys_as_vector();

//         let start = u128::from(from_index.unwrap_or(U128(0)));

//         // Duyệt tất cả các keys -> Trả về JsonToken
//         collection_keys
//             .iter()
//             .skip(start as usize) // Pagination
//             .take(limit.unwrap_or(0) as usize) // Pagination
//             .map(|token_id| self.nft_token(token_id.clone()).unwrap())
//             .collect()
//     }
// }
