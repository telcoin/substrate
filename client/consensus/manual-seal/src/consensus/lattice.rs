// This is the consensus provider passed to Manual Seal for PreRuntime digests

use core::marker::PhantomData;
use std::sync::Arc;
use sp_api::{ProvideRuntimeApi, TransactionFor};
use sp_blockchain::HeaderMetadata;
use sp_runtime::{traits::Block as BlockT, DigestItem, Digest, ConsensusEngineId};
use crate::ConsensusDataProvider;
use sc_client_api::{AuxStore, UsageProvider, HeaderBackend};
use codec::{Encode, Decode};
use sp_lattice::{BlockCategoryInherentData, LATTICE_ENGINE_ID};

#[derive(Encode, Decode)]
pub enum Category {
    Cat1,
    Cat2,
}

pub struct LatticeConsensusDataProvider<C> {
    _client: Arc<C>,
}

impl<C> LatticeConsensusDataProvider<C> {
    pub fn new(_client: Arc<C>) -> Self {
        Self { _client }
    }
}

impl <B, C> ConsensusDataProvider<B> for LatticeConsensusDataProvider<C>
where
    B: BlockT,
    C: ProvideRuntimeApi<B> + Send + Sync,
{
	type Transaction = TransactionFor<C, B>;

    type Proof = ();

    fn create_digest(&self, _parent: &<B as BlockT>::Header, inherents: &sp_inherents::InherentData) -> Result<sp_runtime::Digest, crate::Error> {
        // let category = "Category1";
        let category = inherents.category_inherent_data()?;
        let digest = DigestItem::PreRuntime(LATTICE_ENGINE_ID, category.encode());
        Ok(Digest { logs: vec![digest] })
    }

    fn append_block_import(
		&self,
		_parent: &<B as BlockT>::Header,
		_params: &mut sc_consensus::BlockImportParams<B, Self::Transaction>,
		_inherents: &sp_inherents::InherentData,
		_proof: Self::Proof,
	) -> Result<(), crate::Error> {
        Ok(())
    }
}
