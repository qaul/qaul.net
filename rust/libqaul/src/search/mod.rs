// Copyright (c) 2026 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # Libqaul Search
//!
//! Full-text search over qaul content, powered by [tantivy](https://docs.rs/tantivy).
//!
//! ## Usage
//!
//! Each domain that needs search (messages, users, groups, ...) creates its own [`Search`]
//! instance pointing to a dedicated directory.
//!
//! The domain type must implement [`Searchable`] to provide an ID and the text to index.
//!
//! A single `Search` instance should live for the lifetime of the application.

use std::collections::HashSet;
use std::path::Path;

use tantivy::collector::TopDocs;
use tantivy::query::{BooleanQuery, FuzzyTermQuery, Occur, RegexQuery};
use tantivy::schema::{Field, Schema, Term, Value, STORED, STRING, TEXT};
use tantivy::{doc, Index, IndexReader, IndexWriter, ReloadPolicy, TantivyDocument};
use thiserror::Error;

/// Errors returned by [`Search`] operations.
#[derive(Debug, Error)]
pub enum SearchError {
    /// An error originating from the tantivy search engine.
    #[error("tantivy: {0}")]
    Tantivy(#[from] tantivy::TantivyError),

    /// A filesystem I/O error (e.g. creating the index directory).
    #[error("io: {0}")]
    Io(#[from] std::io::Error),

    /// An error when opening a tantivy directory (e.g. path does not exist).
    #[error("directory: {0}")]
    OpenDirectory(#[from] tantivy::directory::error::OpenDirectoryError),
}

/// The contract any searchable domain type must fulfill.
/// Implementors provide an opaque identity string and a content string for indexing.
///
/// `id` must be stable and unique within a given [`Search`] instance.
///
/// # Example
///
/// ```ignore
/// struct Message { id: String, text: String }
///
/// impl Searchable for Message {
///     fn id(&self) -> &str { &self.id }
///     fn content(&self) -> &str { &self.text }
/// }
/// ```
pub trait Searchable {
    fn id(&self) -> &str;
    fn content(&self) -> &str;
}

/// A full-text search index backed by tantivy.
///
/// Each instance owns a single tantivy index directory. Create one per domain
/// (e.g. one for messages, one for users) and keep it alive for the application lifetime.
pub struct Search {
    writer: IndexWriter,
    reader: IndexReader,

    // We store each Field handle to reuse on every index/search call,
    // avoiding repeated string-based lookups into the schema.
    id_field: Field,
    content_field: Field,
}

impl Search {
    /// Creates a new `Search` instance, building or reopening the tantivy index at `path`.
    ///
    /// The directory is created if it does not already exist.
    pub fn new(path: &str) -> Result<Self, SearchError> {
        // create Schema by defining each index search field and build it
        let mut schema_builder = Schema::builder();

        // Field indexing strategies:
        //   STRING: indexed as-is (no tokenization), stored. Suitable for IDs.
        let id_field = schema_builder.add_text_field("id", STRING | STORED);
        //   TEXT: tokenized and normalized for full-text search.
        let content_field = schema_builder.add_text_field("content", TEXT | STORED);

        let schema = schema_builder.build();
        let index_path = Path::new(path);

        // Tantivy expects the directory to already exist — create it if it doesn't.
        std::fs::create_dir_all(index_path)?;

        // create (or open, if existing) the Index for the corresponding Schema in the provided path
        let index = match Index::create_in_dir(index_path, schema.clone()) {
            Ok(index) => index,
            Err(tantivy::TantivyError::IndexAlreadyExists) => Index::open_in_dir(index_path)?,
            Err(e) => return Err(e.into()),
        };

        // Following tantivy documentation, we should:
        //   "create one IndexReader for the entire lifetime of our program,
        //    and acquire a new Searcher for every single request."
        //
        // With this reload policy, "the IndexReader will reload the Index automatically after each [index] commit."
        let reader = index
            .reader_builder()
            .reload_policy(ReloadPolicy::OnCommitWithDelay)
            .try_into()?;

        // Following tantivy documentation:
        //   "There must be only one writer at a time. This single IndexWriter is already multithreaded."
        //
        // That is why we keep it persistent per instance of Search, initializing it
        // with a 50Mb memory_arena budget (also suggested by the docs).
        let writer: IndexWriter = index.writer(50_000_000)?;

        Ok(Self {
            writer,
            reader,
            id_field,
            content_field,
        })
    }

    /// Stages a single document for indexing, deduplicating by ID.
    ///
    /// Note: this does NOT commit. Call `commit()` when you're ready
    /// to make staged documents visible to searches.
    pub fn index(&mut self, item: &impl Searchable) -> Result<(), SearchError> {
        // Deduplication: delete any existing document with this ID before re-adding.
        // `delete_term` stages the deletion — it also doesn't take effect until commit.
        self.delete_id(item.id());

        // add documents using tantivy's doc macro, using each Field handle
        self.writer.add_document(doc!(
            self.id_field => item.id(),
            self.content_field => item.content(),
        ))?;

        Ok(())
    }

    /// Stages a batch of documents. More efficient than calling `index()` in a loop
    /// since you call `commit()` only once.
    ///
    /// Note: this function stages each item in order.
    /// If any item fails, previously staged items are NOT rolled back.
    ///
    /// Call `commit()` only if all items succeed.
    pub fn index_many(&mut self, items: &[impl Searchable]) -> Result<(), SearchError> {
        for item in items {
            self.index(item)?;
        }
        Ok(())
    }

    /// Flushes all staged index operations to disk, making them visible to searches.
    pub fn commit(&mut self) -> Result<(), SearchError> {
        // We commit our index changes via the blocking `commit()` call.
        //
        // According to the documentation:
        //   "We need to call .commit() explicitly to force the index_writer to finish
        //    processing the documents in the queue, flush the current index to the disk,
        //    and advertise the existence of new documents.
        //
        //    If .commit() returns correctly, then all of the documents that have been
        //    added are guaranteed to be persistently indexed."
        self.writer.commit()?;
        Ok(())
    }

    /// Removes the document with given id, committing the change immediately.
    ///
    /// Note: Unlike `index` and `index_many`, this commits on its own.
    /// It's NOT designed to be called inside a batch operation.
    pub fn remove(&mut self, id: &str) -> Result<(), SearchError> {
        self.delete_id(id);
        self.commit()?;
        Ok(())
    }

    // helper that removes the document with given id
    //
    // Stages a term deletion. Does NOT commit.
    fn delete_id(&mut self, id: &str) {
        let id_term = Term::from_field_text(self.id_field, id);
        self.writer.delete_term(id_term);
    }

    /// Searches the index and reconstructs domain objects via a caller-supplied closure.
    ///
    /// `reconstruct` receives the stored ID of each matching document and returns
    /// `Some(T)` if reconstruction succeeds, or `None` to skip that result.
    /// This keeps `Search` decoupled from any specific domain type.
    ///
    /// Multi-word queries require ALL words to match: `"hello world"` only returns documents
    /// containing both `"hello"` and `"world"`. Each word individually matches via prefix OR fuzzy.
    ///
    /// Returns up to 20 results, deduplicated by document ID.
    pub fn search<T, F>(&self, query: &str, reconstruct: F) -> Result<Vec<T>, SearchError>
    where
        F: Fn(&str) -> Option<T>,
    {
        // For each search operation, we acquire a new Searcher, a very cheap operation
        // in which we receive an instance of Searcher that "points to a snapshotted, immutable version of the index."
        let searcher = self.reader.searcher();

        // Normalize the query to match the TEXT tokenizer's lowercasing.
        let query_lower = query.to_lowercase();
        let words: Vec<&str> = query_lower.split_whitespace().collect();

        if words.is_empty() {
            return Ok(vec![]);
        }

        // For each word, build a sub-query: (prefix OR fuzzy).
        // Then combine all words with AND — every word must match.
        //
        // Example: "helo wrld" becomes:
        //   MUST(prefix("helo.*") OR fuzzy("helo"))
        //   AND
        //   MUST(prefix("wrld.*") OR fuzzy("wrld"))
        let mut word_queries: Vec<(Occur, Box<dyn tantivy::query::Query>)> = Vec::new();

        for word in &words {
            let mut sub_queries: Vec<(Occur, Box<dyn tantivy::query::Query>)> = Vec::new();

            // Prefix query via regex (e.g. "creat" matches "creative", "creativeness"...)
            //
            // We escape regex metacharacters in the user's query to prevent injection.
            // If the regex still fails to compile, we skip it and rely on fuzzy alone.
            let escaped = regex::escape(word);
            match RegexQuery::from_pattern(&format!("{escaped}.*"), self.content_field) {
                Ok(q) => sub_queries.push((Occur::Should, Box::new(q))),
                Err(e) => log::warn!("prefix regex failed for '{word}': {e}"),
            }

            // FuzzyTermQuery: Levenshtein distance matching (max distance 2, tantivy constraint).
            let term = Term::from_field_text(self.content_field, word);
            let fuzzy_q = FuzzyTermQuery::new(term, 2, true);
            sub_queries.push((Occur::Should, Box::new(fuzzy_q)));

            let word_query = BooleanQuery::new(sub_queries);
            word_queries.push((Occur::Must, Box::new(word_query)));
        }

        let combined = BooleanQuery::new(word_queries);
        let top_docs = searcher.search(&combined, &TopDocs::with_limit(20))?;

        // Collect results, deduplicating by stored ID (not by DocAddress,
        // which is a tantivy-internal segment address that could theoretically
        // map multiple addresses to the same logical document).
        let mut seen: HashSet<String> = HashSet::new();
        let mut results = Vec::new();

        for (_score, doc_address) in &top_docs {
            let retrieved: TantivyDocument = searcher.doc(*doc_address)?;

            // Extract the stored ID string, then delegate reconstruction to the caller.
            if let Some(id) = retrieved.get_first(self.id_field).and_then(|v| v.as_str()) {
                if seen.insert(id.to_string()) {
                    if let Some(item) = reconstruct(id) {
                        results.push(item);
                    }
                }
            }
        }

        Ok(results)
    }

    // `reload` is used by unit tests to bypass the `OnCommitWithDelay` constraint, as it uses a background thread
    // to refresh the reader -- this creates a timing gap between `commit()`` and the new documents being visible to searches.
    #[cfg(test)]
    pub(crate) fn reload(&self) -> Result<(), SearchError> {
        self.reader.reload()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use tempfile::TempDir;

    // -------------------------------------------------------------------------
    // Minimal domain type for testing
    //
    // It mirrors what a real caller should look like.
    // Keeping it simple focuses each test on search behaviour, not the domain.
    // -------------------------------------------------------------------------

    struct Message {
        id: String,
        text: String,
    }

    impl Searchable for Message {
        fn id(&self) -> &str {
            &self.id
        }
        fn content(&self) -> &str {
            &self.text
        }
    }

    // -------------------------------------------------------------------------
    // Test helpers
    //
    // `msg()`:
    // Simple factory for the Message pseudo-domain type.
    //
    // `make_search()`:
    // Returns both the Search and the TempDir. The TempDir *must* be kept alive
    // for the duration of the test — dropping it deletes the directory, which
    // would invalidate the index mid-test. Binding it to `_dir` in each test
    // ensures it lives until the end of the scope.
    //
    // `index_and_commit()`:
    // Indexes the given items, commits, and forces an immediate reader reload
    // so results are visible without relying on the background watcher thread.
    // -------------------------------------------------------------------------

    fn msg(id: &str, text: &str) -> Message {
        Message {
            id: id.to_string(),
            text: text.to_string(),
        }
    }

    fn make_search() -> (Search, TempDir) {
        let dir = tempfile::tempdir().expect("failed to create temp dir");
        let search = Search::new(dir.path().to_str().unwrap()).expect("failed to create Search");
        (search, dir)
    }

    fn index_and_commit<S: Searchable>(search: &mut Search, items: &[S]) {
        search.index_many(items).expect("indexing failed");
        search.commit().expect("commit failed");
        search.reload().expect("reader reload failed");
    }

    // -------------------------------------------------------------------------
    // Tests
    // -------------------------------------------------------------------------

    /// The simplest possible case: one document, one matching query.
    #[test]
    fn test_exact_match_returns_document() {
        let (mut search, _dir) = make_search();
        index_and_commit(&mut search, &[msg("msg-1", "hello world")]);

        let results: Vec<String> = search
            .search("hello", |id| Some(id.to_string()))
            .expect("search failed");

        assert_eq!(results, vec!["msg-1"]);
    }

    /// Prefix queries let callers implement "search as you type":
    /// typing "creat" should surface "creative" and "creativeness".
    #[test]
    fn test_prefix_match() {
        let (mut search, _dir) = make_search();
        index_and_commit(
            &mut search,
            &[
                msg("msg-1", "creative writing"),
                msg("msg-2", "lacking creativeness"),
                msg("msg-3", "something unrelated"),
            ],
        );

        let mut results: Vec<String> = search
            .search("creat", |id| Some(id.to_string()))
            .expect("search failed");
        results.sort(); // sort for deterministic assertion

        assert_eq!(results, vec!["msg-1", "msg-2"]);
    }

    /// Fuzzy queries tolerate typos up to Levenshtein distance 2.
    /// A misspelled query should still surface the intended document.
    #[test]
    fn test_fuzzy_match_tolerates_typos() {
        let (mut search, _dir) = make_search();
        index_and_commit(&mut search, &[msg("msg-1", "creativeness")]);

        // "creativness" is one character short — distance 1 from "creativeness"
        let results: Vec<String> = search
            .search("creativness", |id| Some(id.to_string()))
            .expect("search failed");

        assert!(
            results.contains(&"msg-1".to_string()),
            "fuzzy match should surface msg-1 despite the typo"
        );
    }

    /// Re-indexing a document with the same ID should not create a duplicate.
    /// The second `index()` call must replace the first.
    #[test]
    fn test_reindexing_same_id_deduplicates() {
        let (mut search, _dir) = make_search();

        // Index an initial version, then update the content under the same ID.
        index_and_commit(&mut search, &[msg("msg-1", "initial content")]);
        index_and_commit(&mut search, &[msg("msg-1", "updated content")]);

        // Only one document should exist for this ID.
        let old_results: Vec<String> = search
            .search("initial", |id| Some(id.to_string()))
            .expect("search failed");
        let new_results: Vec<String> = search
            .search("updated", |id| Some(id.to_string()))
            .expect("search failed");

        assert!(
            old_results.is_empty(),
            "old content should no longer be indexed"
        );
        assert_eq!(
            new_results,
            vec!["msg-1"],
            "updated content should be findable"
        );
    }

    /// The reconstruct closure is the caller's escape hatch: returning `None`
    /// silently drops a result. This is useful when the backing store no longer
    /// holds a document that the index still references (e.g. a deleted message).
    #[test]
    fn test_reconstruct_closure_filters_stale_ids() {
        let (mut search, _dir) = make_search();
        index_and_commit(
            &mut search,
            &[
                msg("msg-exists", "hello world"),
                msg("msg-deleted", "hello everyone"),
            ],
        );

        // Simulate a store where msg-deleted has been removed.
        let live_store: HashMap<&str, &str> = [("msg-exists", "hello world")].into();

        let results: Vec<String> = search
            .search("hello", |id| {
                live_store.get(id).map(|text| text.to_string())
            })
            .expect("search failed");

        assert_eq!(results, vec!["hello world"]);
    }

    /// A query that matches nothing should return an empty Vec, not an error.
    #[test]
    fn test_no_match_returns_empty_vec() {
        let (mut search, _dir) = make_search();
        index_and_commit(&mut search, &[msg("msg-1", "hello world")]);

        let results: Vec<String> = search
            .search("zzznomatch", |id| Some(id.to_string()))
            .expect("search failed");

        assert!(results.is_empty());
    }

    /// Searching an empty index should return an empty Vec, not an error.
    #[test]
    fn test_search_on_empty_index() {
        let (search, _dir) = make_search();

        let results: Vec<String> = search
            .search("anything", |id| Some(id.to_string()))
            .expect("search failed");

        assert!(results.is_empty());
    }

    /// `index_many` followed by a single `commit` is more efficient than
    /// committing after every document. This test verifies all documents
    /// in the batch are visible after the single commit.
    #[test]
    fn test_batch_indexing_commits_all_documents() {
        let (mut search, _dir) = make_search();

        let messages = vec![
            msg("msg-1", "first message"),
            msg("msg-2", "second message"),
            msg("msg-3", "third message"),
        ];
        index_and_commit(&mut search, &messages);

        let mut results: Vec<String> = search
            .search("message", |id| Some(id.to_string()))
            .expect("search failed");
        results.sort();

        assert_eq!(results, vec!["msg-1", "msg-2", "msg-3"]);
    }

    /// After `remove`, the document should no longer appear in search results.
    #[test]
    fn test_remove_deletes_document() {
        let (mut search, _dir) = make_search();
        index_and_commit(
            &mut search,
            &[msg("msg-1", "hello world"), msg("msg-2", "hello everyone")],
        );

        search.remove("msg-1").expect("remove failed");
        search.reload().expect("reload failed");

        let results: Vec<String> = search
            .search("hello", |id| Some(id.to_string()))
            .expect("search failed");

        assert_eq!(results, vec!["msg-2"]);
    }

    /// Queries containing regex metacharacters should not cause errors
    /// and should not be interpreted as regex syntax in the prefix query.
    ///
    /// Note: some metacharacter queries may still return results via fuzzy matching
    /// (e.g. "hel.o" is Levenshtein distance 1 from "hello"). That's expected —
    /// the important thing is that the regex engine doesn't treat them as operators.
    #[test]
    fn test_query_with_regex_metacharacters() {
        let (mut search, _dir) = make_search();
        index_and_commit(
            &mut search,
            &[msg("msg-1", "hello world"), msg("msg-2", "a]b test")],
        );

        // None of these should error, regardless of metacharacters in the query
        search
            .search("hel.o", |id| Some(id.to_string()))
            .expect("search with '.' should not error");
        search
            .search("a|b", |id| Some(id.to_string()))
            .expect("search with '|' should not error");
        search
            .search("(hello", |id| Some(id.to_string()))
            .expect("search with '(' should not error");
        search
            .search("test[", |id| Some(id.to_string()))
            .expect("search with '[' should not error");

        // Without escaping, "zzz(.+)zzz" would be a valid regex capture group.
        // With escaping, it's a literal that matches nothing in our index.
        // It's also too distant from any indexed token for fuzzy to match.
        let regex_results: Vec<String> = search
            .search("zzz(.+)zzz", |id| Some(id.to_string()))
            .expect("search with regex group syntax should not error");
        assert!(
            regex_results.is_empty(),
            "regex capture group syntax should not match anything"
        );
    }

    /// `index_many` with an empty slice should be a harmless no-op.
    #[test]
    fn test_index_many_empty_slice_is_noop() {
        let (mut search, _dir) = make_search();
        let empty: &[Message] = &[];
        search
            .index_many(empty)
            .expect("indexing empty slice should succeed");
        search.commit().expect("commit should succeed");
        search.reload().expect("reload failed");

        let results: Vec<String> = search
            .search("anything", |id| Some(id.to_string()))
            .expect("search failed");
        assert!(results.is_empty());
    }

    /// tantivy's TEXT index fields are normalized to lowercase, so search should normalize queries.
    #[test]
    fn test_search_is_case_insensitive() {
        let (mut search, _dir) = make_search();
        index_and_commit(&mut search, &[msg("msg-1", "Hello World")]);

        let results: Vec<String> = search
            .search("HELLO", |id| Some(id.to_string()))
            .expect("search failed");
        assert_eq!(results, vec!["msg-1"]);
    }

    /// Multi-word queries use AND semantics: every word must match.
    /// "hello world" matches a doc containing both words, but not one with only "hello".
    #[test]
    fn test_multi_word_query_requires_all_words() {
        let (mut search, _dir) = make_search();
        index_and_commit(
            &mut search,
            &[
                msg("msg-1", "hello world"),
                msg("msg-2", "hello everyone"),
                msg("msg-3", "goodbye world"),
            ],
        );

        // "hello world" should only match msg-1 (both words present)
        let results: Vec<String> = search
            .search("hello world", |id| Some(id.to_string()))
            .expect("search failed");
        assert_eq!(results, vec!["msg-1"]);
    }

    /// Multi-word queries still benefit from prefix and fuzzy matching per word.
    /// "helo wrld" should match "hello world" via fuzzy on both words.
    #[test]
    fn test_multi_word_query_with_typos() {
        let (mut search, _dir) = make_search();
        index_and_commit(&mut search, &[msg("msg-1", "hello world")]);

        let results: Vec<String> = search
            .search("helo wrld", |id| Some(id.to_string()))
            .expect("search failed");
        assert_eq!(results, vec!["msg-1"]);
    }

    /// Multi-word queries where one word matches nothing should return empty results.
    #[test]
    fn test_multi_word_query_partial_miss() {
        let (mut search, _dir) = make_search();
        index_and_commit(&mut search, &[msg("msg-1", "hello world")]);

        // "hello zzzzz" — first word matches, second doesn't
        let results: Vec<String> = search
            .search("hello zzzzz", |id| Some(id.to_string()))
            .expect("search failed");
        assert!(results.is_empty());
    }
}
