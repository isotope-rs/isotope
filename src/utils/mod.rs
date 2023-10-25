pub mod sts;
pub mod iam;


fn remove_whitespace(s: &mut String) {
    s.retain(|c| !c.is_whitespace());
}
