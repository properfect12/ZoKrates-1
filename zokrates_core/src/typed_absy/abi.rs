use typed_absy::types::ConcreteSignature;
use typed_absy::types::ConcreteType;

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct AbiInput {
    pub name: String,
    pub public: bool,
    #[serde(flatten)]
    pub ty: ConcreteType,
}

pub type AbiOutput = ConcreteType;

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct Abi {
    pub inputs: Vec<AbiInput>,
    pub outputs: Vec<AbiOutput>,
}

impl Abi {
    pub fn signature(&self) -> ConcreteSignature {
        ConcreteSignature {
            inputs: self.inputs.iter().map(|i| i.ty.clone()).collect(),
            outputs: self.outputs.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use typed_absy::types::{
        ConcreteArrayType, ConcreteFunctionKey, ConcreteStructMember, ConcreteStructType,
    };
    use typed_absy::{
        parameter::DeclarationParameter, variable::DeclarationVariable, ConcreteType,
        TypedFunction, TypedFunctionSymbol, TypedModule, TypedProgram,
    };
    use zokrates_field::Bn128Field;

    #[test]
    fn generate_abi_from_typed_ast() {
        let mut functions = HashMap::new();
        functions.insert(
            ConcreteFunctionKey::with_id("main").into(),
            TypedFunctionSymbol::Here(TypedFunction {
                arguments: vec![
                    DeclarationParameter {
                        id: DeclarationVariable::field_element("a"),
                        private: true,
                    }
                    .into(),
                    DeclarationParameter {
                        id: DeclarationVariable::boolean("b"),
                        private: false,
                    }
                    .into(),
                ],
                statements: vec![],
                signature: ConcreteSignature::new()
                    .inputs(vec![ConcreteType::FieldElement, ConcreteType::Boolean])
                    .outputs(vec![ConcreteType::FieldElement])
                    .into(),
            }),
        );

        let mut modules = HashMap::new();
        modules.insert("main".into(), TypedModule { functions });

        let typed_ast: TypedProgram<Bn128Field> = TypedProgram {
            main: "main".into(),
            modules,
        };

        let abi: Abi = typed_ast.abi().unwrap();
        let expected_abi = Abi {
            inputs: vec![
                AbiInput {
                    name: String::from("a"),
                    public: false,
                    ty: ConcreteType::FieldElement,
                },
                AbiInput {
                    name: String::from("b"),
                    public: true,
                    ty: ConcreteType::Boolean,
                },
            ],
            outputs: vec![ConcreteType::FieldElement],
        };

        assert_eq!(expected_abi, abi);
    }

    #[test]
    fn serialize_empty() {
        let abi: Abi = Abi {
            inputs: vec![],
            outputs: vec![],
        };

        let json = serde_json::to_string(&abi).unwrap();
        assert_eq!(&json, r#"{"inputs":[],"outputs":[]}"#)
    }

    #[test]
    fn serialize_field() {
        let abi: Abi = Abi {
            inputs: vec![
                AbiInput {
                    name: String::from("a"),
                    public: true,
                    ty: ConcreteType::FieldElement,
                },
                AbiInput {
                    name: String::from("b"),
                    public: true,
                    ty: ConcreteType::FieldElement,
                },
            ],
            outputs: vec![ConcreteType::FieldElement],
        };

        let json = serde_json::to_string_pretty(&abi).unwrap();
        assert_eq!(
            &json,
            r#"{
  "inputs": [
    {
      "name": "a",
      "public": true,
      "type": "field"
    },
    {
      "name": "b",
      "public": true,
      "type": "field"
    }
  ],
  "outputs": [
    {
      "type": "field"
    }
  ]
}"#
        )
    }

    #[test]
    fn serialize_struct() {
        let abi: Abi = Abi {
            inputs: vec![AbiInput {
                name: String::from("foo"),
                public: true,
                ty: ConcreteType::Struct(ConcreteStructType::new(
                    "".into(),
                    "Foo".into(),
                    vec![
                        ConcreteStructMember::new(String::from("a"), ConcreteType::FieldElement),
                        ConcreteStructMember::new(String::from("b"), ConcreteType::Boolean),
                    ],
                )),
            }],
            outputs: vec![ConcreteType::Struct(ConcreteStructType::new(
                "".into(),
                "Foo".into(),
                vec![
                    ConcreteStructMember::new(String::from("a"), ConcreteType::FieldElement),
                    ConcreteStructMember::new(String::from("b"), ConcreteType::Boolean),
                ],
            ))],
        };

        let json = serde_json::to_string_pretty(&abi).unwrap();
        assert_eq!(
            &json,
            r#"{
  "inputs": [
    {
      "name": "foo",
      "public": true,
      "type": "struct",
      "components": {
        "name": "Foo",
        "members": [
          {
            "name": "a",
            "type": "field"
          },
          {
            "name": "b",
            "type": "bool"
          }
        ]
      }
    }
  ],
  "outputs": [
    {
      "type": "struct",
      "components": {
        "name": "Foo",
        "members": [
          {
            "name": "a",
            "type": "field"
          },
          {
            "name": "b",
            "type": "bool"
          }
        ]
      }
    }
  ]
}"#
        )
    }

    #[test]
    fn serialize_nested_struct() {
        let abi: Abi = Abi {
            inputs: vec![AbiInput {
                name: String::from("foo"),
                public: true,
                ty: ConcreteType::Struct(ConcreteStructType::new(
                    "".into(),
                    "Foo".into(),
                    vec![ConcreteStructMember::new(
                        String::from("bar"),
                        ConcreteType::Struct(ConcreteStructType::new(
                            "".into(),
                            "Bar".into(),
                            vec![
                                ConcreteStructMember::new(
                                    String::from("a"),
                                    ConcreteType::FieldElement,
                                ),
                                ConcreteStructMember::new(
                                    String::from("b"),
                                    ConcreteType::FieldElement,
                                ),
                            ],
                        )),
                    )],
                )),
            }],
            outputs: vec![],
        };

        let json = serde_json::to_string_pretty(&abi).unwrap();
        assert_eq!(
            &json,
            r#"{
  "inputs": [
    {
      "name": "foo",
      "public": true,
      "type": "struct",
      "components": {
        "name": "Foo",
        "members": [
          {
            "name": "bar",
            "type": "struct",
            "components": {
              "name": "Bar",
              "members": [
                {
                  "name": "a",
                  "type": "field"
                },
                {
                  "name": "b",
                  "type": "field"
                }
              ]
            }
          }
        ]
      }
    }
  ],
  "outputs": []
}"#
        )
    }

    #[test]
    fn serialize_struct_array() {
        let abi: Abi = Abi {
            inputs: vec![AbiInput {
                name: String::from("a"),
                public: false,
                ty: ConcreteType::Array(ConcreteArrayType::new(
                    ConcreteType::Struct(ConcreteStructType::new(
                        "".into(),
                        "Foo".into(),
                        vec![
                            ConcreteStructMember::new(
                                String::from("b"),
                                ConcreteType::FieldElement,
                            ),
                            ConcreteStructMember::new(String::from("c"), ConcreteType::Boolean),
                        ],
                    )),
                    2,
                )),
            }],
            outputs: vec![ConcreteType::Boolean],
        };

        let json = serde_json::to_string_pretty(&abi).unwrap();
        assert_eq!(
            &json,
            r#"{
  "inputs": [
    {
      "name": "a",
      "public": false,
      "type": "array",
      "components": {
        "size": 2,
        "type": "struct",
        "components": {
          "name": "Foo",
          "members": [
            {
              "name": "b",
              "type": "field"
            },
            {
              "name": "c",
              "type": "bool"
            }
          ]
        }
      }
    }
  ],
  "outputs": [
    {
      "type": "bool"
    }
  ]
}"#
        )
    }

    #[test]
    fn serialize_multi_dimensional_array() {
        let abi: Abi = Abi {
            inputs: vec![AbiInput {
                name: String::from("a"),
                public: false,
                ty: ConcreteType::Array(ConcreteArrayType::new(
                    ConcreteType::Array(ConcreteArrayType::new(ConcreteType::FieldElement, 2)),
                    2,
                )),
            }],
            outputs: vec![ConcreteType::FieldElement],
        };

        let json = serde_json::to_string_pretty(&abi).unwrap();
        assert_eq!(
            &json,
            r#"{
  "inputs": [
    {
      "name": "a",
      "public": false,
      "type": "array",
      "components": {
        "size": 2,
        "type": "array",
        "components": {
          "size": 2,
          "type": "field"
        }
      }
    }
  ],
  "outputs": [
    {
      "type": "field"
    }
  ]
}"#
        )
    }
}
