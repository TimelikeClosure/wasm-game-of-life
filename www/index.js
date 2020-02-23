import * as wasm from "wasm-game-of-life";

window.addEventListener("load", () => {
    const name = window.prompt("What is your name?", "Unnamed Citizen");

    wasm.greet(name);
});
