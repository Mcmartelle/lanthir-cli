#[cfg(test)]
mod tests {
    use lanthir_cli::oats::{parse_oats, Groat, Marker};
    use lanthir_cli::oats_runner::{groats_to_oatlets, Oatlet};

    #[test]
    fn and_then() {
        let oats = r#"~ hello
        "#;
        let grains = parse_oats(&oats, false).unwrap();
        assert_eq!(grains[0].marker, Some(Marker::AndThen));
        assert_eq!(grains[0].content, Some(String::from("hello")));
    }

    #[test]
    fn optional() {
        let oats = r#"? hello
        "#;
        let grains = parse_oats(&oats, false).unwrap();
        assert_eq!(grains[0].marker, Some(Marker::Optional));
        assert_eq!(grains[0].content, Some(String::from("hello")));
    }

    #[test]
    fn one_of() {
        let oats = r#"| hello
        "#;
        let grains = parse_oats(&oats, false).unwrap();
        assert_eq!(grains[0].marker, Some(Marker::OneOf));
        assert_eq!(grains[0].content, Some(String::from("hello")));
    }

    #[test]
    fn unordered() {
        let oats = r#"& hello
        "#;
        let grains = parse_oats(&oats, false).unwrap();
        assert_eq!(grains[0].marker, Some(Marker::Unordered));
        assert_eq!(grains[0].content, Some(String::from("hello")));
    }

    #[test]
    fn all_together() {
        let oats = r#"& hello
    & there // comment
    // this comment shouldn't change anything

    | foo
    // this comment shouldn't change anything
    ~ bar
    ? baz
        "#;
        let grains = parse_oats(&oats, false).unwrap();
        assert_eq!(grains[0].marker, Some(Marker::Unordered));
        assert_eq!(grains[0].content, Some(String::from("hello")));
        assert_eq!(grains[1].marker, Some(Marker::Unordered));
        assert_eq!(grains[1].content, Some(String::from("there")));
        assert_eq!(grains[2].marker, Some(Marker::Breaker));
        assert_eq!(grains[2].content, None);
        assert_eq!(grains[3].marker, Some(Marker::OneOf));
        assert_eq!(grains[3].content, Some(String::from("foo")));
        assert_eq!(grains[4].marker, Some(Marker::AndThen));
        assert_eq!(grains[4].content, Some(String::from("bar")));
        assert_eq!(grains[5].marker, Some(Marker::Optional));
        assert_eq!(grains[5].content, Some(String::from("baz")));
    }

    #[test]
    fn clipboard_groats() {
        let oats = r#"& hello
    & there // comment
    // this comment shouldn't change anything
    = blah blah

    | foo
    // this comment shouldn't change anything
    ~ bar
        "#;
        let grains = parse_oats(&oats, false).unwrap();
        assert_eq!(grains[0].marker, Some(Marker::Unordered));
        assert_eq!(grains[0].content, Some(String::from("hello")));
        assert_eq!(grains[1].marker, Some(Marker::Unordered));
        assert_eq!(grains[1].content, Some(String::from("there")));
        assert_eq!(grains[2].marker, Some(Marker::Clipbo));
        assert_eq!(grains[2].content, Some(String::from("blah blah")));
        assert_eq!(grains[3].marker, Some(Marker::Breaker));
        assert_eq!(grains[3].content, None);
        assert_eq!(grains[4].marker, Some(Marker::OneOf));
        assert_eq!(grains[4].content, Some(String::from("foo")));
        assert_eq!(grains[5].marker, Some(Marker::AndThen));
        assert_eq!(grains[5].content, Some(String::from("bar")));
    }

    #[test]
    fn clipboard_oatlets() {
        let oats = r#"& hello
    & there // comment
    // this comment shouldn't change anything
    = blah blah

    | foo
    // this comment shouldn't change anything
    ~ bar
        "#;
        let groats: Vec<Groat> = parse_oats(&oats, false).unwrap();
        let oatlets: Vec<Oatlet> = groats_to_oatlets(&groats);
        assert_eq!(oatlets[0].marker, Marker::Unordered);
        assert_eq!(oatlets[0].content, Some(String::from("hello")));
        assert_eq!(oatlets[0].clipboard, None);
        assert_eq!(oatlets[0].done, false);
        assert_eq!(oatlets[1].marker, Marker::Unordered);
        assert_eq!(oatlets[1].content, Some(String::from("there")));
        assert_eq!(oatlets[1].clipboard, Some(String::from("blah blah")));
        assert_eq!(oatlets[2].marker, Marker::Breaker);
        assert_eq!(oatlets[2].content, None);
        assert_eq!(oatlets[3].marker, Marker::OneOf);
        assert_eq!(oatlets[3].content, Some(String::from("foo")));
        assert_eq!(oatlets[4].marker, Marker::AndThen);
        assert_eq!(oatlets[4].content, Some(String::from("bar")));
    }
}
