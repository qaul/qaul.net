# CircleCI Config
> *The list of available tasks performed by CircleCI can be found [here](qaul/tooling/circleci.md)*

The `./circleci_config` folder follows the [FYAML](https://github.com/CircleCI-Public/fyaml) specs
to leverage circleci's `pack` CLI command.

We also make use of dynamic configuration, which allows for
a finer control over CircleCI's pipelines.

To learn more about it, check:
> * [CircleCI CLI Docs on `pack`](https://circleci.com/docs/2.0/local-cli/#packing-a-config)
> * [FYAML Specification](https://github.com/CircleCI-Public/fyaml/blob/master/fyaml-specification.md)
> * [Dynamic Configurations on CircleCI](https://circleci.com/docs/2.0/dynamic-config/)

## Naming conventions for jobs and workflows
Jobs and workflows follow the pattern:

`<ACTION>-<PROJECT>-<PLATFORM - if applicable>`

I.E. a libqaul build job for the linux platform should be found in *./jobs/build-libqaul-linux.yml*.

## Regenerating the config.yml file
After modifying part of The `./circleci_config` folder structure, run the following command to regenerate the configuration files:

```shell
# Ensure that you're in the root dir of this project
cd qaul.net

# CircleCI CLI must be installed
circleci version

# Run the script found in .scripts/pack.sh
sh circleci_config/scripts/pack.sh
```

## Folder structure and packing logic
There are two generated configuration files: `config.yml` and `continue-config.yml`.

The former defines CircleCI's entrypoint when a pipeline is triggered, which in turn points
CircleCI to run the proper workflow based on what started the pipeline (defined in the second file).

The configuration for `config.yml` is found in `./config-setup`, whilst the latter is in `./config-continuation`.

As per FYAML specs, any yml file starting with an "@" will be placed as is on the same level as its
parent.

A folder or a file describes a new key, and any contents within it will be nested in this new key.

### Example
To add a new workflow named 'example-workflow' to the `continue-config.yml` file, add a `example-workflow.yml`
in `./config-continuation/workflows`. The resulting tree would look similar to:
```
.
└── config-continuation
          └── workflows
              └── example-workflow.yml
```

Assuming the contents of `example-workflow.yml` are:

```yml
steps:
  - run: echo "testing"
```

The resulting packed configuration file would look similar to:

```yml
# continue-config.yml
workflows:
  example-workflow:
    steps:
      - run: echo "testing"
```