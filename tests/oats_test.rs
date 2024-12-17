#[cfg(test)]
mod tests {
    use lanthir_cli::oats::{parse_oats, Marker};

    #[test]
    fn and_then() {
        let oats = r#"~ hello
        "#;
        let grains = parse_oats(&oats, false).unwrap();
        assert_eq!(grains[0].marker, Some(Marker::AndThen));
        assert_eq!(grains[0].content, Some(String::from("hello")));
        assert_eq!(grains[0].done, false);
    }

    #[test]
    fn one_of() {
        let oats = r#"| hello
        "#;
        let grains = parse_oats(&oats, false).unwrap();
        assert_eq!(grains[0].marker, Some(Marker::OneOf));
        assert_eq!(grains[0].content, Some(String::from("hello")));
        assert_eq!(grains[0].done, false);
    }

    #[test]
    fn unordered() {
        let oats = r#"& hello
        "#;
        let grains = parse_oats(&oats, false).unwrap();
        assert_eq!(grains[0].marker, Some(Marker::Unordered));
        assert_eq!(grains[0].content, Some(String::from("hello")));
        assert_eq!(grains[0].done, false);
    }

    #[test]
    fn all_together() {
        let oats = r#"& hello
    & there

    | foo
    ~ bar
        "#;
        let grains = parse_oats(&oats, false).unwrap();
        assert_eq!(grains[0].marker, Some(Marker::Unordered));
        assert_eq!(grains[0].content, Some(String::from("hello")));
        assert_eq!(grains[0].done, false);
        assert_eq!(grains[1].marker, Some(Marker::Unordered));
        assert_eq!(grains[1].content, Some(String::from("there")));
        assert_eq!(grains[1].done, false);
        assert_eq!(grains[2].marker, Some(Marker::Breaker));
        assert_eq!(grains[2].content, None);
        assert_eq!(grains[2].done, false);
        assert_eq!(grains[3].marker, Some(Marker::OneOf));
        assert_eq!(grains[3].content, Some(String::from("foo")));
        assert_eq!(grains[3].done, false);
        assert_eq!(grains[4].marker, Some(Marker::AndThen));
        assert_eq!(grains[4].content, Some(String::from("bar")));
        assert_eq!(grains[4].done, false);
    }
}
