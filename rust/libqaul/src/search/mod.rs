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
use tantivy::schema::{Field, Schema, Term, Value, FAST, STORED, STRING, TEXT};
use tantivy::{DocAddress, Index, IndexReader, IndexWriter, ReloadPolicy, Score, TantivyDocument};
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

    /// Optional ranking key, used only by indexes built with [`SearchConfig::ranked`].
    ///
    /// When the index is ranked, this value is stored in the `ranking` fast field
    /// and breaks ties between documents of (near-)equal text relevance — higher wins.
    /// Text-only indexes ignore this entirely, so the default of `None` (stored as `0`)
    /// is correct for any type that does not opt into ranking.
    fn ranking_key(&self) -> Option<u64> {
        None
    }
}

/// Selects the on-disk schema and the scoring strategy for a [`Search`] instance.
#[derive(Clone, Copy)]
pub struct SearchConfig {
    ranked: bool,
}

impl SearchConfig {
    /// The resulting index will be a two-field text schema:
    /// `id` (`STRING | STORED`) and `content` (`TEXT | STORED`).
    /// Results are ordered by text relevance alone.
    pub fn text_only() -> Self {
        Self { ranked: false }
    }

    /// The text schema plus a third `ranking` field (`u64`, `FAST`) used to
    /// break relevance ties by recency. Items should return their recency from
    /// [`Searchable::ranking_key`].
    pub fn ranked() -> Self {
        Self { ranked: true }
    }
}

/// Builds the tantivy schema for a given config and returns the field handles.
///
/// **The field add-order is part of the on-disk contract.** When opening an existing
/// index, tantivy validates against the schema stored on disk, and we reuse the `Field`
/// handles produced here (which are positional). `id` (0) and `content` (1) must keep
/// their positions across both configs, so the optional `ranking` (2) is only
/// ever appended last. Changing this order silently corrupts every existing index.
fn build_schema(config: &SearchConfig) -> (Schema, Field, Field, Option<Field>) {
    let mut schema_builder = Schema::builder();
    // STRING: indexed as-is (no tokenization), stored. Suitable for IDs.
    let id_field = schema_builder.add_text_field("id", STRING | STORED);
    // TEXT: tokenized and normalized for full-text search.
    let content_field = schema_builder.add_text_field("content", TEXT | STORED);
    // FAST: column-oriented storage, readable per-segment during scoring.
    let ranking_field = if config.ranked {
        Some(schema_builder.add_u64_field("ranking", FAST))
    } else {
        None
    };

    (
        schema_builder.build(),
        id_field,
        content_field,
        ranking_field,
    )
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

    // Present only for indexes built with `SearchConfig::ranked`. When set, documents
    // carry a recency value in this fast field and `search()` uses the ranked code path.
    ranking_field: Option<Field>,

    // True if this index was freshly created (not opened from an existing directory).
    // Callers use this to decide whether a batch backfill of existing data is needed.
    is_fresh: bool,
}

impl Search {
    /// Creates a new `Search` instance, building or reopening the tantivy index at `path`.
    ///
    /// `config` selects the schema and scoring strategy. When reopening an existing index,
    /// the config must match the one the index was created with (the schema is validated
    /// against the on-disk copy).
    ///
    /// The directory is created if it does not already exist.
    pub fn new(path: &str, config: SearchConfig) -> Result<Self, SearchError> {
        // create Schema by defining each index search field and build it
        let (schema, id_field, content_field, ranking_field) = build_schema(&config);
        let index_path = Path::new(path);

        // Tantivy expects the directory to already exist — create it if it doesn't.
        std::fs::create_dir_all(index_path)?;

        // create (or open, if existing) the Index for the corresponding Schema in the provided path
        let (index, is_fresh) = match Index::create_in_dir(index_path, schema.clone()) {
            Ok(index) => (index, true),
            Err(tantivy::TantivyError::IndexAlreadyExists) => {
                (Index::open_in_dir(index_path)?, false)
            }
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
        // with a 15Mb memory_arena budget (suitable for mobile).
        let writer: IndexWriter = index.writer(15_000_000)?;

        Ok(Self {
            writer,
            reader,
            id_field,
            content_field,
            ranking_field,
            is_fresh,
        })
    }

    /// Returns `true` if this index was freshly created (not opened from an existing directory).
    ///
    /// Callers use this to decide whether a batch backfill of existing data is needed.
    pub fn is_fresh(&self) -> bool {
        self.is_fresh
    }

    /// Stages a single document for indexing, deduplicating by ID.
    ///
    /// Note: this does NOT commit. Call `commit()` when you're ready
    /// to make staged documents visible to searches.
    pub fn index(&mut self, item: &impl Searchable) -> Result<(), SearchError> {
        // Deduplication: delete any existing document with this ID before re-adding.
        // `delete_term` stages the deletion — it also doesn't take effect until commit.
        self.delete_id(item.id());

        // Build the document by hand so the ranking field is only attached when this
        // index is ranked. On a text-only index `ranking_field` is `None` and the
        // recency value is never written.
        let mut document = TantivyDocument::default();
        document.add_text(self.id_field, item.id());
        document.add_text(self.content_field, item.content());
        if let Some(ranking_field) = self.ranking_field {
            // A ranked item without a ranking key sorts as least-recent (0).
            document.add_u64(ranking_field, item.ranking_key().unwrap_or(0));
        }

        self.writer.add_document(document)?;

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
    #[allow(dead_code)]
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
    /// Returns up to `limit` results (defaulting to 20 when `None`), deduplicated by document ID.
    ///
    /// ## Ordering
    ///
    /// - **Text-only** indexes order purely by tantivy's text relevance score.
    /// - **Ranked** indexes order by a composite `(relevance, recency)` key: relevance is
    ///   primary, recency (the `ranking` fast field) only breaks ties.
    ///
    ///   In practice the matcher combines a regex-prefix query with a fuzzy query, and both
    ///   produce coarse, near-constant relevance scores across the matching set. So while the
    ///   ordering is nominally relevance-primary, observed behaviour for typical short queries
    ///   collapses toward pure recency — which is the desired UX for "most recently active
    ///   room first".
    pub fn search<T, F>(
        &self,
        query: &str,
        limit: Option<usize>,
        reconstruct: F,
    ) -> Result<Vec<T>, SearchError>
    where
        F: Fn(&str) -> Option<T>,
    {
        // For each search operation, we acquire a new Searcher, a very cheap operation
        // in which we receive an instance of Searcher that "points to a snapshotted, immutable version of the index."
        let searcher = self.reader.searcher();

        // `TopDocs::with_limit` panics on a zero limit, and callers legitimately pass the
        // live collection size (which may be 0). Treat "room for nothing" as no results.
        let limit = limit.unwrap_or(20);
        if limit == 0 {
            return Ok(vec![]);
        }

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
        // Run the query through the collector appropriate to this index's config.
        // Both paths yield doc addresses already in final display order.
        let doc_addresses = match self.ranking_field {
            Some(_) => self.search_ranked(&searcher, &combined, limit)?,
            None => self.search_by_relevance(&searcher, &combined, limit)?,
        };

        // Collect results, deduplicating by stored ID (not by DocAddress,
        // which is a tantivy-internal segment address that could theoretically
        // map multiple addresses to the same logical document).
        let mut seen: HashSet<String> = HashSet::new();
        let mut results = Vec::new();

        for doc_address in &doc_addresses {
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

    /// Text-only collector: order purely by tantivy's relevance score.
    fn search_by_relevance(
        &self,
        searcher: &tantivy::Searcher,
        query: &BooleanQuery,
        limit: usize,
    ) -> Result<Vec<DocAddress>, SearchError> {
        let top_docs = searcher.search(query, &TopDocs::with_limit(limit))?;
        Ok(top_docs.into_iter().map(|(_score, addr)| addr).collect())
    }

    /// Ranked collector: order by a composite `(relevance, recency)` sort key.
    ///
    /// `(f32, u64)` is `PartialOrd`, which is all `tweak_score` requires, and tuple
    /// ordering is lexicographic.
    fn search_ranked(
        &self,
        searcher: &tantivy::Searcher,
        query: &BooleanQuery,
        limit: usize,
    ) -> Result<Vec<DocAddress>, SearchError> {
        let collector = TopDocs::with_limit(limit).tweak_score(
            move |segment_reader: &tantivy::SegmentReader| {
                let recency_reader = segment_reader
                    .fast_fields()
                    .u64("ranking")
                    .expect("ranked index must declare the 'ranking' fast field")
                    .first_or_default_col(0);

                move |doc: tantivy::DocId, original_score: Score| {
                    let recency: u64 = recency_reader.get_val(doc);
                    (original_score, recency)
                }
            },
        );

        let top_docs: Vec<((Score, u64), DocAddress)> = searcher.search(query, &collector)?;
        Ok(top_docs.into_iter().map(|(_key, addr)| addr).collect())
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

    // A ranked domain type carrying a recency key, used to exercise the ranked path.
    struct Room {
        id: String,
        name: String,
        last_message_at: u64,
    }

    impl Searchable for Room {
        fn id(&self) -> &str {
            &self.id
        }
        fn content(&self) -> &str {
            &self.name
        }
        fn ranking_key(&self) -> Option<u64> {
            Some(self.last_message_at)
        }
    }

    fn room(id: &str, name: &str, last_message_at: u64) -> Room {
        Room {
            id: id.to_string(),
            name: name.to_string(),
            last_message_at,
        }
    }

    fn make_ranked_search() -> (Search, TempDir) {
        let dir = tempfile::tempdir().expect("failed to create temp dir");
        let search = Search::new(dir.path().to_str().unwrap(), SearchConfig::ranked())
            .expect("failed to create ranked Search");
        (search, dir)
    }

    // -------------------------------------------------------------------------
    // Schema-compatibility pins
    //
    // The on-disk chat and users indexes were created with the text-only schema.
    // Its field set and add-order are an on-disk contract: tantivy reuses positional
    // field handles when reopening, so any drift here silently corrupts those indexes.
    // These tests fail loudly if the schema shape ever changes.
    // -------------------------------------------------------------------------

    #[test]
    fn test_text_only_schema_is_stable() {
        let (schema, id_field, content_field, ranking_field) =
            build_schema(&SearchConfig::text_only());
        assert!(
            ranking_field.is_none(),
            "text-only must not add a ranking field"
        );

        let fields: Vec<_> = schema.fields().collect();
        assert_eq!(
            fields.len(),
            2,
            "text-only schema must be exactly id + content"
        );

        let (f0, e0) = &fields[0];
        let (f1, e1) = &fields[1];
        assert_eq!(*f0, id_field);
        assert_eq!(*f1, content_field);
        assert_eq!(e0.name(), "id");
        assert_eq!(e1.name(), "content");

        // id: STRING | STORED -> stored, indexed with the non-tokenizing "raw" tokenizer.
        match e0.field_type() {
            tantivy::schema::FieldType::Str(opts) => {
                assert!(opts.is_stored(), "id must be stored");
                let indexing = opts.get_indexing_options().expect("id must be indexed");
                assert_eq!(indexing.tokenizer(), "raw", "id must use the raw tokenizer");
            }
            other => panic!("id field type changed: {:?}", other),
        }

        // content: TEXT | STORED -> stored, indexed with the "default" tokenizer.
        match e1.field_type() {
            tantivy::schema::FieldType::Str(opts) => {
                assert!(opts.is_stored(), "content must be stored");
                let indexing = opts
                    .get_indexing_options()
                    .expect("content must be indexed");
                assert_eq!(
                    indexing.tokenizer(),
                    "default",
                    "content must use the default tokenizer"
                );
            }
            other => panic!("content field type changed: {:?}", other),
        }
    }

    #[test]
    fn test_ranked_schema_appends_recency_fast_field() {
        let (schema, _id_field, _content_field, ranking_field) =
            build_schema(&SearchConfig::ranked());
        assert!(ranking_field.is_some(), "ranked must add a ranking field");

        let fields: Vec<_> = schema.fields().collect();
        assert_eq!(
            fields.len(),
            3,
            "ranked schema must be id + content + ranking"
        );

        // The ranking field must come *last* so id/content keep positions 0 and 1.
        let (_f2, e2) = &fields[2];
        assert_eq!(e2.name(), "ranking");
        match e2.field_type() {
            tantivy::schema::FieldType::U64(opts) => {
                assert!(opts.is_fast(), "ranking must be a FAST field");
            }
            other => panic!("ranking field type changed: {:?}", other),
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
        let search = Search::new(dir.path().to_str().unwrap(), SearchConfig::text_only())
            .expect("failed to create Search");
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
            .search("hello", None, |id| Some(id.to_string()))
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
            .search("creat", None, |id| Some(id.to_string()))
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
            .search("creativness", None, |id| Some(id.to_string()))
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
            .search("initial", None, |id| Some(id.to_string()))
            .expect("search failed");
        let new_results: Vec<String> = search
            .search("updated", None, |id| Some(id.to_string()))
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
            .search("hello", None, |id| {
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
            .search("zzznomatch", None, |id| Some(id.to_string()))
            .expect("search failed");

        assert!(results.is_empty());
    }

    /// Searching an empty index should return an empty Vec, not an error.
    #[test]
    fn test_search_on_empty_index() {
        let (search, _dir) = make_search();

        let results: Vec<String> = search
            .search("anything", None, |id| Some(id.to_string()))
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
            .search("message", None, |id| Some(id.to_string()))
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
            .search("hello", None, |id| Some(id.to_string()))
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
            .search("hel.o", None, |id| Some(id.to_string()))
            .expect("search with '.' should not error");
        search
            .search("a|b", None, |id| Some(id.to_string()))
            .expect("search with '|' should not error");
        search
            .search("(hello", None, |id| Some(id.to_string()))
            .expect("search with '(' should not error");
        search
            .search("test[", None, |id| Some(id.to_string()))
            .expect("search with '[' should not error");

        // Without escaping, "zzz(.+)zzz" would be a valid regex capture group.
        // With escaping, it's a literal that matches nothing in our index.
        // It's also too distant from any indexed token for fuzzy to match.
        let regex_results: Vec<String> = search
            .search("zzz(.+)zzz", None, |id| Some(id.to_string()))
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
            .search("anything", None, |id| Some(id.to_string()))
            .expect("search failed");
        assert!(results.is_empty());
    }

    /// tantivy's TEXT index fields are normalized to lowercase, so search should normalize queries.
    #[test]
    fn test_search_is_case_insensitive() {
        let (mut search, _dir) = make_search();
        index_and_commit(&mut search, &[msg("msg-1", "Hello World")]);

        let results: Vec<String> = search
            .search("HELLO", None, |id| Some(id.to_string()))
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
            .search("hello world", None, |id| Some(id.to_string()))
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
            .search("helo wrld", None, |id| Some(id.to_string()))
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
            .search("hello zzzzz", None, |id| Some(id.to_string()))
            .expect("search failed");
        assert!(results.is_empty());
    }

    // -------------------------------------------------------------------------
    // Ranked-path tests
    // -------------------------------------------------------------------------

    /// On a ranked index, documents of equal text relevance must be ordered by recency,
    /// most-recent first. Three rooms share the same name (so relevance is identical),
    /// and only `last_message_at` distinguishes them.
    #[test]
    fn test_ranked_orders_by_relevance_then_recency() {
        let (mut search, _dir) = make_ranked_search();
        index_and_commit(
            &mut search,
            &[
                room("room-old", "weekend plans", 100),
                room("room-new", "weekend plans", 300),
                room("room-mid", "weekend plans", 200),
            ],
        );

        // Preserve search order (do NOT sort) — that order is what we are asserting.
        let results: Vec<String> = search
            .search("weekend", None, |id| Some(id.to_string()))
            .expect("search failed");

        assert_eq!(
            results,
            vec!["room-new", "room-mid", "room-old"],
            "equal-relevance rooms must be ordered newest-first by last_message_at"
        );
    }

    /// A stronger text match must still beat a more recent but weaker match, proving
    /// relevance stays the primary key and recency is only the tiebreak.
    #[test]
    fn test_ranked_relevance_outranks_recency() {
        let (mut search, _dir) = make_ranked_search();
        index_and_commit(
            &mut search,
            &[
                // Exact match for "alpha", but old.
                room("room-exact", "alpha", 1),
                // Only a fuzzy/prefix-distance match for the query, but very recent.
                room("room-fuzzy", "alpine", 9_999),
            ],
        );

        let results: Vec<String> = search
            .search("alpha", None, |id| Some(id.to_string()))
            .expect("search failed");

        assert_eq!(
            results.first().map(String::as_str),
            Some("room-exact"),
            "the exact text match must rank first despite being far less recent"
        );
    }
}
