use std::collections::HashMap;

#[derive(Deserialize, Debug, PartialEq)]
struct Config {
    variables: HashMap<String, Definition>,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(tag = "type")]
enum Definition {
    #[serde(rename = "str")]
    StringType(DefinitionType<String>),
    #[serde(rename = "int")]
    IntType(DefinitionType<i64>),
}

#[derive(Deserialize, Debug, PartialEq)]
struct DefinitionType<T> {
    default: Option<T>,
    choices: Option<Vec<Choice<T>>>,
}

#[derive(Deserialize, Debug, PartialEq)]
struct Choice<T> {
    label: Option<String>,
    value: T,
}

#[test]
fn test_load_config() {
    let config_s = r#"
    [variables.foo]
    type = 'int'
    default = 2
    choices = [
        {label = 'One', value = 1},
        {label = 'Two', value = 2},
        {label = 'Three', value = 3},
    ]

    [variables.bar]
    type= 'str'
    "#;
    let config: Config = toml::from_str(config_s).unwrap();
    assert_eq!(
        config,
        Config {
            variables: hashmap! {
                "foo".to_string() => Definition::IntType(DefinitionType{
                    default: Some(2),
                    choices: Some(vec![
                        Choice{label: Some("One".to_string()), value: 1},
                        Choice{label: Some("Two".to_string()), value: 2},
                        Choice{label: Some("Three".to_string()), value: 3},
                    ]),
                }),
                "bar".to_string() => Definition::StringType(DefinitionType{
                    default: None,
                    choices: None,
                }),
            },
        }
    );
}
