// This is the consensus provider passed to Manual Seal for PreRuntime digests

use core::marker::PhantomData;
use sp_api::{ProvideRuntimeApi, TransactionFor};
use sp_blockchain::HeaderMetadata;
use sp_runtime::{traits::Block as BlockT, DigestItem, Digest};
use sc_consensus_manual_seal::ConsensusDataProvider;
use sc_client_api::{AuxStore, UsageProvider, HeaderBackend};
use codec::{Encode, Decode};

#[derive(Encode, Decode)]
pub enum Category {
    Cat1,
    Cat2,
}

pub struct LatticeConsensusDataProvider<B, C, P> {
	// phantom data for required generics
    _phantom: PhantomData<(B, C, P)>,
}

impl<B, C, P> LatticeConsensusDataProvider<B, C, P>
where
    B: BlockT,
    C: AuxStore + ProvideRuntimeApi<B> + UsageProvider<B>,
{
    pub fn new() -> Self {
        Self { _phantom: PhantomData }
    }
}

impl <B, C, P> ConsensusDataProvider<B> for LatticeConsensusDataProvider<B, C, P>
where
    B: BlockT,
    C: AuxStore
		+ HeaderBackend<B>
		+ HeaderMetadata<B, Error = sp_blockchain::Error>
		+ UsageProvider<B>
		+ ProvideRuntimeApi<B>,
	P: Send + Sync,
{

    type Transaction = TransactionFor<C, B>;

    type Proof = P;

    fn create_digest(&self, _parent: &<B as BlockT>::Header, inherents: &sp_inherents::InherentData) -> Result<sp_runtime::Digest, sc_consensus_manual_seal::Error> {
        // let category = "Category1";
        let category = inherents.category_inherent_data();
        let digest = DigestItem::PreRuntime(*b"tel0", category.encode());
        Ok(Digest { logs: vec![digest] })
    }

    fn append_block_import(
		&self,
		_parent: &<B as BlockT>::Header,
		_params: &mut sc_consensus::BlockImportParams<B, Self::Transaction>,
		_inherents: &sp_inherents::InherentData,
		_proof: Self::Proof,
	) -> Result<(), sc_consensus_manual_seal::Error> {
        Ok(())
    }
}
