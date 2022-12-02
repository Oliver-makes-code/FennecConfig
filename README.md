# Fennec Config

A config designed around readability

```fennec
# Basic k/v pairings
key1 = "value"
key2 = 15

# Type hints to guide on what values are expected
# These are not checked when parsing, they only serve to guide people reading the config
# As they're not checked, they can contain any text, as long as it's not a : or =
key1: string = "value"
key2: number = 15

# Nested k/v pairs
someNest { # Note, no equal sign is needed
    key1: string = "owo"
    key2: string = "nya"
}

# Array of values
someArray [
    "owo" "uwu"
]

# Multiline strings
someString: string = """
    awoo nya owo uwu owo uwu uwu nya awoo.
    nya nya nya owo uwu owo owo owo.
"""

# Put a - before a multiline string to trim indents
someOtherString: string = -"""
    owo
    uwu
"""

# Primitive types
strings [
    "owo"
    """
        owo
    """
    -"""
        uwu
    """
]
number [
    5915587277 # Int
    6.2831853071 # Float
    0x45 # Hex literal
    1x45 # Sign bit supported
    0o105 # Octal literal
    0b1000101 # Binary literal
]
boolean [
    true # true/false
    False # True/False
    1b # 1b/0b
]
```