# macroquest-rs

Create MacroQuest Plugins using idiomatic and safe Rust.


## FAQ

### Setup Building with Cargo

In order to build a macroquest-rs based plugin, you need to have a checkout of
[MacroQuest](https://github.com/macroquest/macroquest/) that has already been
built.

Whenever building a macroquest-rs based plugin, you need to set *at least* the
``MACROQUEST_DIR`` environment variable to the location of that checkout. This
can be done automatically for cargo by creating a ``.cargo/config.toml`` file
with contents like:

```toml
[env]
MACROQUEST_DIR = "C:\\Users\\UserName\\Projects\\MacroQuest"
```

With that, ``cargo build`` should be able to correctly locate the MacroQuest
directory and build against it.

### Setup rust-analyzer in VSCode

Because our crate needs to link against MacroQuest, specifically against
``MQ2Main`` in order just to build, we need to teach ``rust-analyzer`` how to
locate the ``MQ2Main.dll`` that we've built.

With Windows, the primary way of doing that is setting the ``PATH`` environment
variable, which we can do in vscode by creating a ``.vscode/settings.json``
file that looks like:

```json
{
    "rust-analyzer.server.extraEnv": {
        "PATH": "C:\\Users\\UserName\\Projects\\MacroQuest\\build\\bin\\release;${env:PATH}"
    }
}

```

With that, ``rust-analyzer`` will be able to correctly locate the ``MQ2Main.dll``
file and will be able to successfully analyize your plugins.


### Install a plugin

When building with Cargo, the DLL will end up in either ``target/debug/`` or in
``target/release/`` depending on if you're building in the debug or release
profile.

Currently, you can install a generated plugin by copying the DLL out of this
directory and into the ``$MACROQUEST/plugins/`` directory.
