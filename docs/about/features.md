# Features
Greatness has many features unlike the competition. Some of them you may not think are features, but I do. And I'm the project maintainer. So ha!

## Scripting
Greatness uses the [rhai](https://rhai.rs) scripting language. This is because it is secure, fast, easy to implement, and whatever the oposite of bug prone is. Granted, external scripts *do not run by default* because of security concerns. You can obviously enable them however. Rhai even has its own [book](https://rhai.rs/book)! Greatness provides a scripting reference for the stuff it introduces itself.

## Git flexibility
Instead of providing multiple wrappers around various commands (which can restrict if you need something out of the ordinary), greantess instead provides `pack` and `prompt`. Pack to get your commit ready (or you can tell it not to commit, or even add the files for that matter). Then, run `greatness prompt` and do whatever crazy stuff you need!

## Confromant Commands
Almost all of greatness's subcommands come with `add` and `rm` variants. Want to pull an external manifest? *Add*. Want to remove it? *Rm*. This means you hardly ever have the look at the help screen.

## Doctoring
Greatness comes with a `doctor` subcommand. It checks the configuration and makes sure its sane. Then, it'll tell you the diagnosis, and leave it to you to fix it, as greatness cannot yet fix its own issues (pr/issue anyone?)

## More!
Look aroud the help screen, and I'm sure you'll find something you like.

