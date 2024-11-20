use anyhow::{anyhow, Result};
use pest::iterators::{Pair, Pairs};
use pest::Parser;
use pest_derive::Parser;
use std::fmt;

#[derive(Parser)]
#[grammar = "./grammar.pest"]
pub struct Grammar;

#[derive(Debug, PartialEq)]
pub struct Spells {
    pub spells: Vec<Spell>,
}

#[derive(Debug, PartialEq)]
pub struct Spell {
    pub invoke_word: String,
    pub spell_type_params: SpellTypePart,
    pub executable_params: Vec<ExecutablePart>,
}

#[derive(Debug, PartialEq)]
pub struct SpellTypePart {
    pub spell_type: String,
    pub modifiers: Modifiers,
}

#[derive(Debug, PartialEq, Default)]
pub struct Modifiers {
    pub modifiers: Vec<Modifier>,
}

#[derive(Debug, PartialEq)]
pub enum Modifier {
    Adjective { value: String },
    Repetition { value: u32 },
    Condition { condition_type: String },
    Duration { value: String },
}

#[derive(Debug, PartialEq)]
pub struct ExecutablePart {
    pub executable: Executable,
    pub modifiers: Modifiers,
}

#[derive(Debug, PartialEq)]
pub struct Executable {
    pub value: String,
}

impl fmt::Display for Spells {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Spells:")?;
        for spell in &self.spells {
            writeln!(f, "{}", spell)?;
        }
        Ok(())
    }
}

impl fmt::Display for Spell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Invoke: {}\nSpell Type: {}\nExecutable Params: {}\n",
            self.invoke_word,
            self.spell_type_params,
            self.executable_params
                .iter()
                .map(|p| p.to_string())
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}

impl fmt::Display for SpellTypePart {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} with modifiers: [{}]",
            self.spell_type,
            self.modifiers
                .modifiers
                .iter()
                .map(|m| m.to_string())
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}

impl fmt::Display for Modifiers {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.modifiers.is_empty() {
            write!(f, "No modifiers")
        } else {
            write!(
                f,
                "{}",
                self.modifiers
                    .iter()
                    .map(|m| m.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            )
        }
    }
}

impl fmt::Display for Modifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Modifier::Adjective { ref value } => write!(f, "Adjective: {}", value),
            Modifier::Repetition { value } => write!(f, "Repetition: {}", value),
            Modifier::Condition { ref condition_type } => {
                write!(f, "Condition: {}", condition_type)
            }
            Modifier::Duration { ref value } => write!(f, "Duration: {}", value),
        }
    }
}

impl fmt::Display for ExecutablePart {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} with modifiers: [{}]",
            self.executable.value,
            self.modifiers
                .modifiers
                .iter()
                .map(|m| m.to_string())
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}

impl fmt::Display for Executable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Executable: {}", self.value)
    }
}

pub fn raw_parse_string(input: &str) -> Result<Pairs<Rule>, anyhow::Error> {
    let parsed_data = Grammar::parse(Rule::spells, input);

    match parsed_data {
        Ok(pairs) => Ok(pairs),
        Err(e) => Err(anyhow::anyhow!("Error during parsing: {}", e)),
    }
}

pub fn parse_string(input: &str) -> Result<Spells, anyhow::Error> {
    let mut error_details = Vec::new();
    let parsed_data = match raw_parse_string(input) {
        Ok(data) => data,
        Err(e) => {
            error_details.push(format!("Raw parsing error: {}", e));
            return Err(anyhow::anyhow!("Parsing failed with multiple errors")
                .context(error_details.join(", ")));
        }
    };

    match parse_to_structure(parsed_data) {
        Ok(spells) => Ok(spells),
        Err(e) => {
            error_details.push(format!("Structure conversion error: {}", e));
            Err(anyhow::anyhow!("Parsing failed with multiple errors")
                .context(error_details.join(", ")))
        }
    }
}

pub fn parse_to_structure(pairs: Pairs<Rule>) -> Result<Spells, anyhow::Error> {
    let mut spells = Vec::new();

    for pair in pairs {
        if pair.as_rule() == Rule::spell {
            let spell = parse_spell(pair)?;
            spells.push(spell);
        }
    }

    Ok(Spells { spells })
}

fn parse_spell(pair: Pair<Rule>) -> Result<Spell, anyhow::Error> {
    let mut invoke_word = String::new();
    let mut spell_type_params = None;
    let mut executable_params = Vec::new();

    for inner_pair in pair.into_inner() {
        match inner_pair.as_rule() {
            Rule::invoke_word => invoke_word = inner_pair.as_str().to_string(),
            Rule::spell_type_params => {
                spell_type_params = Some(parse_spell_type_part(inner_pair)?);
            }
            Rule::executable_params => {
                executable_params.push(parse_executable_part(inner_pair)?);
            }
            _ => {}
        }
    }

    Ok(Spell {
        invoke_word,
        spell_type_params: spell_type_params.ok_or_else(|| anyhow!("Missing spell_type_params"))?,
        executable_params,
    })
}

fn parse_spell_type_part(pair: Pair<Rule>) -> Result<SpellTypePart, anyhow::Error> {
    let mut spell_type = String::new();
    let mut modifiers = Modifiers::default();

    for inner_pair in pair.into_inner() {
        match inner_pair.as_rule() {
            Rule::spell_type => spell_type = inner_pair.as_str().to_string(),
            Rule::modifiers => modifiers = parse_modifiers(inner_pair)?,
            _ => {}
        }
    }

    Ok(SpellTypePart {
        spell_type,
        modifiers,
    })
}

fn parse_modifiers(pair: Pair<Rule>) -> Result<Modifiers, anyhow::Error> {
    let mut modifiers = Vec::new();

    for inner_pair in pair.into_inner() {
        match inner_pair.as_rule() {
            Rule::adjective => modifiers.push(Modifier::Adjective {
                value: inner_pair.as_str().to_string(),
            }),
            Rule::condition => modifiers.push(Modifier::Condition {
                condition_type: inner_pair.as_str().to_string(),
            }),
            Rule::repetition => modifiers.push(Modifier::Repetition {
                value: parse_repetition(inner_pair)?,
            }),
            Rule::duration => modifiers.push(Modifier::Duration {
                value: inner_pair.as_str().to_string(),
            }),
            _ => {}
        }
    }
    Ok(Modifiers { modifiers })
}

fn parse_repetition(pair: Pair<Rule>) -> Result<u32> {
    if let Some(inner_pair) = pair.into_inner().next() {
        match inner_pair.as_rule() {
            Rule::number => {
                let value = inner_pair
                    .as_str()
                    .parse::<u32>()
                    .map_err(|e| anyhow!("Failed to parse number: {}", e))?;
                return Ok(value);
            }
            _ => {
                return Err(anyhow!(
                    "Unexpected rule inside repetition: {:?}",
                    inner_pair.as_rule()
                ));
            }
        }
    }
    Err(anyhow!("No number found in repetition"))
}

fn parse_executable_part(pair: Pair<Rule>) -> Result<ExecutablePart, anyhow::Error> {
    let mut executable = None;
    let mut modifiers = Modifiers::default();

    for inner_pair in pair.into_inner() {
        match inner_pair.as_rule() {
            Rule::executable => {
                executable = Some(Executable {
                    value: inner_pair.as_str().to_string(),
                });
            }
            Rule::modifiers => modifiers = parse_modifiers(inner_pair)?,
            _ => {}
        }
    }

    Ok(ExecutablePart {
        executable: executable.ok_or_else(|| anyhow!("Missing executable"))?,
        modifiers,
    })
}
