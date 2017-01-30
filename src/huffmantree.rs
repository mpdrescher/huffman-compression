use Data; //type

use std::collections::BTreeMap;
use std::vec::IntoIter;

use byteweight::ByteWeightCounter;
use byteweight::ByteWeight;
use byteweight;

use FlattenedData;

pub struct HuffmanTree
{
	root: HuffmanNode
}

impl HuffmanTree
{
	//huffman tree generation from a byte vector (Data)
	pub fn from_data(data: &Data) -> HuffmanTree
	{
		//count the occurence count of the individual bytes
		let mut byte_weight_counter = ByteWeightCounter::new();
		for byte in data
		{
			byte_weight_counter.feed(*byte);
		}

		//sort the weight distribution and transform into HuffmanNode vec
		let mut sorted_dist = byteweight::to_vec(byte_weight_counter.finalize());
		sorted_dist.sort_by(|a, b| b.1.cmp(&a.1)); //if you vec.pop() you get the smallest
		let mut sorted_nodes = Vec::new();
		for elem in sorted_dist
		{
			sorted_nodes.push(HuffmanNode::with_value(elem));
		}
		if sorted_nodes.len() == 0
		{
			//assure that there is at least one element in the vec
			//this is why i can use 3 unwraps until the function end
			sorted_nodes.push(HuffmanNode::new()); 
		}
		if sorted_nodes.len() == 1
		{

		}
		
		//merge the two lowest nodes until there is only one root node
		while sorted_nodes.len() > 1
		{
			let node1 = sorted_nodes.pop().unwrap();
			let node2 = sorted_nodes.pop().unwrap();
			let mut new_node = HuffmanNode::new();
			new_node.set_right(node1);
			new_node.set_left(node2);
			sorted_nodes.push(new_node);
			//re-sort
			sorted_nodes.sort_by(|a, b| b.weight.cmp(&a.weight));
		}

		HuffmanTree
		{
			root: sorted_nodes.pop().unwrap()
		}
	}

	pub fn get_lookup_map(&self) -> BTreeMap<u8, FlattenedData>
	{
		let mut lookup_map = BTreeMap::new();
		self.root.fill_lookup_map(&mut lookup_map, Vec::new());
		lookup_map
	}
}

//THE LEFT PATH IS A ZERO, THE RIGHT PATH IS A ONE!!
#[derive(Debug)]
pub struct HuffmanNode
{
	weight: usize, //occurences of the nodes below or this node in the compression data
	value: Option<u8>,
	left: Option<Box<HuffmanNode>>,
	right: Option<Box<HuffmanNode>>
}

impl HuffmanNode
{
	fn new() -> HuffmanNode
	{
		HuffmanNode
		{
			weight: 0,
			value: None,
			left: None,
			right: None
		}
	}

	fn with_value(byteweight: ByteWeight) -> HuffmanNode
	{
		HuffmanNode
		{
			weight: byteweight.1,
			value: Some(byteweight.0),
			left: None,
			right: None
		}
	}

	fn set_left(&mut self, node: HuffmanNode)
	{
		self.weight += node.weight;
		self.left = Some(Box::new(node));
	}

	fn set_right(&mut self, node: HuffmanNode)
	{
		self.weight += node.weight;
		self.right = Some(Box::new(node));
	}

	//return a map in which the sequence of a byte can be searched
	pub fn fill_lookup_map(&self, mut map: &mut BTreeMap<u8, FlattenedData>, path: FlattenedData)
	{
		match self.value
		{
			Some(v) => {
				map.insert(v, path.clone());
				return;
			},
			None => {
				match self.right
				{
					Some(ref v) => {
						let mut right_path = path.clone();
						right_path.push(1);
						v.fill_lookup_map(&mut map, right_path)
					},
					None => {}
				};
				match self.left
				{
					Some(ref v) => {
						let mut left_path = path;
						left_path.push(0);
						v.fill_lookup_map(&mut map, left_path)
					},
					None => {}
				};
			}
		}
	}
}

//represent a lookup map with bytes
//chain of:
//byte, path0, path1, pathn, 2, [...] byteN, path0, path1, pathn, 3 -> End
pub fn lookup_map_to_bytes(map: BTreeMap<u8, FlattenedData>) -> Data
{
	let mut result = Vec::new();
	for mut entry in map
	{
		result.push(entry.0);
		result.append(&mut entry.1);//TODO: memory expensive
		result.push(2);
	}
	if result.len() > 1
	{
		let _ = result.pop();
		result.push(3);
	}
	result
}

//parse a series of bytes produced by lookup_map_to_bytes(..)
#[allow(dead_code)]
pub fn bytes_to_lookup_map(data_iter: &mut IntoIter<u8>) -> Option<BTreeMap<FlattenedData, u8>>
{
	let mut result = BTreeMap::new();
	'l1: loop
	{
		let byte = match data_iter.next()
		{
			Some(v) => v,
			None => return None
		};
		let mut path = Vec::new();
		//can't seem to remove warning :O
		let mut cur_byte: u8 = 4;
		'l2: loop
		{
			cur_byte = match data_iter.next()
			{
				Some(v) => v,
				None => return None
			};

			if cur_byte == 2//byte end
			{
				break 'l2;
			}
			if cur_byte == 3 //end
			{
				//bug i had to track down, if this line is removed the line 9 lines after this one
				//is not called, resulting in an incomplete lookup map (buffer isn't pushed)
				//and wrong decompression
				result.insert(path, byte);
				break 'l1;
			}
			else 
			{
			    path.push(cur_byte);
			}
		}
		result.insert(path, byte);
	}
	Some(result)
}