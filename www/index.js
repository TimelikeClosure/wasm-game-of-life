import { Universe } from "wasm-game-of-life";

window.addEventListener("load", () => {
    const universe = Universe.new();
    const renderArea = document.getElementById("game-root");
    defineRenderLoop(universe, renderArea)();
});

const defineRenderLoop = (source, target) => {
    const renderLoop = () => {
        target.textContent = source.render();
        source.tick();
        requestAnimationFrame(renderLoop);
    }
    return renderLoop;
}
