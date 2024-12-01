ask() {
    echo -en "\e[34m[\e[35m?\e[34m]\e[0m $1"
}

info() {
    echo -e "\e[34m[\e[32m*\e[34m]\e[0m $1"
}

warn() {
    echo -e "\e[34m[\e[33mW\e[34m]\e[0m $1"
}

error() {
    echo -e "\e[34m[\e[31mE\e[34m]\e[0m $1"
}
