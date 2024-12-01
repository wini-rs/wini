import 'htmx.org';

let isNavBarHidden = false;

$("#hide-sidebar").on("click", () => {
    $("#sidebar").css("margin-left", isNavBarHidden ? "0vw" : "calc(-20vw - 16px)")
    isNavBarHidden = !isNavBarHidden;
})


const hlCurrentPage = () => {
    const currentUrl = window.location.pathname;
    const currentPage = currentUrl.split("/")[2] ?? "introduction";

    setHlPage(currentPage);
}

const setHlPage = (s: string) => {
    $("li").forEach(e => s !== e.getAttribute("hx-replace-url")?.split("/")?.at(-1) ? e.rmClass("active") : e.addClass("active"));
}

hlCurrentPage();
