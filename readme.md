
<h1 align="center">mayfig</h1>
<p align="center">a pretty mediocre config format</p>


### example

```properties
input {
    keyboard {
        # xkb_file = "~/.config/keymap/may.xkb"

        repeat_delay = 600
        repeat_rate = 25
    }

    touchpad {
        tap = true

        natural_scroll = true
        # scroll_method = "two_finger"
    }
}

cursor {
    xcursor_theme = "Bibata-Modern-Classic"
    xcursor_size = 24
}

bind {
    "mod escape" = "quit"
    "mod q" = "close"

    "mod t" = "spawn" [ "kitty" ]
    "mod n" = "spawn" [ "firefox" ]
}
```

### spec

`mayfig` is fundamentally a `key=value` style format.

**comments**

```properties
# comments start with a hashtag

# multiline comments need a hashtag
# in front of every line
```

**categories**

```properties
# similar to hyprlang and the sway config format,
# mayfig categories can be created by creating a scoped
# block behind the key with curly braces
#
# categories are to mayfig what tables are to
# toml, objects are to json and mappings are to yaml
#
# all key=value pairs in categories must be on their
# own seperate line
layout {
    thing = true

    # you can even add nested categories
    tiling {
        gaps = 10
        margin = 20
    }
}
```

**sequences**

```properties
# sequences are to mayfig what arrays are to json and toml
# they can be created with square brackets
size = [ 24 24 ]

# while the commata are optional,
# you can still set them if you prefer
offset = [ 0, 0 ]

# if you have a lot of values you can split them up into seperate
# lines, but the opening brackets have to be on the same line as the
# `=` sign.
tiling_exceptions = [
    "com.system76.CosmicFilesDialog"
    "com.system76.CosmicFiles"
    "jetbrains-toolbox"
    "blueman-manager"
]
```

**strings**

```properties
# strings have to be escaped with quotes
# single and double quoted strings are the
# same and do not make a difference
st1 = "this is a string"
st2 = 'single quoted strings work too'

map {
    # keys are also strings, but quotes are optional,
    # if you restrict yourself to /[a-zA-Z_][a-zA-Z0-9_]*/
    unquoted = true
    "with quotes" = true
}
```

**numbers**

```properties
# integers
i1 = -1
i2 = 3

# floats
f1 = 1.0
f2 = -2.5

# special
s1 = .inf # positive infinity
s2 = -.inf # negative infinity
# nan is not a supported value
```

**tagged enums**

```properties
# in mayfig you can "tag" values by putting the values
# in brackets behind the string tag
thing = "tag" [ "value" ]

# this is how enums are defined in mayfig
# by omitting the value you can create unit enum variants
bind {
    "mod q" = "close"
    "mod t" = "spawn" [ "kitty" ]
    "mod 0" = "workspace" [ 0 ]
}

# you can also use them in category keys
# there you can even omit the quotes around the tag
windowrules {
    class [ "com.system76.CosmicFiles" ] {
        floating = true
        size = [ 1000 700 ]
    }

    title [ "maym ~" ] {
        opacity = 0.6
    }
}
```

### usage

`mayland.mf`
```properties
cursor {
    xcursor_theme = "Bibata-Modern-Classic"
    xcursor_size = 24
}

bind {
    "mod q" = "close"
    "mod t" = "spawn" [ "kitty" ]
    "mod 0" = "workspace" [ 0 ]
}
```

`Cargo.toml`
```toml
[dependencies]
mayfig = { git = "https://github.com/m4rch3n1ng/mayfig" }
serde = { version = "1", features = ["derive"] }
```

`src/main.rs`
```rust
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
struct Mayland {
    cursor: Cursor,
    bind: HashMap<String, Action>,
}

#[derive(Debug, Deserialize)]
struct Cursor {
    xcursor_theme: String,
    xcursor_size: u32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
enum Action {
    Close,
    Spawn(String),
    Workspace(usize),
}

fn main() {
    let content = std::fs::read_to_string("mayland.mf").unwrap();
    let mayland = mayfig::from_str::<Mayland>(&content).unwrap();
    dbg!(mayland);
}
```

### background

mayfig was made for the [mayland](https://github.com/m4rch3n1ng/mayland) wayland compositor
