import 'htmx.org';

let isNavBarHidden = false;

const switchHidden = () => {
    if (isNavBarHidden) {
        $("#sidebar").rmClass("hidden");
    } else {
        $("#sidebar").addClass("hidden");
    }

    isNavBarHidden = !isNavBarHidden;
}

$("#hide-sidebar").on("click", switchHidden)


const hlCurrentPage = () => {
    const currentUrl = window.location.pathname;
    const currentPage = currentUrl.split("/")[2] ?? "introduction";

    setHlPage(currentPage);

    if (window.innerWidth < 1200) {
        switchHidden();
    }
}

const setHlPage = (s: string) => {
    $("li").forEach(e => s !== e.getAttribute("hx-replace-url")?.split("/")?.at(-1) ? e.rmClass("active") : e.addClass("active"));
}

hlCurrentPage();
