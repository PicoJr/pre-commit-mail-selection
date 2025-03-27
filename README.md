# Pre Commit Email Selection Git Hook

It looks like this (YMMV) if you configure this as your pre-commit hook:

```
? Please select an email for commit ›
❯ foo@bar.com
  baz@example.com
```

It will ask **each time** which email you would like to configure before committing.

Your selection will be written to your local `.git/config` and used for the current commit.

## But why would anyone use/need this ?

I admit it's really niche, just imagine you happened to share a local git repository with someone,  
you both work on the same files, with the same user, on an airtight (no internet) system,  
and you wish to be able to distinguish commit authors.

Sounds awful ? It should. 

It could also be useful if you need to contribute with several identities on a regular basis,  
from the same local git repository.

## How does it work ?

It lists emails from the following sources:

1. local `.git/config`
2. global git config (several possible paths)
3. `.git-emails.toml`

It then presents a selection menu for choosing the email you wish to use for committing.

Your selection will be written to your local `.git/config` and used for the current commit.

Since the default selection is the email from `.git/config`, it will remember your last choice:  
the last configured email will be the first one in the selection list the next time it runs.

## How to build it so that it runs with minimal dependencies (statically)

Build using musl

```
cargo build --target=x86_64-unknown-linux-musl
```

and then

```
❯ ldd target/x86_64-unknown-linux-musl/debug/pre-commit-mail-selection
        statically linked
```

## How to configure it as my `pre-commit` hook ?

1. compile it: `cargo build --target=x86_64-unknown-linux-musl --release`
2. install it `cp target/x86_64-unknown-linux-musl/release/pre-commit-mail-selection .git/hooks/pre-commit`

## How do I supply my own emails ?

create `.git-emails.toml` file inside your git repository:

see [.git-emails.toml](./.git-emails.toml) for an example.

## Known issue

Does not work with IDE that do not run pre-commit hooks in a terminal.