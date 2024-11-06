### Spellcasting Parser

Spellcasting Parser is designed to parse and validate spellcasting commands. It allows users to create and validate spells using a variety of components, including modifiers, spell types, and executable actions.

### Parsing Process
## Grammar Overview

The parsing process involves reading a spellcasting string and breaking it down into its components according to the following rules:
    1 Invoke Word: The command begins with either cast or invoke, indicating that the user is casting or invoking a spell.
    2 Spell Type Parameters: After the invoke word, the parser checks for a spell type (e.g., rune, projectile, self, touch). Optionally, modifiers can be attached to modify the spell.
    3 Modifiers: Modifiers are descriptive elements that can add additional characteristics to the spell.
    4 Executable Parameters: These are the actions performed by the spell, such as ignite, apply damage, push, or explode. Additional modifiers may be added to executable parameters to further define their behavior.

## Future use
    
Once the spell is parsed and validated, the resulting data can be used in various ways, for example in a game. The parsed information can be used to construct and execute spell behavior.