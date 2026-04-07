use crate::{NftContract, NftError, TokenRecord};

impl NftContract {
    pub fn mint(
        &mut self,
        token_id: impl Into<String>,
        owner: impl Into<String>,
        metadata_uri: Option<String>,
    ) -> Result<(), NftError> {
        let token_id = token_id.into();
        if self.tokens.contains_key(&token_id) {
            return Err(NftError::AlreadyMinted);
        }

        self.tokens.insert(
            token_id,
            TokenRecord {
                owner: owner.into(),
                approved: None,
                metadata_uri,
            },
        );
        Ok(())
    }
}
