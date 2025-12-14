# Plan

## Features

* Fetch PKGBUILDs from the AUR.
* Customizing AUR repo URLs with `lazy-template`.
* Build pacman repositories from within Linux containers.
* The manifest should be in the HJSON format.

## General method

This tool should build each package in a separate container.

Some container images may inherit from other container images based on dependencies graph.

There may be container images whose purpose is not to build, but to install common dependencies between two or more packages.

## Tasks before completion

* [ ] Clean up `check-all.bash`.
* [ ] Re-examine `Cargo.toml`.
* [ ] Create `README.md`.
* [ ] Update GitHub descriptions.
* [ ] Update GitHub URLs.
* [ ] Backup the old `pacman-repo-builder/pacman-repo-builder` into somewhere else.
* [ ] Replace `pacman-repo-builder/pacman-repo-builder` (official) with this code.
* [ ] Publish the crate, and the binary.
* [ ] Update the GitHub Action.
* [ ] Delete this file.
