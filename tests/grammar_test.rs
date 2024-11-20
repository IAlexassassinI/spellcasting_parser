use spellcasting_parser::*;

#[test]
fn test_single_spell() {
    let input = "cast flaming rune to explode";
    let parsed = parse_string(input).unwrap();

    let expected = Spells {
        spells: vec![Spell {
            invoke_word: "cast".to_string(),
            spell_type_params: SpellTypePart {
                spell_type: "rune".to_string(),
                modifiers: Modifiers {
                    modifiers: vec![Modifier::Adjective {
                        value: "flaming".to_string(),
                    }],
                },
            },
            executable_params: vec![ExecutablePart {
                executable: Executable {
                    value: "explode".to_string(),
                },
                modifiers: Modifiers::default(),
            }],
        }],
    };

    assert_eq!(parsed, expected);
}

#[test]
fn test_spell_with_multiple_modifiers() {
    let input = "cast swift and powerful projectile to powerful and flaming explode";
    let parsed = parse_string(input).unwrap();

    let expected = Spells {
        spells: vec![Spell {
            invoke_word: "cast".to_string(),
            spell_type_params: SpellTypePart {
                spell_type: "projectile".to_string(),
                modifiers: Modifiers {
                    modifiers: vec![
                        Modifier::Adjective {
                            value: "swift".to_string(),
                        },
                        Modifier::Adjective {
                            value: "powerful".to_string(),
                        },
                    ],
                },
            },
            executable_params: vec![ExecutablePart {
                executable: Executable {
                    value: "explode".to_string(),
                },
                modifiers: Modifiers {
                    modifiers: vec![
                        Modifier::Adjective {
                            value: "powerful".to_string(),
                        },
                        Modifier::Adjective {
                            value: "flaming".to_string(),
                        },
                    ],
                },
            }],
        }],
    };

    assert_eq!(parsed, expected);
}

#[test]
fn test_spell_with_repetition() {
    let input = "invoke flaming and 3 of times rune to apply heal";
    let parsed = parse_string(input).unwrap();

    let expected = Spells {
        spells: vec![Spell {
            invoke_word: "invoke".to_string(),
            spell_type_params: SpellTypePart {
                spell_type: "rune".to_string(),
                modifiers: Modifiers {
                    modifiers: vec![
                        Modifier::Adjective {
                            value: "flaming".to_string(),
                        },
                        Modifier::Repetition { value: 3 },
                    ],
                },
            },
            executable_params: vec![ExecutablePart {
                executable: Executable {
                    value: "apply heal".to_string(),
                },
                modifiers: Modifiers::default(),
            }],
        }],
    };

    assert_eq!(parsed, expected);
}

#[test]
fn test_spell_with_condition() {
    let input = "cast if is burning rune to apply heal";
    let parsed = parse_string(input).unwrap();

    let expected = Spells {
        spells: vec![Spell {
            invoke_word: "cast".to_string(),
            spell_type_params: SpellTypePart {
                spell_type: "rune".to_string(),
                modifiers: Modifiers {
                    modifiers: vec![Modifier::Condition {
                        condition_type: "if is burning".to_string(),
                    }],
                },
            },
            executable_params: vec![ExecutablePart {
                executable: Executable {
                    value: "apply heal".to_string(),
                },
                modifiers: Modifiers::default(),
            }],
        }],
    };

    assert_eq!(parsed, expected);
}

#[test]
fn test_complex_spell_with_multiple_executable_params() {
    let input = "invoke swift and frozen rune to swift pull also frozen and powerful apply heal also explode";
    let parsed = parse_string(input).unwrap();

    let expected = Spells {
        spells: vec![Spell {
            invoke_word: "invoke".to_string(),
            spell_type_params: SpellTypePart {
                spell_type: "rune".to_string(),
                modifiers: Modifiers {
                    modifiers: vec![
                        Modifier::Adjective {
                            value: "swift".to_string(),
                        },
                        Modifier::Adjective {
                            value: "frozen".to_string(),
                        },
                    ],
                },
            },
            executable_params: vec![
                ExecutablePart {
                    executable: Executable {
                        value: "pull".to_string(),
                    },
                    modifiers: Modifiers {
                        modifiers: vec![Modifier::Adjective {
                            value: "swift".to_string(),
                        }],
                    },
                },
                ExecutablePart {
                    executable: Executable {
                        value: "apply heal".to_string(),
                    },
                    modifiers: Modifiers {
                        modifiers: vec![
                            Modifier::Adjective {
                                value: "frozen".to_string(),
                            },
                            Modifier::Adjective {
                                value: "powerful".to_string(),
                            },
                        ],
                    },
                },
                ExecutablePart {
                    executable: Executable {
                        value: "explode".to_string(),
                    },
                    modifiers: Modifiers::default(),
                },
            ],
        }],
    };

    assert_eq!(parsed, expected);
}

#[test]
fn test_empty_input() {
    let input = "";
    let parsed = parse_string(input).unwrap();

    let expected = Spells { spells: vec![] };

    assert_eq!(parsed, expected);
}
