# Lil' UFO

A UFO font file manipulation tool that provides various commands for viewing and editing UFO font files.

<img src="docs/lilufo-logo.png" alt="Lil' UFO Logo" width="150" />

## Features

- Display basic font information
- Round all glyph points to even integers
- View and manage kerning groups
- View and add kerning pairs

## Installation

### Requirements

- Rust and Cargo (1.53.0 or newer)

### Building from source

```bash
# Clone the repository
git clone https://github.com/yourusername/lilufo.git
cd lilufo

# Build the project
cargo build --release

# The binary will be available at target/release/lilufo
```

## Usage

Lil' UFO uses a subcommand structure. The basic syntax is:

```bash
lilufo --ufo-path <path-to-ufo-file> <subcommand> [options]
```

### Available Commands

#### Display Basic Information

Shows basic information about the font file including family name, version, and glyph count.

```bash
lilufo --ufo-path MyFont.ufo basic-info
```

#### Round Points to Even Integers

Rounds all points in all glyphs to the nearest even integer.

```bash
lilufo --ufo-path MyFont.ufo round-to-even
```

#### View Kerning Groups

Displays all kerning groups defined in the font.

```bash
lilufo --ufo-path MyFont.ufo show-kerning-groups
```

#### View Kerning Pairs

Displays all kerning pairs defined in the font.

```bash
lilufo --ufo-path MyFont.ufo show-kerning
```

#### Add Kerning Group

Creates a new kerning group with specified members.

```bash
lilufo --ufo-path MyFont.ufo add-kerning-group \
  --name "ROUND_LEFT" \
  --side "left" \
  --members "O,Q,C,G"
```

#### Edit Kerning Group

Modifies an existing kerning group.

```bash
lilufo --ufo-path MyFont.ufo edit-kerning-group \
  --name "ROUND_LEFT" \
  --side "left" \
  --members "O,Q,C,G,D" \
  --append
```

Use `--append` to add to the existing members instead of replacing them.

#### Add Kerning Pair

Adds a new kerning pair between glyphs or groups.

```bash
lilufo --ufo-path MyFont.ufo add-kerning-pair \
  --first "ROUND_LEFT" \
  --second "T" \
  --value -80
```

## Testing

Run the test suite with:

```bash
# Run all tests
cargo test

# Run a specific test
cargo test -- commands::add_kerning_pair
```

## Examples

### Typical Workflow

```bash
# First, examine the font information
lilufo --ufo-path MyFont.ufo basic-info

# Look at existing kerning groups
lilufo --ufo-path MyFont.ufo show-kerning-groups

# Add a new kerning group
lilufo --ufo-path MyFont.ufo add-kerning-group --name "ROUND_LEFT" --side "left" --members "O,Q,C,G"

# Add kerning between this group and another glyph
lilufo --ufo-path MyFont.ufo add-kerning-pair --first "ROUND_LEFT" --second "T" --value -80

# Verify the new kerning
lilufo --ufo-path MyFont.ufo show-kerning
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.