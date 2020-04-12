//! Code heavily based on https://github.com/http-rs/tide/blob/4aec5fe2bb6b8202f7ae48e416eeb37345cf029f/backup/examples/staticfile.rs

use async_std::{fs, future, io, task};
use http::{
    header::{self},
    StatusCode,
};
use std::path::{Component, Path, PathBuf};
use std::pin::Pin;
use tide::{Endpoint, Request, Response, Result};

/// A trait that provides a way to get a [`&Path`](std::path::Path) to your static
/// assets directory from your tide app's state. Meant to be used with the
/// [`serve_static_files`] function.
///
/// [`serve_static_files`]: tide_naive_static_files::serve_static_files
///
/// ```no_run
/// use std::path::Path;
/// use tide_naive_static_files::StaticRootDir;
///
/// struct MyState;
///
/// impl StaticRootDir for MyState {
///     fn root_dir(&self) -> &Path {
///         Path::new("./my-static-assets-dir")
///     }
/// }
/// ```
pub trait StaticRootDir {
    fn root_dir(&self) -> &Path;
}

impl<T: StaticRootDir> StaticRootDir for &T {
    fn root_dir(&self) -> &Path {
        (*self).root_dir()
    }
}

async fn stream_bytes(root: PathBuf, actual_path: &str) -> io::Result<Response> {
    let mut path = get_path(&root, actual_path);

    // Loop if the path points to a directory because we want to try looking for
    // an "index.html" file within that directory.
    let (meta, path): (fs::Metadata, PathBuf) = loop {
        let meta = fs::metadata(&path).await.ok();

        // If the file doesn't exist, then bail out.
        if meta.is_none() {
            // ---------------------------------------------
            // changed for qaul.net
            // ---------------------------------------------
            // deliver /index.html file for EmberJS WebGUI routing paths
            
            //println!("not found: {:?}", actual_path);
            
            if 
                actual_path.starts_with("/feed") ||
                actual_path.starts_with("/messenger") ||
                actual_path.starts_with("/users") ||
                actual_path.starts_with("/files") ||
                actual_path.starts_with("/settings") ||
                actual_path.starts_with("/info") ||
                actual_path.starts_with("/register") ||
                actual_path.starts_with("/login")
            {
                path = get_path(&root, "/index.html");
                continue; // Try again.
            } else {
            return Ok(tide::Response::new(StatusCode::NOT_FOUND.as_u16())
                .set_header(header::CONTENT_TYPE.as_str(), mime::TEXT_HTML.as_ref())
                .body_string(format!("Couldn't locate requested file {:?}", actual_path)));
            }
            
            // end of changes
            // ----------------------------------------------
        }
        let meta = meta.unwrap();

        // If the path points to a directory, look for an "index.html" file.
        if !meta.is_file() {
            path.push("index.html");
            continue; // Try again.
        } else {
            break (meta, path);
        }
    };

    let mime = mime_guess::from_path(&path).first_or_octet_stream();
    let size = format!("{}", meta.len());

    // We're done with the checks. Stream file!
    let file = fs::File::open(PathBuf::from(&path)).await.unwrap();
    let reader = io::BufReader::new(file);
    Ok(tide::Response::new(StatusCode::OK.as_u16())
        .body(reader)
        .set_header(header::CONTENT_LENGTH.as_str(), size)
        .set_mime(mime))
}

/// Percent-decode, normalize path components and return the final path joined with root.
/// See https://github.com/iron/staticfile/blob/master/src/requested_path.rs
fn get_path(root: &Path, path: &str) -> PathBuf {
    let rel_path = Path::new(path)
        .components()
        .fold(PathBuf::new(), |mut result, p| {
            match p {
                Component::Normal(x) => result.push({
                    let s = x.to_str().unwrap_or("");
                    &*percent_encoding::percent_decode(s.as_bytes()).decode_utf8_lossy()
                }),
                Component::ParentDir => {
                    result.pop();
                }
                _ => (), // ignore any other component
            }

            result
        });
    root.join(rel_path)
}

/// Use in a tide [`tide::Route::get`](tide::Route::get) handler to serve static files from an
/// endpoint. In order to use this function, your tide app's state must
/// implement the [`StaticRootDir`](tide_naive_static_files::StaticRootDir) trait.
///
/// The static assets will be served from the route provided to the `app.at`
/// function. In the example below, the file `./my-static-asset-dir/foo.html`
/// would be obtainable by making a GET request to
/// `http://my.server.address/static/foo.html`.
///
/// ```no_run
/// use std::path::Path;
/// use tide_naive_static_files::{StaticRootDir, serve_static_files};
///
/// struct MyState;
///
/// impl StaticRootDir for MyState {
///     fn root_dir(&self) -> &Path {
///         Path::new("./my-static-asset-dir")
///     }
/// }
///
/// # fn main() {
/// let state = MyState;
/// let mut app = tide::with_state(state);
/// app.at("static/*path")
///     .get(|req| async { serve_static_files(req).await.unwrap() });
/// # }
/// ```
pub async fn serve_static_files(ctx: Request<impl StaticRootDir>) -> Result {
    let path: String = ctx.param("path").expect(
        "`tide_naive_static_files::serve_static_files` requires a `*path` glob param at the end!",
    );
    let root = ctx.state();
    let resp =
        task::block_on(async move { stream_bytes(PathBuf::from(root.root_dir()), &path).await })
            .unwrap_or_else(|e| {
                eprintln!("tide-naive-static-files internal error: {}", e);
                internal_server_error("Internal server error reading file")
            });

    Ok(resp)
}

/// A struct that holds a path to your app's static assets directory. This
/// struct implements [`tide::Endpoint`](tide::Endpoint) so it can be passed directly to
/// [`tide::Route::get`](tide::Route::get).
///
/// The static assets will be served from the route provided to the `app.at`. In
/// the example below, the file `./my-static-asset-dir/foo.html` would be
/// obtainable by making a GET request to
/// `http://my.server.address/static/foo.html`.
///
/// ```no_run
/// use tide_naive_static_files::StaticFilesEndpoint;
///
/// # fn main() {
/// let mut app = tide::new();
/// app.at("/static").strip_prefix().get(StaticFilesEndpoint {
///     root: "./my-static-asset-dir/".into(),
/// });
/// # }
/// ```
pub struct StaticFilesEndpoint {
    pub root: PathBuf,
}

type BoxFuture<T> = Pin<Box<dyn future::Future<Output = T> + Send>>;

impl<State> Endpoint<State> for StaticFilesEndpoint {
    type Fut = BoxFuture<Response>;

    fn call(&self, ctx: Request<State>) -> Self::Fut {
        let path = ctx.uri().to_string();
        let root = self.root.clone();

        Box::pin(async move {
            stream_bytes(root, &path).await.unwrap_or_else(|e| {
                eprintln!("tide-naive-static-files internal error: {}", e);
                internal_server_error("Internal server error reading file")
            })
        })
    }
}

fn internal_server_error(body: &'static str) -> Response {
    tide::Response::new(StatusCode::INTERNAL_SERVER_ERROR.as_u16())
        .set_header(header::CONTENT_TYPE.as_str(), mime::TEXT_HTML.as_ref())
        .body_string(body.into())
}
