#!/bin/bash
set -eu

git config --global pull.rebase true
git config --global core.autocrlf input

#link git hooks
git config core.hooksPath $(readlink -f .githooks)
