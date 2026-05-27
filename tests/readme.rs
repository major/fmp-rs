#[test]
fn library_dependency_example_present() {
    let readme = include_str!("../README.md");
    let crate_docs = include_str!("../src/lib.rs");
    let expected = "rusty-fmp = { default-features = false }";

    assert!(
        readme.contains(expected),
        "README library dependency example missing; expected `{expected}`"
    );
    assert!(
        crate_docs.contains(&format!("//! {expected}")),
        "crate-level library dependency example missing; expected `//! {expected}`"
    );
}

#[test]
fn schema_example_has_string_preferred_path() {
    let readme = include_str!("../README.md");

    // The doc example must show preferred_path as a string, not an array.
    assert!(
        readme.contains(r#""preferred_path": "market quote""#),
        "README schema example must show preferred_path as a string 'market quote'"
    );
    assert!(
        !readme.contains(r#""preferred_path": ["#),
        "README schema example must NOT show preferred_path as an array"
    );
}

#[test]
fn schema_docs_describe_flat_commands() {
    let readme = include_str!("../README.md");

    // The prose description must call commands a flat array, not group objects.
    assert!(
        readme.contains("flat array of leaf command entries"),
        "README schema docs must describe commands as a flat array of leaf command entries"
    );
    assert!(
        !readme.contains("list of group objects"),
        "README schema docs must NOT describe commands as group objects"
    );
}
