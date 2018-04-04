use core;
use super::interface;
use super::super::alloc::SliceWrapper;
use super::histogram::ContextType;
use super::input_pair::InputReference;
use super::constants::{kSigned3BitContextLookup, kUTF8ContextLookup};
use super::interface::LiteralPredictionModeNibble;
pub trait IRInterpreter {
    fn inc_local_byte_offset(&mut self, inc:usize);
    fn get_stride(&self, local_byte_offset: usize) -> u8;
    fn local_byte_offset(&self) -> usize;
    fn update_block_type(&mut self, new_type: u8, new_stride: u8);
    fn block_type(&self) -> u8;
    fn literal_data_at_offset(&self, index: usize) -> u8;
    fn literal_context_map(&self) -> &[u8];
    fn prediction_mode(&self) -> ::interface::LiteralPredictionModeNibble;
    fn update_cost(&mut self, stride_prior: u8, selected_bits:u8, cm_prior: usize, literal: u8);
}


pub fn push_base<'a,
                 Interpreter: IRInterpreter,
                 Cb: FnMut(&[interface::Command<InputReference>])>(xself: &mut Interpreter,
                                                                   val: interface::Command<InputReference<'a>>,
                                                                   callback: &mut Cb) {
        match val {
           interface::Command::BlockSwitchCommand(_) |
           interface::Command::BlockSwitchDistance(_) |
           interface::Command::PredictionMode(_) => {}
           interface::Command::Copy(ref copy) => {
               xself.inc_local_byte_offset(copy.num_bytes as usize);
           },
           interface::Command::Dict(ref dict) => {
               xself.inc_local_byte_offset(dict.final_size as usize);
           },
            interface::Command::BlockSwitchLiteral(block_type) => xself.update_block_type(block_type.block_type(),
                                                                                          block_type.stride()),
           interface::Command::Literal(ref lit) => {
               let stride = xself.get_stride(xself.local_byte_offset()) as usize;
               let mut priors= [0u8; 8];
               for poffset in 0..core::cmp::max((stride & 7) + 1, 2) {
                   if xself.local_byte_offset() > poffset {
                       let input_offset = xself.local_byte_offset() - poffset -  1;
                       priors[7 - poffset] = xself.literal_data_at_offset(input_offset);
                   }
               }
               let mut cur = 0usize;
               for literal in lit.data.slice().iter() {
                   let (huffman_table_index, selected_bits) = compute_huffman_table_index_for_context_map(priors[(cur + 7)&7], priors[(cur + 6) &7], xself.literal_context_map(), xself.prediction_mode(), xself.block_type());
                   xself.update_cost(priors[(cur + 7 - stride) & 7], selected_bits, huffman_table_index, *literal);
                   priors[cur & 7] = *literal;
                   cur += 1;
                   cur &= 7;
               }
               xself.inc_local_byte_offset(lit.data.slice().len());
           }
        }
        let cbval = [val];
        callback(&cbval[..]);        
    }


// not sure why this fails
//impl<'a> interface::CommandProcessor<'a> for IRInterpreter {
//    fn push<Cb: FnMut(&[interface::Command<InputReference>])>(&mut self,
//                                                              val: interface::Command<InputReference<'a>>,
//                                                              callback: &mut Cb) {
//        push_base(self, val, callback)
//    }
//}



fn compute_huffman_table_index_for_context_map (
    prev_byte: u8,
    prev_prev_byte: u8,
    literal_context_map: &[u8],//interface::PredictionModeContextMap<SliceType>,
    prediction_mode: LiteralPredictionModeNibble,
    block_type: u8,
) -> (usize, u8) {
    let prior = Context(prev_byte, prev_prev_byte, prediction_mode.to_context_enum().unwrap());
    assert!(prior < 64);
    let context_map_index = ((block_type as usize)<< 6) | prior as usize;
    if context_map_index < literal_context_map.len() {
        (literal_context_map[context_map_index] as usize, prior)
    } else {
        (prior as usize, prior)
    }
}

pub fn Context(p1: u8, p2: u8, mode: ContextType) -> u8 {
  match mode {
    ContextType::CONTEXT_LSB6 => {
      return (p1 as (i32) & 0x3fi32) as (u8);
    }
    ContextType::CONTEXT_MSB6 => {
      return (p1 as (i32) >> 2i32) as (u8);
    }
    ContextType::CONTEXT_UTF8 => {
      return (kUTF8ContextLookup[p1 as (usize)] as (i32) |
              kUTF8ContextLookup[(p2 as (i32) + 256i32) as (usize)] as (i32)) as (u8);
    }
    ContextType::CONTEXT_SIGNED => {
      return ((kSigned3BitContextLookup[p1 as (usize)] as (i32) << 3i32) +
              kSigned3BitContextLookup[p2 as (usize)] as (i32)) as (u8);
    }
  }
  //  0i32 as (u8)
}
