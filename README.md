# YumNote

## Table of Contents

- [About](#about)
- [Getting Started](#getting_started)
- [Usage](#usage)
- [Contributing](../CONTRIBUTING.md)

## About 

YumNote is currently a CLI based tool to create/monitor notes on the fly. I needed something better for my work flow, so I decided to make my own solution.

## Getting Started 

These instructions will get you a copy of the project up and running on your local machine for development and testing purposes. See [deployment](#deployment) for notes on how to deploy the project on a live system.

### Prerequisites

What things you need to install the software and how to install them.

```sh
[Rust Toolchain](https://www.rust-lang.org/learn/get-started)
[Linux Based OS](https://www.linux.org/pages/download/)
```

### Installing

A step by step series of examples that tell you how to get a development env running.

Say what the step will be

```sh
cargo install yum_note
-- I recommend setting an alias. I use `alias ynote="yum_note"`
```

## Usage 

```sh
enum NoteEnum {
    Todo,
    Working,
    Done,
    Deleted,
}
```

ynote -a (Creates a new note) requires: Name (Name of the file) | Content (The Note Itself) | Status (NoteEnum Value)

ynote -e (Edits a note) requires: Name (Name of File)

ynote -d (Deletes a Note) requires: Name (Name of File) 

** This repo is a work in progress
