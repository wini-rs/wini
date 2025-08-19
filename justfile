set shell := ["nu", "-c"]


# Check that all the branches are OK
check-all:
    nu ./scripts/check-all.nu


init-branches:
    # nu ./scripts/init-branch.nu
    echo "Deprecated"

# On wini-template, make default == main
sync-default-main:
    nu ./scripts/sync-default-main.nu

# Sync codeberg wini-template with the state of github wini-template
sync-codeberg-with-github:
    nu ./scripts/sync-codeberg-with-github.nu

see-branch *ARGS:
    nu ./scripts/see-branch.nu ...[{{ARGS}}]
