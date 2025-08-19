source ./check.nu


def main [...features: string] {
    let dir = check-with-features $features

    print "# --- You can now --- #"
    print $"cd ($dir)"
}
