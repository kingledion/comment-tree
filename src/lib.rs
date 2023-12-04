//! A multi-tree implementation optimized for comment threads
//! 
//! Implements a Forest of Tree objects that represents multiple comment threads 
//! that can be attached to some parent post or image. 
use thiserror::Error;

// RepositoryError enumerates all possible errors returned from intereactions with CouchDB repository
#[derive(Error, Debug)]
pub enum Error {
}