//! Document cache and state management.
//!
//! Provides LRU caching for open documents with a configurable capacity.

#![allow(dead_code)]

use std::num::NonZeroUsize;

use dampen_core::ir::DampenDocument;
use dampen_core::parser::error::ParseError;
use dampen_core::parser::parse;
use lru::LruCache;
use tower_lsp::lsp_types::Url;
use tracing::{debug, trace};

/// State for an open document.
///
/// Contains the document content, parsed AST (if valid), and version info.
#[derive(Debug, Clone)]
pub struct DocumentState {
    /// Document URI
    pub uri: Url,
    /// Document content
    pub content: String,
    /// Document version (incremented on each change)
    pub version: i32,
    /// Parsed AST (None if parse failed)
    pub ast: Option<DampenDocument>,
    /// Parse errors (empty if parse succeeded)
    pub parse_errors: Vec<ParseError>,
}

impl DocumentState {
    /// Creates a new document state.
    ///
    /// Parses the content immediately and stores the result.
    ///
    /// # Arguments
    ///
    /// * `uri` - Document URI
    /// * `content` - Document content
    /// * `version` - Document version
    pub fn new(uri: Url, content: String, version: i32) -> Self {
        trace!("Creating DocumentState for {} (version {})", uri, version);

        // Parse the document
        let (ast, parse_errors) = match parse(&content) {
            Ok(doc) => (Some(doc), vec![]),
            Err(error) => (None, vec![error]),
        };

        Self {
            uri,
            content,
            version,
            ast,
            parse_errors,
        }
    }
}

/// LRU cache for open documents.
///
/// Maintains a fixed-capacity cache of document states. When the cache
/// is full and a new document is inserted, the least recently accessed
/// document is evicted.
pub struct DocumentCache {
    cache: LruCache<Url, DocumentState>,
}

impl DocumentCache {
    /// Creates a new document cache with the specified capacity.
    ///
    /// # Arguments
    ///
    /// * `capacity` - Maximum number of documents to cache
    ///
    /// # Returns
    ///
    /// New DocumentCache instance
    pub fn new(capacity: usize) -> Self {
        // SAFETY: capacity.max(1) ensures the value is at least 1, so NonZeroUsize::new will never return None
        let capacity = unsafe { NonZeroUsize::new_unchecked(capacity.max(1)) };

        debug!("Creating DocumentCache with capacity {}", capacity);

        Self {
            cache: LruCache::new(capacity),
        }
    }

    /// Gets a document from the cache.
    ///
    /// Marks the document as recently used.
    ///
    /// # Arguments
    ///
    /// * `uri` - Document URI
    ///
    /// # Returns
    ///
    /// Reference to the document state if found
    pub fn get(&mut self, uri: &Url) -> Option<&DocumentState> {
        self.cache.get(uri)
    }

    /// Gets a mutable reference to a document from the cache.
    ///
    /// Marks the document as recently used.
    ///
    /// # Arguments
    ///
    /// * `uri` - Document URI
    ///
    /// # Returns
    ///
    /// Mutable reference to the document state if found
    pub fn get_mut(&mut self, uri: &Url) -> Option<&mut DocumentState> {
        self.cache.get_mut(uri)
    }

    /// Inserts or updates a document in the cache.
    ///
    /// If the cache is full, the least recently used document is evicted.
    ///
    /// # Arguments
    ///
    /// * `uri` - Document URI
    /// * `state` - Document state
    pub fn insert(&mut self, uri: Url, state: DocumentState) {
        debug!("Inserting document into cache: {}", uri);
        self.cache.put(uri, state);
    }

    /// Removes a document from the cache.
    ///
    /// # Arguments
    ///
    /// * `uri` - Document URI
    ///
    /// # Returns
    ///
    /// The removed document state if it existed
    pub fn remove(&mut self, uri: &Url) -> Option<DocumentState> {
        debug!("Removing document from cache: {}", uri);
        self.cache.pop(uri)
    }

    /// Clears all documents from the cache.
    pub fn clear(&mut self) {
        debug!("Clearing document cache");
        self.cache.clear();
    }

    /// Returns the number of documents in the cache.
    pub fn len(&self) -> usize {
        self.cache.len()
    }

    /// Returns true if the cache is empty.
    pub fn is_empty(&self) -> bool {
        self.cache.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_uri() -> Url {
        Url::parse("file:///test.dampen").unwrap()
    }

    fn test_doc_state() -> DocumentState {
        DocumentState::new(test_uri(), "<column/>".to_string(), 1)
    }

    #[test]
    fn test_cache_insert_and_get() {
        let mut cache = DocumentCache::new(10);
        let uri = test_uri();
        let state = test_doc_state();

        cache.insert(uri.clone(), state);

        assert!(cache.get(&uri).is_some());
        assert_eq!(cache.len(), 1);
    }

    #[test]
    fn test_cache_remove() {
        let mut cache = DocumentCache::new(10);
        let uri = test_uri();
        let state = test_doc_state();

        cache.insert(uri.clone(), state);
        let removed = cache.remove(&uri);

        assert!(removed.is_some());
        assert!(cache.get(&uri).is_none());
        assert_eq!(cache.len(), 0);
    }

    #[test]
    fn test_cache_capacity() {
        let mut cache = DocumentCache::new(2);

        let uri1 = Url::parse("file:///test1.dampen").unwrap();
        let uri2 = Url::parse("file:///test2.dampen").unwrap();
        let uri3 = Url::parse("file:///test3.dampen").unwrap();

        cache.insert(uri1.clone(), test_doc_state());
        cache.insert(uri2.clone(), test_doc_state());
        cache.insert(uri3.clone(), test_doc_state());

        // First document should be evicted
        assert!(cache.get(&uri1).is_none());
        assert!(cache.get(&uri2).is_some());
        assert!(cache.get(&uri3).is_some());
        assert_eq!(cache.len(), 2);
    }
}
