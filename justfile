# list all recipes
default:
  @just --list

# install development build
dev:
  pip install maturin
  maturin develop --extras dev

# install release build
release:
  pip install maturin
  maturin develop --extras dev --profile release

# run code checks
check:
  #!/usr/bin/env bash

  error=0
  trap error=1 ERR

  echo
  (set -x; ruff . )

  echo
  ( set -x; black --check . )

  echo
  ( set -x; mypy . )

  echo
  ( set -x; pytest )

  echo
  ( set -x; make -C docs doctest )

  test $error = 0

# auto-fix code issues
fix:
  black .
  ruff --fix .
