# Hosting your own template

Hosting your own template is something that is really encouraged in wini.

Wini has some subjectives defaults that might not fit everybody. It's normal that you don't agree with all of them.

Therefore, hosting your own template is really straigthforward:

1. Clone the template

```sh
git clone https://codeberg.org/wini/wini-template
```

2. Make your modifications

3. Host it as remote (on codeberg, gitlab, github, etc.)

And... that's pretty much it!

## Use your template

Using your own template is also meant to be really easy. When you do `wini init`, you have the following options:

```
◆ Create a project from  
  Official wini templates
► Remote git repository
  Local git repository
```

You just have to pick "Remote git repository", which will lead to 

```
◆ Remote repository URL:
```

And

```
◆ Which branch should be used ?
```

where you just have to enter informations about your remote git repository. And... that's really it! Congrats!

## Update your template

Updating your template is also very easy, you just have to:

```sh
# 1. Add the original template as remote
git remote add original-template https://codeberg.org/wini/wini-template

# 2. Pull the latest changes
git pull original-template

# 3. Push the changes to your remote
git push origin

# 4. (Optionally) Delete the original template remote
git remote remove original-template
```
