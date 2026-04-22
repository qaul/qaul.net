# Flutter Dependencies and Tests

This page describes helper scripts and `make` commands to run dependency fetch and tests across the Flutter app and sub-packages.

## Where to run commands

Run commands from `qaul_ui`:

```bash
cd qaul_ui
```

## Make commands

- `make flutter-get-all`: runs `flutter pub get` for `qaul_ui` and all Flutter sub-packages.
- `make flutter-test-all`: runs `flutter test` for `qaul_ui` and all sub-packages that contain a `test` directory.

## Script commands

You can also run scripts directly:

```bash
./bin/fluttergetall.sh
./bin/fluttertestall.sh
```

## CI usage

CircleCI Flutter jobs use these scripts to keep local and CI behavior aligned:

- dependency installation uses `./bin/fluttergetall.sh`
- Flutter test job uses `./bin/fluttertestall.sh`
