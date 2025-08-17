rm scripts/*.sh

let new_justfile = head -n -3 justfile
$new_justfile | save -f justfile

rm scripts/on-install.nu
