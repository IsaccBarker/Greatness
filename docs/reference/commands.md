# Command reference

## Usage
`greatness <flags> <subcommand>`

## Base flags
* `-h`, Print help information.
* `--ignore-root-check`, Allow to run as root.
* `-V`, Print version information.

## Subcommands
### Init
Initialises greatness on your computer!
#### Flags
* `--force` Forces to reinitialize, erasing your current configuration, *but not your files*.
```init```

### Doctor
Doctors, and finds issues in your configuration.
```doctor```

### Add
Add a/some file(s) to your configuration.
```add <files>...```

### Rm
Remove a/some file(s) from your configuration.
```rm <files>...```

### Pack
Pack all your dotfiles into your git repository. Doesn't move, only copies.
```pack```

### Prompt
Start a special shell with git configurations set bennificial for greatness.
```prompt```
#### Flags
* `--no-overwrite-ps1`, Don't overwrite the ps1 of your shell.

### Pull
Fetch, merge, or remove an external manifest.
```pull <subcommand>```

#### Subcommands
##### Add
Pull and install an configuration.
```add <from>```
* `--as-main`, Pull and install the configuration **overwritting the main configuration**
* `--only-with-tag`, Only merge files with a specific tag.

##### Rm
Remove a external configuration. Does not remove it if installed as main.
```rm <name>```

### Script
Deal with Rhai scripts.
#### Subcommands
##### Assign
Assigns/adds a script to a file.
```assign <file> <script>```

##### Rm
Removes a script from a file. Doesn't delete the script or the file.
```rm```

##### Jog
Run all scripts associated with all files in the main configuration.
```jog```

### Status
Print the status of your configuration.
```status [file]```

### Tag
Tag(s) (a) file(s).
```tag <tag> <files>...```

