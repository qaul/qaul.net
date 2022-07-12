# Lefthook
[lefthook](https://github.com/evilmartians/lefthook) is a Git hook manager, and it can be used to standardize the project's code.

### Installation
To enable the hooks configured in *lefthook.yml* to be executed in your machine, lefthook needs to be installed globally. This can be accomplished in two ways:
- If you have Node.js installed: `npm install -g @arkweid/lefthook`
- If you’re in a Ruby-based environment: `gem install lefthook`
> *Note: You may need to run these commands as **sudo***

Next, run `lefthook install` on the repo's root folder.
This will install the configured hooks into *.git/hooks* and enable them to run when applicable.

You can see it has worked by receiving a message similar to:
```bash
SYNCING lefthook.yml
SERVED HOOKS: pre-commit, pre-push, prepare-commit-msg
```