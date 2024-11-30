import 'htmx.org';


let isNavBarHidden = false;

$("#hide-sidebar").on("click", () => {
    $("#sidebar").css("margin-left", isNavBarHidden ? "0vw" : "calc(-20vw - 16px)")
    isNavBarHidden = !isNavBarHidden;
})
