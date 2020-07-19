use std::{borrow::Cow, collections::HashMap};

fn find_hole_position(template: &str, hole: &str) -> Option<usize> {
    let mut start = 0;
    while let Some(pos) = template[start..].find('{') {
        if template[(pos + 1)..].starts_with(hole)
            && template[(pos + 1 + hole.len()..)].starts_with('}')
        {
            return Some(pos);
        }
        start = pos + 1;
        if pos >= template.len() {
            break;
        }
    }
    return None;
}

pub fn fill<'t, 'c, S: AsRef<str>>(
    template: &'t str,
    context: &'c HashMap<&'static str, S>,
) -> Cow<'t, str> {
    let mut result = Cow::Borrowed(template);
    for (k, v) in context {
        if let Some(pos) = find_hole_position(template, k) {
            result = Cow::Owned(format!(
                "{}{}{}",
                &template[..pos],
                v.as_ref(),
                &template[(pos + k.len() + 2..)]
            ));
        }
    }
    result
}
