#![cfg(feature = "ctr")]
#![feature(test)]

#[cfg(feature = "ctr")]
stream_cipher::bench_sync!(aesni::Aes256Ctr);
