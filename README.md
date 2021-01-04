# Rubsty

[![GitHub Actions Workflow](https://github.com/3wille/rubsty/workflows/Tests/badge.svg)](https://github.com/3wille/rubsty/actions)
[![codecov](https://codecov.io/gh/3wille/rubsty/branch/main/graph/badge.svg?token=25STMW403D)](https://codecov.io/gh/3wille/rubsty)

Commandline tool for consistent language version specifications

## Storytime
You might have come across all those tools managing which versions of some programming language are to be used for your projects.
For ruby, these are at least ``rvm``, ``rbenv`` and ``asdf`` and they all have different files that can set the current language version your project is build for.
This tool is set out to help out with these files and let your contributers choose their own tool.

## Roadmap

- [ ] Ruby:
  - [ ] Files:
    - [x] .ruby-version
        - [ ] catch [rvm gemsets](https://rvm.io/workflow/projects#project-file-ruby-version)
    - [x] .tool-versions
    - [ ] Gemfile & Gemfile.lock
    - [ ] [.rvmrc](https://rvm.io/workflow/projects#project-file-rvmrc)
    - [ ] [.versions.conf](https://rvm.io/workflow/projects#project-file-versionsconf)
  - [ ] 'set all versions' command
- [ ] Other things I guess
