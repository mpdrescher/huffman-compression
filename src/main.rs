use std::env;
use std::io::Error as IOError;
use std::io::Read;
use std::io::Write;
use std::fs::File;

mod datafile;
mod huffmantree;
mod byteweight;
mod compression;
mod byteconvert;

use datafile::DataFile;
use huffmantree::HuffmanTree;

pub const APP_NAME: &'static str = "huffman";

type Compress = bool;
type Filepaths = Vec<String>;
pub type Data = Vec<u8>; //simple sequence of bytes
pub type FlattenedData = Vec<u8>; //sequence of 0/1

fn main() 
{
	//parse arguments
	let (compress, filepaths) = match parse_args()
	{
		Some(v) => v,
		None => {
			print_help();
			return;
		}
	};

	//read file content
	let files: Vec<DataFile> = match read_files(filepaths)
	{
		Ok(v) => v,
		Err(e) => {
			println!("IO error: {}", e);
			return;
		}
	};

	if compress
	{
		for file in files
		{
			let filepath = file.get_path();
			let new_filepath = get_compressed_file_name(filepath.clone());
			let original_length = file.get_size();
			println!("compressing '{}'", file.get_path());
			let (compressed_data, huffman_tree) = compression::compress(file);
			println!("writing '{}'", new_filepath);
			match save_compressed_file(original_length, compressed_data, huffman_tree, filepath.clone())
			{
				Ok(_) => {},
				Err(e) => {
					println!("IO error: '{}'", e);
				}
			}
		}
	}
	else 
	{
	    for file in files
	    {
	    	let filepath = file.get_path();
	    	let old_filepath = get_decompressed_file_name(filepath.clone());
	    	println!("decompressing '{}'", filepath);
	    	let decompressed_data = match compression::decompress(file.into_data())
	    	{
	    		Some(v) => v,
	    		None => {
	    			println!("error while decompressing. incorrect file format?");
	    			break;
	    		}
	    	};
	    	println!("writing '{}'", old_filepath);
	    	match save_decompressed_file(decompressed_data, old_filepath)
	    	{
	    		Ok(_) => {},
	    		Err(e) => {
	    			println!("IO error: '{}'", e);
	    		}
	    	}
	    }
	}
}

//file structure:
//-first 4 bits -> u64 representing the number of bytes that are stored
//-huffman tree
//-data
fn save_compressed_file(original_length: usize, mut data: Data, tree: HuffmanTree, filepath: String) -> Result<usize, IOError>
{
	let mut content = Vec::new();
	content.append(&mut byteconvert::u64_to_u8(original_length as u64));

	let lookup_map = tree.get_lookup_map();
	content.append(&mut huffmantree::lookup_map_to_bytes(lookup_map));

	content.append(&mut data);
	
	let mut file = try!(File::create(get_compressed_file_name(filepath)));
	try!(file.write_all(content.as_slice()));
	Ok(0)
}

fn save_decompressed_file(data: Data, filepath: String) -> Result<usize, IOError>
{
	let mut file = try!(File::create(filepath));
	try!(file.write_all(data.as_slice()));
	Ok(0)
}

//Data: Vec<u8>, Error: IO Error
//read all bytes of all given files
fn read_files(paths: Filepaths) -> Result<Vec<DataFile>, IOError>
{
	let mut result: Vec<DataFile> = Vec::new();
	for path in paths
	{
		let mut data: Data = Vec::new();
		//try to open the file, error when it doesn't exist
		let mut file = match File::open(path.clone())
		{
			Ok(v) => v,
			Err(e) => {
				print!("error on file: '{}' -> ", path);
				return Err(e);
			}
		};
		//try to read the file, error on read error
		match file.read_to_end(&mut data)
		{
			Ok(_) => {
				println!("reading file: '{}'", path);
			},
			Err(e) => {
				print!("error on file: '{}' -> ", path);
				return Err(e)
			}
		};
		let datafile = DataFile::from(data, path.clone());
		result.push(datafile);
	}
	Ok(result)
}

//returns: (Compress, Filepaths) as (bool, Vec<String>)
//reads commandline arguments
fn parse_args() -> Option<(Compress, Filepaths)>
{
	let mut args = env::args();
	//skip the execution path, which is always the first arg
	//unwrap because one arg is always given
	let _ = args.next().expect("what happened?"); 
	//check for first argument -> compress/decompress
	let compress_arg = match args.next()
	{
		Some(v) => v,
		None => return None
	};
	let mut compress: bool = false;
	if compress_arg == "compress"
	{
		compress = true;
	}
	else if compress_arg != "decompress"
	{
		println!("unknown subcommand: {}\n", compress_arg);
		return None;
	}
	let mut filepaths = Vec::new();
	for arg in args
	{
		filepaths.push(arg);
	}
	if filepaths.len() == 0
	{
		return None;
	}
	Some((compress, filepaths))
}

fn print_help()
{
	println!(">> {} compression tool <<", APP_NAME);
	println!("");
	println!("USAGE:");
	println!("{} <compress|decompress> <File1> [File2] [FileN]", APP_NAME);
}

//adds file extension
fn get_compressed_file_name(mut str: String) -> String
{
	str.push_str(".comp");
	str
}

fn get_decompressed_file_name(mut str: String) -> String
{
	if str.ends_with(".comp")
	{
		return str.replace(".comp", "");
	}
	else 
	{
	    str.push_str(".decomp");
	    return str;
	}
}