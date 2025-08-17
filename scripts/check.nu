# Returns a string
def check-with-features [features: list] {
    let tmp_dir = mktemp -d
    cp -r wini $tmp_dir
    cd $'($tmp_dir)/wini'

    ls  **/*
    | where name !~ 'target' and type == 'file' and name !~ '\.lockb|\.ico$'
    | each {|file|
        open $file.name --raw
        | lines
        | reduce --fold {content: "", should_delete: false, depth: 0} {|line, acc|
            if $line =~ '^// IFFEAT' or $line =~ '^# IFFEAT' {
                if $acc.should_delete {
                    let return_acc = ($acc | update depth ($acc.depth + 1))
                    return $return_acc
                }

                let feat = ($line | split row ' ' | last)

                if $feat not-in $features {
                    let return_acc = ($acc | update should_delete true)
                    return $return_acc
                }

                return $acc
            }

            if $line =~ '^// ENDIF' or $line =~ '^# ENDIF' {
                if $acc.should_delete {
                    if $acc.depth == 0 {
                        let return_acc = ($acc | update should_delete false)
                        return $return_acc
                    } else {
                        let return_acc = ($acc | update depth ($acc.depth - 1))
                        return $return_acc
                    }
                }

                return $acc
            }

            if $acc.should_delete {
                return $acc
            }

            let new_str = ($acc.content + '
' + $line)
            let return_acc = ($acc | update content $new_str)
            return $return_acc
        }
        | get content
        | str trim
        | $in + "\n"
        | save -f $file.name
    }

    nix develop -c nu -c "just compile-scss; cargo clippy -- --deny warnings --allow non-snake-case; cargo test"

    # Free space after computation
    rm -rf $'($tmp_dir)/wini/target'

    $'($tmp_dir)/wini'
}
