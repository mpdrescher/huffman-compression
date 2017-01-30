use Data; //type
use FlattenedData; //type

use std::ops::BitAnd;
use std::ops::BitOrAssign;

const BYTE_ONE: u8 = 0b00000001;

//returns FlattenedData as Vec<u8>
//flattens every byte and appends the bits
//-> byte stream to bit stream
pub fn flatten_bytes(data: Data) -> FlattenedData 
{
	let mut result = Vec::new();
	for byte in data
	{
		result.append(&mut flatten_byte(byte));
	}
	result
}

//returns FlattenedData as Vec<u8>
//converts a byte into a series of bits
//0b10101010 -> vec!(1,0,1,0,1,0,1,0)
pub fn flatten_byte(byte: u8) -> FlattenedData
{
	let mut result = Vec::new();
	for shift_rev in 0..8
	{
		//count from higher bit to lower bit
		let shift = 7-shift_rev;
		let bitmask = BYTE_ONE << shift;//(iter, result) -> 1,128; 2,64; 3,32 ...
		if byte.bitand(bitmask) == 0
		{
			result.push(0);
		}
		else 
		{
		    result.push(1);
		}
	}
	result
}

//converts flattened data to a bytestream
//if a sequence doesn't fit evenly in bytes (len % 8 != 0)
//the remaining bits are filled up with 1s
pub fn get_bytes(bits: FlattenedData) -> Data
{
	let one = BYTE_ONE;
	let mut result = Vec::new();
	for byte in bits.as_slice().chunks(8)
	{
		if byte.len() == 8
		{
			result.push(into_byte(byte))
		}
		else 
		{
			let mut new_byte = Vec::new();
			for bit in byte.iter()
			{
				new_byte.push(*bit);
			}
			while new_byte.len() < 8
			{
				new_byte.push(one);
			}
			result.push(into_byte(new_byte.as_slice()));
		}
	}
	result
}

//returns u8 / byte
//converts a series of bit into a byte
//vec!(1,0,1,0,1,0,1,0) -> 0b10101010
fn into_byte(bits: &[u8]) -> u8
{
	let mut result = 0;
	let mut shift = 7;
	for bit in bits.iter()
	{
		result.bitor_assign((BYTE_ONE.bitand(bit)) << shift);
		shift -= 1;
	}
	result
}

pub fn u64_to_u8(num: u64) -> Vec<u8>
{
	let mut shift = 64;
	let mut bits: FlattenedData = Vec::new();
	while shift > 0
	{
		shift -= 1;
		match num.bitand(1 << shift)
		{
			0 => {
				bits.push(0);
			},
			_ => {
				bits.push(1);
			}
		};
	}
	get_bytes(bits)
}

//make sure you call this function with a vec of size 8
pub fn u8_to_u64(data: Data) -> u64
{
	let bytesize: u64 = 256;
	let mut result = 0;
	let mut data_iter = data.into_iter();
	for i in 0..8
	{
		let irev = 7-i;
		let number = bytesize.pow(irev as u32) * (data_iter.next().unwrap() as u64);
		result += number;
	}
	result
}