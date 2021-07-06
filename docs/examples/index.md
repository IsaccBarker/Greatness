# Script Example
Pretend to be me. No, not lierally. That would be dellusional. But you know what I mean. Anyways, lets say we have the following files:
1. ~/.bashrc
2. ~/.zshrc
3. ~/.config/alacritty/alacritty.yml
4. ~/.config/starship.toml

Granted, I have more dotfiles, such as X config files, but lets not worry about those. Lets go through the steps in order to create greatness!

## Adding
To add your files, just run `greatness add`:
```bash
greatness add ~/.bashrc ~/.zshrc ~/.config/alacritty/alacritty.yml ~/.config/starship.toml
```

## Encryption
Encryption is not implemented yet. I just need to figure out how `rust-crypto` works.... **PRs welcome**!

## Scripting
Lets also assume I'm horrible at writting machine agnostic configuration files, and we need to resolve our home directory before using our .zshrc...  
*~/.greatness/scripts/names.lua*
```lua
function process(data, filename)
    if string.find(data, 'COMMENT: Managed by Greatness!')
        data = 'COMMENT: Managed by Greatness!\n\n' .. data
    end

    data = string.gsub(data, [[{{ astrid:lastname }}]], 'greenwod')
    data = string.gsub(data, [[{{ mack:firstname }}]], 'will')

    return data
end
```
*[old] ~/.astrid.data*
```sql
NAME: astrid {{ astrid:lastname }}
LOCATION: northern canada
SITUATION: lost in the quiet apocalypse
EX-SPOUSE: will mackenzie
```
*[new] ~/.astrid.data*
```sql
COMMENT: Managed by Greatness!

NAME: astrid greenwood
LOCATION: northern canada
SITUATION: lost in the quiet apocalypse
EX-SPOUSE: will mackenzie
```

*run*
```bash
greatness script assign ~/.astrid.data names.lua # Don't provide the full name to the script file
```

Bang! Now run `greatness jog` to rerun the script files, or `greatness pull` will do it for you if you push and repull.

## Packing
In order to push your dotfiles to a remote, you must first pack them:
```bash
greatness pack
```

## Pushing
In order to do any git actions, it is recomended you use
```bash
greatness prompt
```

From there, run git commands as normal. Keep in mind that `greatness pack` automically commits. All you need to do is set up a remote and push! This may be automated in the future.

## Updating
Simply go back into the prompt:
```bash
greatness prompt
```

Then do:
```bash
git pull
```

After that, exit, and run:
```bash
greatness pull -m <your repo> # Here, -m means to replace the local repo with the stuff found in the remote.
```

Bang! All of your dotfiles are on your local machine.

