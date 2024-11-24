# Wini in CLI

## Create a project
You can create a new project by using:

```sh
wini init
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


## Deploy

You can run your application in production by doing:
```sh
wini run-prod
```


## How to do ... ?

Wini commands are just based on `just`, so you can look at `./justfile` to see what is run behind the hood, and you can customize it as you wish!

Or you can do 
```sh
wini -h
# or
wini help
```
for a quick recap
