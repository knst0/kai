//! Shared helpers for the live integration tests attached to each query module.
//!
//! These tests talk to the real `api.cdnlibs.org` service. They are gated behind
//! the `live-tests` feature so `cargo test` stays offline and deterministic by
//! default. Run them with:
//!
//! ```text
//! cargo test --features live-tests -- --test-threads=1
//! ```
//!
//! Some endpoints require an account. Set `MANGALIB_TOKEN` to exercise those;
//! without it the authenticated tests skip themselves instead of failing.

use crate::client::Client;

/// A manga that has existed for years and has many chapters, branches,
/// relations and similar titles. Default fixture for title-scoped endpoints.
pub const MANGA_SLUG: &str = "8642--kage-no-jitsuryokusha-ni-naritakute";

/// Numeric id matching [`MANGA_SLUG`].
pub const MANGA_ID: i64 = 8642;

/// A title with an active translation branch, used for branch/chapter tests.
pub const BRANCHED_MANGA_SLUG: &str = "193018--bad-bon-blood";

/// Numeric id matching [`BRANCHED_MANGA_SLUG`].
pub const BRANCHED_MANGA_ID: i64 = 193018;

/// An anime title used for anime-scoped endpoints.
pub const ANIME_SLUG: &str = "16133--jujutsu-kaisen-anime";

/// A genre id that exists in the catalog constants, for filter tests.
pub const GENRE_ID: i64 = 34;

/// The comment endpoints only accept these post types.
pub const COMMENT_POST_TYPE: &str = "manga";

/// A long-lived account used for the public user endpoints.
pub const USER_ID: i64 = 1514956;

/// A bookmark shelf id. `/bookmarks` requires one; 2 is "reading".
pub const BOOKMARK_STATUS: i64 = 2;

/// A slug that is guaranteed not to resolve, for negative tests.
pub const MISSING_SLUG: &str = "0--kai-does-not-exist-abcdefghijklmnop";

/// Builds a client configured for the live test suite.
///
/// [`Client`] is not `Sync`, so each test owns its own instance. The rate limit
/// is deliberately lower than the crate default and the suite is meant to run
/// single-threaded (`--test-threads=1`) so it does not trip upstream throttling.
pub fn client() -> Client {
    let mut builder = Client::builder().rate_limit_per_second(2).rate_limit_per_minute(60);
    if let Ok(token) = std::env::var("MANGALIB_TOKEN") {
        builder = builder.api_token(token);
    }
    builder.build()
}

/// Returns `true` when a usable `MANGALIB_TOKEN` is present.
pub fn has_token() -> bool {
    std::env::var("MANGALIB_TOKEN").is_ok_and(|t| !t.trim().is_empty())
}

/// Returns early from the calling test when no token is configured.
macro_rules! skip_without_token {
    () => {
        if !$crate::queries::test_util::has_token() {
            eprintln!("skipping: MANGALIB_TOKEN is not set");
            return;
        }
    };
}

#[allow(unused_imports)]
pub(crate) use skip_without_token;
