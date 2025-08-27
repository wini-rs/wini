# Replace the extension by another one
def replace-ext [from, to] {
    str replace $'\.($from)$' $'.($to)'
}

# Create a colored Yes/No prompt askind for input
def prompt_yesno [prompt, default_opt] {
    print -n $'($prompt) (
        if $default_opt == 'y' {
            "[\e[32;1mY\e[33m/\e[91mn\e[0m]"
        } else {
            "[\e[92my\e[33m/\e[31;1mN\e[0m]"
        }
    ) '

    let yes_or_no = input

    return (($yes_or_no | str downcase) == (if $default_opt == 'y' {'n'} else {'y'}));
}

# Logging
def ask [str] {
    print $"\e[34m[\e[35m?\e[34m]\e[0m ($str)"
}

def info [str] {
    print $"\e[34m[\e[32m*\e[34m]\e[0m ($str)"
}

def warn [str] {
    print $"\e[34m[\e[33mW\e[34m]\e[0m ($str)"
}

def error [str] {
    print $"\e[34m[\e[31mE\e[34m]\e[0m ($str)"
}

# Useful to do `expr | pipeline | neg` instead of `not (expr | pipeline)`
def neg [] {
    not $in
}
