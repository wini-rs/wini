source ./check.nu

def main [] {
    open ./scripts/branches.json | transpose name features | each {|i| 
        print $"# -------------------------------------- #"
        print $"# --- Testing feature \"($i.name)\" --- #"
        print $"# -------------------------------------- #"
        print $"($i.features | append "test")"
        check-with-features ($i.features | append "test")
    }
}
