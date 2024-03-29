use blake3::Hasher;
use data_encoding::HEXUPPER;
use ring::digest::Context;
use ring::digest::{SHA1_FOR_LEGACY_USE_ONLY, SHA256, SHA384, SHA512, SHA512_256};
use std::ffi::OsStr;
use std::time::Instant;

use pretty_bytes::converter::convert;

use std::fs::{metadata, File};

use std::io::Read;

#[cfg(feature = "memmap")]
use anyhow::Result;

#[derive(Clone)]
pub enum HasherEnum {
    Blake3Hasher(Box<Hasher>),
    SHADigest(Box<Context>),
    MD5(Box<[u8; 16]>),
}

impl HasherEnum {
    pub fn new(hash_type: &str) -> Self {
        match hash_type {
            "blake3" => HasherEnum::Blake3Hasher(Box::new(blake3::Hasher::new())),
            "128" => HasherEnum::SHADigest(Box::new(Context::new(&SHA1_FOR_LEGACY_USE_ONLY))),
            "256" => HasherEnum::SHADigest(Box::new(Context::new(&SHA256))),
            "384" => HasherEnum::SHADigest(Box::new(Context::new(&SHA384))),
            "512" => HasherEnum::SHADigest(Box::new(Context::new(&SHA512))),
            "512_256" => HasherEnum::SHADigest(Box::new(Context::new(&SHA512_256))),
            "MD5" => HasherEnum::MD5(Box::new([0; 16])),
            _ => panic!("Incorrect hash string input."),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
enum AlgorithmID {
    BLAKE3,
    SHA1,
    SHA256,
    SHA384,
    SHA512,
    SHA512_256,
    MD5,
}
#[derive(Clone)]
pub struct HasherOptions {
    pub hasher: HasherEnum,
    id: AlgorithmID,
}

impl HasherOptions {
    pub fn new(hash_type: &str) -> Self {
        let hasherinstance: HasherOptions;
        match hash_type {
            "blake3" => {
                hasherinstance = HasherOptions {
                    hasher: HasherEnum::Blake3Hasher(Box::new(blake3::Hasher::new())),
                    id: AlgorithmID::BLAKE3,
                }
            }
            "128" => {
                hasherinstance = HasherOptions {
                    hasher: HasherEnum::SHADigest(Box::new(Context::new(
                        &SHA1_FOR_LEGACY_USE_ONLY,
                    ))),
                    id: AlgorithmID::SHA1,
                }
            }
            "256" => {
                hasherinstance = HasherOptions {
                    hasher: HasherEnum::SHADigest(Box::new(Context::new(&SHA256))),
                    id: AlgorithmID::SHA256,
                }
            }
            "384" => {
                hasherinstance = HasherOptions {
                    hasher: HasherEnum::SHADigest(Box::new(Context::new(&SHA384))),
                    id: AlgorithmID::SHA384,
                }
            }
            "512" => {
                hasherinstance = HasherOptions {
                    hasher: HasherEnum::SHADigest(Box::new(Context::new(&SHA512))),
                    id: AlgorithmID::SHA512,
                }
            }
            "512_256" => {
                hasherinstance = HasherOptions {
                    hasher: HasherEnum::SHADigest(Box::new(Context::new(&SHA512_256))),
                    id: AlgorithmID::SHA512_256,
                }
            }
            "MD5" => {
                hasherinstance = HasherOptions {
                    hasher: HasherEnum::MD5(Box::new([0; 16])),
                    id: AlgorithmID::MD5,
                }
            }
            _ => panic!("Incorrect hash string input."),
        };
        hasherinstance
    }

    pub fn hash_len(&self) -> usize {
        match self.id {
            AlgorithmID::BLAKE3 => 256,
            AlgorithmID::SHA1 => 128,
            AlgorithmID::SHA256 => 256,
            AlgorithmID::SHA384 => 384,
            AlgorithmID::SHA512 => 512,
            AlgorithmID::SHA512_256 => 256,
            AlgorithmID::MD5 => 128,
        }
    }

    pub fn return_hash(self, input: &[u8]) -> Vec<u8> {
        self.mutli_hash_update(input).multi_hash_finish()
    }

    pub fn multi_hash_finish(self) -> Vec<u8> {
        let answer: Vec<u8>;
        match self.hasher {
            HasherEnum::Blake3Hasher(hasher) => {
                let temp_hasher = hasher.finalize();
                answer = temp_hasher.as_bytes()[..].to_vec();
            }
            HasherEnum::SHADigest(digest) => {
                let temp_digest = digest.finish();
                answer = temp_digest.as_ref()[..].to_vec()
            }
            HasherEnum::MD5(digest) => answer = digest.to_vec(),
        }
        answer
    }

    pub fn mutli_hash_update(self, input: &[u8]) -> Self {
        let hasherenum = self.hasher;
        match hasherenum {
            HasherEnum::Blake3Hasher(mut hasher) => {
                hasher.update_rayon(input);
                HasherOptions {
                    hasher: HasherEnum::Blake3Hasher(hasher),
                    id: self.id,
                }
            }
            HasherEnum::SHADigest(mut digest) => {
                digest.update(input);
                HasherOptions {
                    hasher: HasherEnum::SHADigest(digest),
                    id: self.id,
                }
            }
            HasherEnum::MD5(_) => HasherOptions {
                hasher: HasherEnum::MD5(Box::new(md5::compute(input).0)),
                id: self.id,
            },
        }
    }
}

#[cfg(feature = "memmap")]
fn maybe_memmap_file(file: &File) -> Result<Option<memmap::Mmap>> {
    let metadata = file.metadata()?;
    let file_size = metadata.len();
    Ok(
        if !metadata.is_file() || file_size > isize::max_value() as u64 || file_size == 0 {
            // Not a real file.
            // Too long to safely map.
            // https://github.com/danburkert/memmap-rs/issues/69
            // Mapping an empty file currently fails.
            // https://github.com/danburkert/memmap-rs/issues/72
            None
        } else {
            // Explicitly set the length of the memory map, so that filesystem
            // changes can't race to violate the invariants we just checked.
            let map = unsafe {
                memmap::MmapOptions::new()
                    .len(file_size as usize)
                    .map(file)?
            };
            Some(map)
        },
    )
}

fn maybe_hash_memmap(_base_hasher: &HasherOptions, _file: &File) -> Option<Vec<u8>> {
    #[cfg(feature = "memmap")]
    {
        if let Some(map) = maybe_memmap_file(_file).unwrap() {
            return Some(
                _base_hasher
                    .clone()
                    .mutli_hash_update(&map)
                    .multi_hash_finish(),
            );
        }
    }
    None
}

fn hash_file(base_hasher: &HasherOptions, filepath: &std::ffi::OsStr) -> Vec<u8> {
    let file = File::open(filepath).unwrap();
    if let Some(output) = maybe_hash_memmap(base_hasher, &file) {
        println!("File is being hashed with memmap.");
        output // the fast path
    } else {
        // the slow path
        print!("File is being hashed using a buffer with a size of ");
        hash_reader(base_hasher, file)
    }
}

fn hash_reader(base_hasher: &HasherOptions, mut reader: impl Read) -> Vec<u8> {
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
    let mut buffer = [0; 256 * 1024];
    //buffer size optmized for large files
    println!("{}.", convert(buffer.len() as f64));
    loop {
        let count = match reader.read(&mut buffer) {
            Ok(count) => count,
            Err(why) => panic!("Couldn't load data from file to hash|{}", why),
        };
        if count == 0 {
            break;
        }
        hasher = hasher.mutli_hash_update(&buffer[..count]);
    }
    hasher.multi_hash_finish()
}

static FILE_NAME: &str = "./test";

fn main() {
    let file_size = metadata(FILE_NAME).unwrap().len();
    println!(
        "Input file named {} is {} in size.",
        FILE_NAME,
        convert(file_size as f64)
    );
    println!("Ensuring file is cached");
    let digest = HasherOptions::new("blake3");
    hash_file(&digest, OsStr::new(FILE_NAME));
    println!("File is cached.");

    let hash_type = ["blake3", "128", "256", "384", "512", "512_256", "MD5"];
    hash_type.iter().for_each(|each_hash| {
        let start = Instant::now();
        let digest = HasherOptions::new(each_hash);
        println!(
            "{} value is {:?}",
            each_hash,
            HEXUPPER.encode(&hash_file(&digest, OsStr::new(FILE_NAME)))
        );
        let elapsed_time = start.elapsed().as_millis();
        println!(
            "{} took {:?} milliseconds. Speed is {}s",
            each_hash,
            elapsed_time,
            if elapsed_time == 0 {
                "N/A".to_string()
            } else {
                convert(((file_size as u128 / elapsed_time) * 1000) as f64)
            }
        );
    })
}
