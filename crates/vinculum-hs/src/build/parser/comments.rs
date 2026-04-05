#[inline]
pub(crate) fn is_only_comment(line: &str) -> Option<&str> {
    let clean_line = line.trim_start();

    if let Some((pre, post)) = clean_line.split_once("--")
        && pre.is_empty()
        && !post.is_empty()
    {
        return Some(post.trim());
    }

    None
}
