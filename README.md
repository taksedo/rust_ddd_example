# rust_ddd_example

Reference project with Rust and DDD

[![codecov](https://codecov.io/gh/taksedo/rust_ddd_example_shop/branch/master/graph/badge.svg?token=BFGRXLQFI3)](https://codecov.io/gh/taksedo/rust_ddd_example_shop)

[![image](https://codecov.io/gh/taksedo/rust_ddd_example_shop/branch/master/graphs/sunburst.svg?token=BFGRXLQFI3)](https://codecov.io/gh/taksedo/rust_ddd_example_shop)

```
build_and_run_local_image.sh
```
# Introduction

This project is inspired by https://github.com/stringconcat/ddd_practice and the book [Clean Architecture](https://www.amazon.com/Clean-Architecture-Craftsmans-Software-Structure/dp/0134494164)

# Start dev environment

In current project dev containers are implemented, so you can start project with **VSCode Dev Containers** or **RustRover Dev containers**.

Dev container includes following containers:
+ dev environment
+ kafka
+ kafka-ui
+ db

Before starting the App db migration is needed. To manage this please start the following script

```shell
tools/scripts/migration_run.sh
```