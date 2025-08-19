source ./check.nu


def main [...features: string] {
    let dir = check-with-features $features

    print "\n\n\n\e[1;34m# --- You can now --- #\e[0m"
    print $"cd ($dir)"
}
