# Rustydo

This is a terminal based TODO list made with Rust, using the [ratatui](https://ratatui.rs/) crate.

## Developing process

Just making the UI to render a basic view of a list of tasks.

Lately, I'm planing to make a database saving all the information. I'm thinking about Redis, but I don't know yet.

## Prerequisites

You just need [Docker](https://www.docker.com/) installed on your local machine to run this application.

## Try it

Once you have Docker installed, run the following command in the root directory of the project:
```shell
make
```
Yes, as simple as that, and Docker will do everything for you.

Check out the `Makefile` and the `Dockerfile` to see what is going on during the building process of the application.
