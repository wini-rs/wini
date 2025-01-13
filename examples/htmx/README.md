# htmx

A basic example that demonstrates how to use `wini` + `htmx`. If you want to see a more complete example, I encourage you to look for [wini-website](../../wini-website) which uses `htmx` for the page transitioning!

*The following commands are compatible with `just`, so you can also use `just` instead of `wini`*

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

## htmx

To add `htmx`, you can just do 

```sh
wini js-add htmx.org
```
