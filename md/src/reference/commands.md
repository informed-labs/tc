# Command Reference

<!-- toc -->

`Contextual` commands need to be run from the functor or service directory. The directory structure holds the required state. `Workflow` commands can be run anywhere and neither require the techno-core repo nor any local state.

Below is a table showing equivalent workflow and contextual commands. Contextual commands are typically used during dev and in CI.

| Workflow | Contextual |
|----------|------------|
|          | build      |
| manifest | compose    |
| deploy   | create     |
| release  | tag        |
| diff     | changelog  |
| invoke   | test       |
| stash    |            |
| route    |            |


## Contextual Commands

### compose

`compose` generates a self-contained description of the topology.


```
Options:
  -d, --dir <DIR>
  -e, --env <ENV>
  -s, --sandbox <SANDBOX>
  -c, --component <COMPONENT>
```

```sh
cd services/extraction/dealer
tc compose -s test -e dev-af
```

### build

`tc` optimizes the build process to decrease deploy time by reducing the aritifact size. For example, deploying the entire `extraction` topology takes less than 10s on a decent network.

1. Dependencies (in Gemfile, Pip or Poetry) are built as layers  (optional)
2. The code is `packed` and thus the deploys are significantly faster

```sh
cd services/extraction
tc build
```

The above command zips all the functions and renders the stepfn state-machine, events and routes (if any).

To build layers manually:

```sh
cd services/extraction/vision_page_process
tc build --layers
tc update -c layers
```
`tc deploy -c layers` creates a layer with the same name as the function name. So make sure to add the layer in `function.json -> runtime -> layers` section


### create

Create a sandboxed topology or functor

```
Options:
  -d, --dir <DIR>
  -e, --env <ENV>
  -s, --sandbox <SANDBOX>
  -m, --mode <express|standard>
      --notify
  -h, --help

```

```
tc create -e dev --sandbox test
2023-08-01T21:21:53.820 Initializing env dev for test
2023-08-01T21:21:55.438 Creating logf (924 B)
2023-08-01T21:21:55.844 Creating aggregator log-group /aws/states/my_logs
2023-08-01T21:21:57.212 Enabling stepfn logging
2023-08-01T21:21:58.623 Adding permission to filter
2023-08-01T21:21:59.985 Creating subscription filter test { $.type = "TaskStateEntered"  ||  $.type = "TaskFailed" }
```

### update

```
Options:
  -d, --dir <DIR>
  -e, --env <ENV>
  -s, --sandbox <SANDBOX>
  -c, --component <COMPONENT>
  -l, --link <LINK>
  -a, --asset <ASSET>
```

### delete
To delete a sandbox:

```
cd service/my-service
tc delete --sandbox test --env dev
```
```
