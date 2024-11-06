use pest::Parser;
use anyhow::{anyhow, Result};
use spellcasting_parser::*;

#[test]
fn basic_spell_test() -> Result<()> {
    let got = Grammar::parse(Rule::spell, "cast rune flaming ignite")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;

    assert_eq!(got.as_str(), "cast rune flaming ignite");
    assert_eq!(got.as_span().start(), 0);
    assert_eq!(got.as_span().end(), 24);

    let got = Grammar::parse(Rule::spell, "cast rune flaming and swift ignite")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;

    assert_eq!(got.as_str(), "cast rune flaming and swift ignite");

    let got = Grammar::parse(Rule::spell, "invoke self if is burning apply damage")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;

    assert_eq!(got.as_str(), "invoke self if is burning apply damage");

    Ok(())
}

#[test]
fn spell_with_invalid_format() -> Result<()> {
    let pair = Grammar::parse(Rule::spell, "rune flaming ignite");
    assert!(pair.is_err());

    let pair = Grammar::parse(Rule::spell, "cast rune flaming swift ignite");
    assert!(pair.is_err());

    let pair = Grammar::parse(Rule::spell, "cast rune flaming apply");
    assert!(pair.is_err());

    Ok(())
}

#[test]
fn spell_with_multiple_executable_params() -> Result<()> {
    let got = Grammar::parse(Rule::spell, "cast rune flaming and swift ignite also apply heal")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;

    assert_eq!(got.as_str(), "cast rune flaming and swift ignite also apply heal");

    let got = Grammar::parse(Rule::spell, "invoke self if is burning apply damage also apply heal")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;

    assert_eq!(got.as_str(), "invoke self if is burning apply damage also apply heal");

    Ok(())
}

#[test]
fn spell_with_incorrect_modifiers() -> Result<()> {
    let pair = Grammar::parse(Rule::spell, "cast something rune apply heal");
    assert!(pair.is_err());

    let pair = Grammar::parse(Rule::spell, "cast rune if something apply heal");
    assert!(pair.is_err());

    let pair = Grammar::parse(Rule::spell, "cast rune for 10 apply damage");
    assert!(pair.is_err());

    Ok(())
}

#[test]
fn spell_with_optional_modifiers() -> Result<()> {
    let got = Grammar::parse(Rule::spell, "invoke self apply damage")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;

    assert_eq!(got.as_str(), "invoke self apply damage");

    let got = Grammar::parse(Rule::spell, "cast rune powerful apply damage")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;

    assert_eq!(got.as_str(), "cast rune powerful apply damage");

    Ok(())
}

#[test]
fn spell_with_valid_conditions() -> Result<()> {
    let got = Grammar::parse(Rule::spell, "cast rune if is burning apply damage")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;

    assert_eq!(got.as_str(), "cast rune if is burning apply damage");

    let got = Grammar::parse(Rule::spell, "cast rune if is moving apply heal")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;

    assert_eq!(got.as_str(), "cast rune if is moving apply heal");

    Ok(())
}

#[test]
fn spell_with_repetition_and_duration() -> Result<()> {
    let got = Grammar::parse(Rule::spell, "cast rune for 5s apply damage")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;

    assert_eq!(got.as_str(), "cast rune for 5s apply damage");

    let got = Grammar::parse(Rule::spell, "invoke self for 10s apply heal")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;

    assert_eq!(got.as_str(), "invoke self for 10s apply heal");

    Ok(())
}
