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
  # NOTE: Running "make -C docs text" to deal with https://github.com/sphinx-doc/sphinx/issues/11681
  ( set -x; make -C docs text; make -C docs doctest )

  echo
  ( set -x; cargo check )

  echo
  ( set -x; cargo clippy )

  test $error = 0

# auto-fix code issues
fix:
  black .
  ruff --fix .

# make docs
docs:
  rm -rf docs/source/_autosummary
  rm -rf docs/build
  make -C docs html
  @echo Docs are in: $PWD/docs/build/html/index.html
