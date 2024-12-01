# Util script to create a colored Yes/No

yesno() {
    case "$1" in
        "y")
            echo -en "[\e[32;1mY\e[33m/\e[91mn\e[0m]"
            ;;
        "n")
            echo -en "[\e[92my\e[33m/\e[31;1mN\e[0m]"
            ;;
        *)
            echo -en "[\e[92my\e[33m/\e[91mn\e[0m]"
            ;;
    esac
}
