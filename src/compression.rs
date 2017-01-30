use std::collections::BTreeMap;

use datafile::DataFile;
use FlattenedData;
use Data;

use huffmantree;
use huffmantree::HuffmanTree;
use byteconvert;

pub fn compress(file: DataFile) -> (FlattenedData, HuffmanTree)
{
	let tree = file.get_huffman_tree();
	let lookup_map = tree.get_lookup_map();
	let data = file.into_data();
	let mut result: FlattenedData = Vec::new();
	for byte in data
	{
		result.append(&mut lookup_map.get(&byte).unwrap().clone());
	}
	(byteconvert::get_bytes(result), tree)
}

//parsing the data and passing it to real_decompress()
pub fn decompress(data: Data) -> Option<(Data)>//Option<Data>
{
	//reading byte length
	let mut byte_len_vec = Vec::new();
	let mut data_iter = data.into_iter();
	for _ in 0..8
	{
		byte_len_vec.push(match data_iter.next()
		{
			Some(v) => v,
			None => return None
		});
	}
	let byte_len = byteconvert::u8_to_u64(byte_len_vec);
	
	let lookup_map = match huffmantree::bytes_to_lookup_map(&mut data_iter)
	{
		Some(v) => v,
		None => return None
	};

	let remaining_data = data_iter.collect::<Vec<u8>>();

	Some(real_decompress(byte_len, lookup_map, remaining_data))
}

//the above compress function is the public interface and mostly needed for parsing
fn real_decompress(byte_len: u64, lookup_map: BTreeMap<FlattenedData, u8>, remaining_data: Data) -> Vec<u8>
{
	let mut result = Vec::new();
	let flattened_data = byteconvert::flatten_bytes(remaining_data);
	let mut buffer = Vec::new();
	for elem in flattened_data
	{
		buffer.push(elem);
		if lookup_map.contains_key(&buffer)
		{
			let ch = lookup_map.get(&buffer).unwrap().clone();
			result.push(ch.clone());
			buffer.clear();
		}
		if (result.len() as u64) == byte_len
		{
			break;
		}
	}
	result
}