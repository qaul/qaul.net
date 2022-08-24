# CircleCI
> *The configuration docs can be found [here](qaul/tooling/circleci-configuration.md)*

Here's a breakdown of each task that can be currently performed by CircleCI, and how to trigger such a task:

1. [Testing/analyzing Flutter codebase](#analyze-and-test-flutter)
1. [Testing Rust compilation](#build-rust)
2. [Building libqaul binaries](#build-libqaul-binaries)
3. [Building and Releasing a new Mobile Version](#release-flutter-version)

### analyze-and-test-flutter
This workflow is triggered on each commit pushed to the repo that has changed the contents of `./qaul_ui`. It runs
`flutter test` and `flutter analyze` in the `./qaul_ui` folder.

### build-rust
This workflow is triggered on each commit pushed to the repo that has changed the contents of `./rust`.
It tests that libqaul successfully compiles to the Linux platform.

### Build libqaul binaries
This task builds the library binaries to all platforms and creates a new Github release, where you can download them.

To trigger such a task, one must create an annotated tag starting with "v" followed by the new [SemVer](https://semver.org) and ending with the "-rust" suffix.

!> You should use the version described in `rust/libqaul/Cargo.toml` as the release version, and update it accordingly. Any diversion between the git tag's version and that of Cargo.toml **will fail** the pipeline.

#### Platform-specific release pipelines
Optionally, you may add another suffix, describing the platform you'd like to build and release for.
This is specially useful when a pipeline fails for a single platform. In that case, you may push
a hotfix for the desired platform and tag a platform-specific release.

The options are: `.*-android`; `.*-ios`; `.*-linux`; `.*-macos`; `.*-windows`.

```bash
# Creates annotated tag - **ALL Platforms**
git tag -a v<SemVer>-rust -m '<brief message>'

# [Optional] Platform-specific tag
git tag -a v<SemVer>-rust-android -m '<message>'

# Pushes both commits and annotated tags (available since Git 1.8.3)
git push --follow-tags
```

### Release Flutter Version
This task will create a beta release on the Play Store & Testflight, as well as generate the libqaul mobile binaries.
In addition, desktop installers and the mobile apps will be added to a new Github Release.

To trigger this workflow, one must create an annotated tag starting with "v" followed by the new [SemVer](https://semver.org) and ending with the "-flutter" suffix.

!> You should use the version described in `qaul_ui/pubspec.yaml` as the release version, and update it accordingly. Any diversion between the git tag's version and that of pubspec.yaml **will fail** the pipeline.

```bash
# Creates annotated tag
git tag -a v<SemVer>-flutter -m '<brief message>'

# Pushes both commits and annotated tags (available since Git 1.8.3)
git push --follow-tags
```
