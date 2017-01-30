pub type Distribution = [usize; 256];
pub type ByteWeight = (u8, usize); //-> byte, count

pub struct ByteWeightCounter
{
	counter: Distribution
}

impl ByteWeightCounter
{
	pub fn new() -> ByteWeightCounter
	{
		ByteWeightCounter
		{
			counter: [0; 256]
		}
	}

	pub fn feed(&mut self, byte: u8)
	{
		self.counter[byte as usize] += 1;
	}

	pub fn finalize(self) -> Distribution
	{
		self.counter
	}
}

pub fn to_vec(dist: Distribution) -> Vec<ByteWeight>
{
	let mut result = Vec::new();
	for byte in 0..256
	{
		let bytecount = dist[byte];
		if bytecount > 0
		{
			result.push((byte as u8, bytecount));
		}
	}
	result
}