use super::*;
use std::collections::HashSet;

#[derive(Debug)]

pub enum BlockValidationErr
{
    MisMatchedIndex,
    InvalidHash,
    AChronologicalTimestamp;
    MisMatchedPreviousHash,
    InvalidGenesisBlockFormat,
    InvalidInput,
    InsufficientInputValue,
    InvalidCoinbaseTransaction,
}

pub struct Blockchain{
    pub blocks: Vec<Block>,
    unspent_outputs: HashSet<Hash>,
}

impl Blockchain
{
    pub fn new() -> self{
        Blockchain
        {
            blocks: Vec![],
            unspent_outputs: HashSet::new(),

        }
    }

    pub fn update_with_block(&mut self, block:Block) -> Result<(), BlockValidationErr>
    {
        let i = self.blocks.len();

        if block.index!= i as u32 
        {
            return Err(BlockValidationErr:MisMatchedIndex);
        }
        else if(!block:: check_difficulty(&block.hash(), block.difficulty))
        {
            return Err(BlockValidationErr::InvalidHash);
        }
        else if (i!=0)
        {
            //Non Genesis block
            let prev_block = &self.blocks[i-1];
            if(block.timestamp<= prev_block.timestamp){
                return Err(BlockValidationErr::AChronologicalTimestamp);
            }
            else if(block.prev_block_hash!=prev_block.hash)
            {
                return Err(BlockValidationErr::MisMatchedPreviousHash);
            }

        }
        else //Genesis block
        {
            if(blokc.prev_block_hash!= vec![0; 32])
            {
                return Err(BlockValidationErr:: InvalidGenesisBlockFormat);
            }
        }
    }
}