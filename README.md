A [flash](https://github.com/folke/flash.nvim)-style plugin for [Blaze Explorer](https://github.com/tomblazejewski/blaze-explorer)

# App Mappings

## Plugin level

| Default Mappings | Action                                                            |
| ---------------- | ----------------------------------------------------------------- |
| `m`              | Launch search for file (selects the given directory when matched) |
| `M`              | Launch search for file (opens the given directory when matched)   |

## "PopUp" level

The plugin does not issue a visible popup, these keymaps work when the search is active.

| Mappings | Action           |
| -------- | ---------------- |
| `<Esc>`  | Close popup      |
| `<BS>`   | Drop search char |

# Functionalities

- [x] Select given directory
- [x] Open given directory
