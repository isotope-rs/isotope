pub mod iam;
pub mod sts;

fn remove_whitespace(s: &mut String) {
    s.retain(|c| !c.is_whitespace());
}
