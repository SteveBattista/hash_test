use std::time::Instant;
use ring::digest::{SHA1_FOR_LEGACY_USE_ONLY, SHA256, SHA384, SHA512, SHA512_256};
use ring::digest::Context;
use blake3::Hasher;
use data_encoding::HEXUPPER;
use std::ffi::OsStr;


use std::fs::File;

use std::io::{ Read};

use anyhow::Result;

#[derive(Clone)]
pub enum HasherEnum {
    Blake3Hasher (Hasher),
    SHADigest ( Context),
}

impl HasherEnum {
    pub fn new( hash_type : &str)-> Self{
        match hash_type {
            "blake3" => HasherEnum::Blake3Hasher(blake3::Hasher::new()),
            "128" => HasherEnum::SHADigest(Context::new(&SHA1_FOR_LEGACY_USE_ONLY)),
            "256" => HasherEnum::SHADigest(Context::new(&SHA256)),
            "384" => HasherEnum::SHADigest(Context::new(&SHA384)),
            "512" => HasherEnum::SHADigest(Context::new(&SHA512)),
            "512_256" => HasherEnum::SHADigest(Context::new(&SHA512_256)),
            _ => panic!("Incorrect hash string input.")
        }
    }
}


#[derive(Debug, Eq, PartialEq,Clone)]
enum AlgorithmID {
    BLAKE3,
    SHA1,
    SHA256,
    SHA384,
    SHA512,
    SHA512_256,
}
#[derive(Clone)]
pub struct HasherOptions  {
    pub hasher: HasherEnum,
    id: AlgorithmID,

}

impl HasherOptions{
    pub fn new( hash_type : &str)-> Self{
        let hasherinstance : HasherOptions;
        match hash_type {
            "blake3" => {
                hasherinstance = HasherOptions {
                    hasher: HasherEnum::Blake3Hasher(blake3::Hasher::new()),
                    id : AlgorithmID::BLAKE3,
                }
            }
            "128" => {
                hasherinstance = HasherOptions {
                    hasher : HasherEnum::SHADigest(Context::new(&SHA1_FOR_LEGACY_USE_ONLY)),
                    id : AlgorithmID::SHA1,
                }
            }
            "256" => {
                hasherinstance = HasherOptions {
                    hasher : HasherEnum::SHADigest(Context::new(&SHA256)),
                    id : AlgorithmID::SHA256,
                }
            }
            "384" => {
                hasherinstance = HasherOptions {
                    hasher : HasherEnum::SHADigest(Context::new(&SHA384)),
                    id : AlgorithmID::SHA384,
                }
            }
            "512" => {
                hasherinstance = HasherOptions {
                    hasher :  HasherEnum::SHADigest(Context::new(&SHA512)),
                    id : AlgorithmID::SHA512,
                }
            }
            "512_256" => {
                hasherinstance = HasherOptions {
                    hasher : HasherEnum::SHADigest(Context::new(&SHA512_256)),
                    id : AlgorithmID::SHA512_256,
                }
            }
            _ => panic!("Incorrect hash string input.")
        };
        hasherinstance
    }



pub fn len(&self) -> usize{
    match self.id {
        AlgorithmID::BLAKE3 => 256,
        AlgorithmID::SHA1 => 128,
        AlgorithmID::SHA256 => 256,
        AlgorithmID::SHA384 => 384,
        AlgorithmID::SHA512 => 512,
        AlgorithmID::SHA512_256 => 256,
    }
}

    pub fn return_hash <'a> (self, input : &[u8]) -> Vec<u8>{
        self.mutli_hash_update(input).multi_hash_finish()
    }

    pub fn multi_hash_finish <'a> (self) -> Vec<u8>{
        let answer: Vec<u8>;
        match self.hasher {
            HasherEnum::Blake3Hasher(hasher) =>{
                let temp_hasher = hasher.finalize();
                answer = temp_hasher.as_bytes()[..].to_vec();
            },
            HasherEnum::SHADigest(digest) => {
                let temp_digest = digest.finish();
                answer = temp_digest.as_ref()[..].to_vec()
            },
        }
        answer
    }

    pub fn mutli_hash_update<'a> (self, input : &[u8]) -> Self{
        let hasherenum = self.hasher;
           match hasherenum{
               HasherEnum::Blake3Hasher(mut hasher) =>{
                hasher.update_with_join::<blake3::join::RayonJoin>(input);
                   HasherOptions {
                       hasher :  HasherEnum::Blake3Hasher(hasher),
                       id : self.id,
                   }
               },
               HasherEnum::SHADigest(mut digest) => {
                   digest.update(input);
                   HasherOptions {
                       hasher :  HasherEnum::SHADigest(digest),
                       id : self.id,
                   }
               },
           }
   }

}

#[cfg(feature = "memmap")]
fn maybe_memmap_file(file: &File) -> Result<Option<memmap::Mmap>> {
    let metadata = file.metadata()?;
    let file_size = metadata.len();
    Ok(if !metadata.is_file() {
        // Not a real file.
        None
    } else if file_size > isize::max_value() as u64 {
        // Too long to safely map.
        // https://github.com/danburkert/memmap-rs/issues/69
        None
    } else if file_size == 0 {
        // Mapping an empty file currently fails.
        // https://github.com/danburkert/memmap-rs/issues/72
        None
    } else {
        // Explicitly set the length of the memory map, so that filesystem
        // changes can't race to violate the invariants we just checked.
        let map = unsafe {
            memmap::MmapOptions::new()
                .len(file_size as usize)
                .map(&file)?
        };
        Some(map)
    })
}

fn maybe_hash_memmap(
    _base_hasher: &HasherOptions,
    _file: &File,
) -> Option<Vec<u8>> {
    #[cfg(feature = "memmap")]
    {
        if let Some(map) = maybe_memmap_file(_file).unwrap(){
            return Some(_base_hasher.clone().mutli_hash_update(&map).multi_hash_finish());
        }
    }
    None
}

fn hash_file(
    base_hasher: &HasherOptions,
    filepath: &std::ffi::OsStr,
) -> Vec<u8> {
    let file = File::open(filepath).unwrap();
    if let Some(output) = maybe_hash_memmap(&base_hasher, &file) {
        println!("File is being hashed with memmap.");
        output // the fast path
    } else {
        // the slow path
        print!("File is being hashed using a buffer with a size of ");
        hash_reader(&base_hasher, file)
    }
}

fn hash_reader(
    base_hasher: &HasherOptions,
    mut reader: impl Read,
) -> Vec<u8> {
    let mut hasher = base_hasher.clone();
    // TODO: This is a narrow copy, so it might not take advantage of SIMD or
    // threads. With a larger buffer size, most of that performance can be
    // recovered. However, this requires some platform-specific tuning, based
    // on both the SIMD degree and the number of cores. A double-buffering
    // strategy is also helpful, where a dedicated background thread reads
    // input into one buffer while another thread is calling update() on a
    // second buffer. Since this is the slow path anyway, do the simple thing
    // for now.
    //std::io::copy(&mut reader, &mut hasher).unwrap();
    let mut buffer = [0; 128 *1024];
    println!("{} bytes.",buffer.len());
    loop {
        let count = match reader.read(&mut buffer) {
            Ok(count) => count,
            Err(why) => panic!("Couldn't load data from file to hash|{}", why.to_string()),
        };
        if count == 0 {
            break;
        }
        hasher = hasher.mutli_hash_update(&buffer[..count]);
    }
    hasher.multi_hash_finish()
}



fn main() {
println!("Ensuring file is cached");
let digest = HasherOptions::new("blake3");
hash_file(&digest,OsStr::new("./test"));
println!("File is cached.");

let hash_type = ["blake3","128","256","384","512","512_256"];
    hash_type.iter().for_each( |each_hash |{
        let start = Instant::now();
        let digest = HasherOptions::new(each_hash);
        println! ("{} value is {:?}",each_hash,HEXUPPER.encode(&hash_file(&digest,OsStr::new("./test"))));
        println!("{} took {:?} milliseconds", each_hash, start.elapsed().as_millis());
    })

}
