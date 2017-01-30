use Data; //type

use huffmantree::HuffmanTree;

pub struct DataFile
{
	data: Data,
	path: String
}

impl DataFile
{
	pub fn from(data: Data, path: String) -> DataFile
	{
		DataFile
		{
			data: data,
			path: path
		}
	}

	pub fn get_path(&self) -> String
	{
		self.path.clone()
	}

	pub fn get_huffman_tree(&self) -> HuffmanTree
	{
		HuffmanTree::from_data(&self.data)
	}

	pub fn into_data(self) -> Data
	{
		self.data
	}

	pub fn get_size(&self) -> usize
	{
		self.data.len()
	}
}