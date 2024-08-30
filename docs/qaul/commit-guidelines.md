# Commit Guidelines

This is best praxis guide for committing to the qaul project.
We value every contributions!

## Pull Request & Commit Structure

* Please organize the commits by functionality.
* Please structure your work in feature branches.
* Make separate commits for comment typo & coding style fixes, if they are not directly in the functions you're working on.

### Before Pull Request

Please always rebase

#### How to Rebase your Branch against Main Branch

Rebasing puts your commits on top of the main branch.

```sh
# checkout main branch and pull latests changes to make
# sure it is up to date.
checkout main
git pull

# checkout your_branch and rebase it against main branch
checkout your_branch
git rebase main

# now you can push your_branch to github and create
# a pull request.
```

You can use rebasing also during development to bring the latest
changes from main branch to your_branch.

## Coding Style

The coding style is following each languages linting guide lines.

## Comments

Please write a short explanation on it's functionality in each file.
Please write a short comment for every function that describes what this function is doing.

For automated code documentation generation, use triple slashes `///` line comments for function description & file descriptions.

## Licensing

Each code file needs to start with the qaul license:

```rs
// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.
```
