use crypto::hashes::{blake2b::Blake2b256, Digest};
use crypto::signatures::ed25519;

use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread,
};

fn main() -> Result<(), Error> {
    //string to search for
    let contains = "1ota";
    // contains is at any position of the did
    let position = Position::Any;
    let threads = 16;

    //validate that the input is valid base 58
    bs58::decode(contains)
        .with_alphabet(bs58::Alphabet::BITCOIN)
        .into_vec()
        .unwrap();

    let cancel = Arc::new(AtomicBool::new(false));

    let mut pool = vec![];
    for _ in 0..threads {
        let _cancel = cancel.clone();
        let _position = position.clone();
        pool.push(thread::spawn(move || {
            search_did(contains, _position, _cancel)
        }));
    }
    for worker in pool {
        if let Ok(Ok(key)) = worker.join() {
            println!(
                "Found a match: private key: {:?}, public key: {:?}, did_tag: {:?}",
                key.0, key.1, key.2
            );
        }
    }
    Ok(())
}

#[derive(Clone, PartialEq)]
enum Position {
    Any,
    Start,
}
fn search_did(
    contains: &str,
    position: Position,
    cancel: Arc<AtomicBool>,
) -> Result<(String, String, String), Error> {
    while !cancel.load(Ordering::Relaxed) {
        if let Ok(key) = generate_key_did_tag_pair(contains, &position) {
            cancel.store(true, Ordering::Relaxed);
            return Ok(key);
        }
    }
    Err(Error::Canceled)
}

fn generate_key_did_tag_pair(
    contains: &str,
    position: &Position,
) -> Result<(String, String, String), Error> {
    let secret: ed25519::SecretKey = ed25519::SecretKey::generate()?;
    let public: ed25519::PublicKey = secret.public_key();

    let did = encode_key_to_did_tag(&public.to_bytes().to_vec());
    // any position
    if position == &Position::Any {
        if did.contains(contains) {
            let secret: Vec<u8> = secret.to_bytes().to_vec().into();
            return Ok((
                bs58::encode(secret)
                    .with_alphabet(bs58::Alphabet::BITCOIN)
                    .into_string(),
                bs58::encode(public)
                    .with_alphabet(bs58::Alphabet::BITCOIN)
                    .into_string(),
                did,
            ));
        }
    }
    // start position
    if position == &Position::Start {
        if &did[0..contains.len()] == contains {
            let secret: Vec<u8> = secret.to_bytes().to_vec();
            return Ok((
                bs58::encode(secret)
                    .with_alphabet(bs58::Alphabet::BITCOIN)
                    .into_string(),
                bs58::encode(public)
                    .with_alphabet(bs58::Alphabet::BITCOIN)
                    .into_string(),
                did,
            ));
        }
    }
    Err(Error::UnwantedDID)
}

fn encode_key_to_did_tag(key: &[u8]) -> String {
    bs58::encode(&Blake2b256::digest(key))
        .with_alphabet(bs58::Alphabet::BITCOIN)
        .into_string()
}

#[derive(Debug)]
enum Error {
    Crypto(crypto::Error),
    Canceled,
    UnwantedDID,
}

impl From<crypto::Error> for Error {
    fn from(error: crypto::Error) -> Self {
        Error::Crypto(error)
    }
}
