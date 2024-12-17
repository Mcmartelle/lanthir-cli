#[cfg(test)]
mod tests {
    use lanthir_cli::checklist::parse_checklist;

    #[test]
    fn all_alone() {
        let ckl = r#"hello
    there
        "#;
        let vec = parse_checklist(&ckl, false).unwrap();
        assert_eq!(vec[0].alone, Some(String::from("hello")));
        assert_eq!(vec[1].alone, Some(String::from("there")));
    }

    #[test]
    fn before_and_after() {
        let ckl = r#"hello ```foo``` there
        "#;
        let vec = parse_checklist(&ckl, false).unwrap();
        assert_eq!(vec[0].before, Some(String::from("hello")));
        assert_eq!(vec[0].wrapper, Some(String::from("foo")));
        assert_eq!(vec[0].after, Some(String::from("there")));
    }
}
