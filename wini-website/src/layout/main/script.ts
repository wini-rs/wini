import 'htmx.org';


let isNavBarHidden = false;

$("#hide-sidebar").on("click", () => {
    $("#sidebar").css("margin-left", isNavBarHidden ? "0vw" : "-20vw")
    isNavBarHidden = !isNavBarHidden;
})
