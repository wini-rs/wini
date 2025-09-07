rm scripts/*.sh

let new_justfile = head -n -3 justfile
$new_justfile | save -f justfile

# IFFEAT ssr
rm src/shared/wini/ssg.rs
# ENDIF
rm scripts/on-install.nu
