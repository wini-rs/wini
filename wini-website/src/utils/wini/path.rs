/// A macro to concatenate multiple path segments into a single `PathBuf`.
///
/// This macro allows you to easily combine an arbitrary number of path segments
/// (as string literals or string slices) into a single path. It is particularly
/// useful for constructing file paths in a platform-independent manner.
///
///
/// # Parameters
///
/// - `path1, path2, ..., pathN`: One or more path segments to be concatenated.
///   Each segment should be a string literal (`&str`) or a string slice.
///
/// # Returns
///
/// The macro returns a `PathBuf` that represents the concatenated path. You can
/// convert it to a string or use it directly with file system operations.
///
/// # Example
///
/// ```
/// use std::path::PathBuf;
/// use wini::concat_paths;
///
/// // Concatenate multiple path segments
/// let result = concat_paths!("./foo/", "./bar", "./baz/");
///
/// println!("{}", result.display()); // Output: ./foo/bar/baz/
/// ```
#[macro_export]
macro_rules! concat_paths {
    ($($path:expr),+) => {
        {
            use std::path::PathBuf;
            let mut path = PathBuf::new();
            $(
                path.push($path);
            )+
            path
        }
    };
}
