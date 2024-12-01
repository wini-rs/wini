# sqlx_test

*The following commands are compatible with `just`, so you can also use `just` instead of `wini`*

## Setup

To test this, you need to first use:

```sh
wini start-docker
```
and
```sh
wini migrate
```

## Start

To enter the development environment: 
```sh
wini env # This requires Nix
```

To run the project:
```sh
wini run # By default, starts on port 3000
```

After that, you can see the project by looking at `localhost:3000`, from your browser or in CLI (`curl localhost:3000`) !
