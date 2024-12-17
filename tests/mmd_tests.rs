#[cfg(test)]
mod tests {
    use lanthir_cli::mermaid::parse_mermaid;

    #[test]
    fn simple_flowchart() {
        let mmd = r#"flowchart TD
    A-->B
        "#;
        let graph = parse_mermaid(&mmd, false).unwrap();
        assert_eq!(graph["A"].label, None);
        assert_eq!(graph["A"].outputs[0].destination, "B");
    }

    #[test]
    fn labels() {
        let mmd = r#"flowchart TD
    A[apple]-->|foo|B{banana}
        "#;
        let graph = parse_mermaid(&mmd, false).unwrap();
        assert_eq!(graph["A"].label, Some(String::from("apple")));
        assert_eq!(graph["B"].label, Some(String::from("banana")));
        assert_eq!(graph["A"].outputs[0].label, Some(String::from("foo")));
        assert_eq!(graph["A"].outputs[0].destination, "B");
    }

    #[test]
    fn symbols() {
        let mmd = r#"flowchart TD
    A--Torture Test-->B("run[-_(){}'~./\|&;<>$`,:@%^*+=?{}!]")
        "#;
        let graph = parse_mermaid(&mmd, false).unwrap();
        assert_eq!(graph["A"].label, None);
        assert_eq!(graph["B"].label, None);
        assert_eq!(
            graph["B"].cmd,
            Some(String::from("-_(){}'~./\\|&;<>$`,:@%^*+=?{}!"))
        );
        assert_eq!(
            graph["A"].outputs[0].label,
            Some(String::from("Torture Test"))
        );
        assert_eq!(graph["A"].outputs[0].destination, "B");
    }

    #[test]
    fn command_test() {
        let mmd = r#"flowchart TD
    A--CMD Test-->B["run[echo #quot;hello#quot;]"]
        "#;
        let graph = parse_mermaid(&mmd, false).unwrap();
        assert_eq!(graph["A"].label, None);
        assert_eq!(graph["B"].label, None);
        assert_eq!(graph["B"].cmd, Some(String::from("echo \"hello\"")));
        assert_eq!(graph["A"].outputs[0].label, Some(String::from("CMD Test")));
        assert_eq!(graph["A"].outputs[0].destination, "B");
    }
}
