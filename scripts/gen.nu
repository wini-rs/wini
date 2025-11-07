def main [] {
    print "log"
    chmod 755 wini
    print "log"
    chmod 755 wini-sdk

    print "log"
    cp -r wini-base/client/* wini
    cp -r wini-base/shared/* wini
    print "log"
    cp -r wini-base/sdk/* wini-sdk
    cp -r wini-base/shared/* wini-sdk
    print "log"

    # Client
    cd wini
    remove-gating 'client'
    cargo fmt; cargo clippy; cargo test

    # SDK
    cd ../wini-sdk
    remove-gating 'sdk'
    cargo fmt; cargo clippy; cargo test

    print "log"

    cd ..
    chmod 444 wini
    chmod 444 wini-sdk
}

def remove-gating [current_target: string] {
    ls  **/*
    | where name !~ 'target' and type == 'file' and name !~ '\.lockb|\.ico$'
    | each {|file|
        open $file.name --raw
        | lines
        | reduce --fold {content: "", should_delete: false, depth: 0} {|line, acc|
            if $line =~ '^//// IFTARGET' or $line =~ '^## IFTARGET' {
                if $acc.should_delete {
                    let return_acc = ($acc | update depth ($acc.depth + 1))
                    return $return_acc
                }

                let target = ($line | split row ' ' | last)

                if  $target != $current_target {
                    let return_acc = ($acc | update should_delete true)
                    return $return_acc
                }

                return $acc
            }

            if $line =~ '^//// ENDIF' or $line =~ '^## ENDIF' {
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

}
