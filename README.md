# *Notice!*
I don't really have the time to maintain Greatness (with school), so I'll be merging these features into another dotfiles manager in the coming months. Until then, I will continue to maintain this project, but after that, it will become defunct. I'll add a link to the dotfile manager I decide to commit these changes to (probably Dotter).

# Greatness!
<p align="center">
  <a href="github.com/IsaccBarker/Greatness" target="blank"><img src="assets/greatness.png" alt="Greatness Logo" /></a>
  <img src="https://forthebadge.com/images/badges/powered-by-electricity.svg">
  <img src="https://forthebadge.com/images/badges/uses-git.svg">
  <img src="https://forthebadge.com/images/badges/does-not-contain-msg.svg">
  <br>
  <img src="https://img.shields.io/badge/PRs-welcome-brightgreen.svg?style=flat-square">
  <img src="https://img.shields.io/github/last-commit/IsaccBarker/Greatness?style=flat-square">
</p>
Achieve it! How you ask? Well, it's pretty simple; just use greatness!

#### Disclaimer
I do not believe that greatness is the best. It fits a medium sized niche, and thus cannot be best. Hence, we use the name greatness, not best.

## Install greatness
### Long way:
```bash
# Only run this if you don't have rust installed.
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install from crates.io
cargo install great

# Or build the latest!
git clone https://github.com/IsaccBarker/greatness.git
cd greatness
cargo install --path .
```
### Short way
```bash
bash -c "$(curl -fsLS https://git.io/JcDJe)"
great pull add -m Wowee/GreatnessIsAmazing # Defaults to github. Put in a full URL if you don't use GitHub!
```

## What is greatness?
Greatness is you being great and better than everyone else. You have to uphold that superiority complex don't you?

## How do I achieve greatness?
Simple! Use this tool. This tool is designed to bring your system up to a working state that you like in a small amount of time. It is flexible, 100% modular, and fast.

You can view the documentation (wiki) [here](https://github.com/IsaccBarker/Greatness/wiki/)!

## Why is Greatness *great*?
Well, just remember the name. Greatness has to be the best. But if you are dim (but still great), here is a comparison chart.
#### Disclaimer
You may recognise this chart from Chezmoi, but with some rows stripped out. This is because I do not know what they mean, and thus cannot implement them. A clock simply means that it will be supported, but isn't yet. If you want them, please file a great issue/pr.

|                                        | greatness         | chezmoi       | dotbot            | rcm               | homesick          | yadm          | bare git   |
| -------------------------------------- | ----------------- | ------------- | ----------------- | ----------------- | ----------------- | ------------- | ---------- |
| Distribution                           | Source/Binary     | Single binary | Python package    | Multiple files    | Ruby gem          | Single script | n/a        |
| Install method                         | Many              | Many          | git submodule     | Many              | Ruby gem          | Many          | Manual     |
| Non-root install on bare system        | ✅                | ✅            | Difficult         | Difficult         | Difficult         | ✅            | ✅         |
| Windows support                        | ❌                | ✅            | ❌                | ❌                | ❌                | ✅            | ✅         |
| Bootstrap requirements                 | Rust, automatic   | Go, automatic | Python, git       | Perl, git         | Ruby, git         | git           | git        |
| Source repos                           | Single            | Single        | Single            | Multiple          | Single            | Single        | Single     |
| dotfiles are...                        | Files             | Files         | Symlinks          | Files             | Symlinks          | Files         | Files      |
| Config file                            | Required, Managed | Optional      | Required          | Optional          | None              | Optional      | Optional   |
| Password manager integration           | ❓                | ✅            | ❌                | ❌                | ❌                | ❌            | ❌         |
| Machine-to-machine file differences    | Scripting         | Templates     | Alternative files | Alternative files | Alternative files | Alternative files, templates | Manual |
| Custom variables in templates          | ✅                | ✅            | ❌                | ❌                | ❌                | ❌            | ❌         |
| Dotfile Merging                        | ✅                | ❌            | ❌                | ❌                | ❌                | ❌            | ❌         |
| Scriptable                             | ✅                | ❌            | ❌                | ❌                | ❌                | ❌            | ❌         |
| Modular                                | ✅                | ❌            | ❌                | ❌                | ❌                | ❌            | ❌         |
| Executable files                       | ✅                | ✅            | ✅                | ✅                | ✅                | ❌            | ✅         |
| Run scripts                            | ✅                | ✅            | ✅                | ✅                | ❌                | ❌            | ❌         |
| Run once scripts                       | ✅                | 🕒            | ❌                | ❌                | ❌                | ❌            | ❌         |
| Software Auto-Install                  | ✅                | ❌            | ❌                | ❌                | ❌                | ❌            | ❌         |
| Machine-to-machine symlink differences | ✅                | ✅            | ❌                | ❌                | ❌                | ✅            | ❌         |
| File Tagging                           | ✅                | ❌            | ❌                | ❌                | ❌                | ❌            | ❌         |
| Shell completion                       | 🕒                | ✅            | ❌                | ❌                | ❌                | ✅            | ✅         |
| Archive import                         | 🕒                | ✅            | ❌                | ❌                | ❌                | ❌            | ❌         |
| Archive export                         | 🕒                | ✅            | ❌                | ❌                | ❌                | ❌            | ✅         |
| Implementation language                | Rust, Lua         | Go            | Python            | Perl              | Ruby              | Bash          | C          |

As you can see, greatness is best. However, a sort of close second [chezmoi], has a weird name, is not scriptable, doesn't support dotfile merging, and isn't modular. This makes it not ideal for situations where you might want to pick parts of different peoples rices, merge them, script program installation (or have greatness do it for you), and then put it out into the world as a repository, which other people can then use as modules.

### But don't use Windows. There is one reasons for this
1. I don't have a Windows machine.
Windows support may or may not be added in the future. The underlying code for supporting Windows is added to the best of my ability, but overall it should not work. Please note that not providing Windows support is mainly for your safety; I would hate for anything bad to happen to your files.

### What is greatness isn't to my taste, but I want to stick with rust?
No worries! Check out [dotter](https://github.com/SuperCuber/dotter) or [toml bombadil](https://github.com/oknozor/toml-bombadil).
