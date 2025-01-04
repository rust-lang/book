document.addEventListener("DOMContentLoaded", function () {
    const htmlElement = document.querySelector("html");
    htmlElement.setAttribute("dir", "rtl");
    htmlElement.style.textAlign = "right";
    
    const bodyElement = document.querySelector("body");
    if (bodyElement) {
        bodyElement.style.direction = "rtl";
        bodyElement.style.textAlign = "right";
    }
});
