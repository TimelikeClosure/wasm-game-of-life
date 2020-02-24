import { Universe } from "wasm-game-of-life";
import { memory } from "wasm-game-of-life/wasm_game_of_life_bg";

const CELL_SIZE = 10;
const GRID_COLOR = "#CCCCCC";
const DEAD_COLOR = "#FFFFFF";
const ALIVE_COLOR = "#000000";

// document.addEventListener("DOMContentLoaded", () => {
//     document
//     .getElementById("game-start")
//     .addEventListener('click', gameStart);
// });

// const gameStart = event => {
//     const renderArea = document.getElementById("game-root");
//     const universe = Universe.new();
//     const renderLoop = defineRenderLoop(universe, renderArea);
//     event.target.remove();
//     renderLoop();
// }

const getIndex = (row, column) => row * width + column;

const getGroupIndex = index => Math.floor(index / 8);

const getCellIndex = index => index % 8;

const defineRenderLoop = (source, ctx) => {
    const drawGrid = () => {
        // Initialize pen
        ctx.beginPath();
        ctx.strokeStyle = GRID_COLOR;

        // Draw rows
        {
            const xMax = width * (CELL_SIZE + 1);
            for (let line = 0; line <= height; line++){
                const y = line * (CELL_SIZE + 1);
                ctx.moveTo(0, y);
                ctx.lineTo(xMax, y);
            }
        }
        // Draw columns
        {
            const yMax = height * (CELL_SIZE + 1);
            for (let line = 0; line <= height; line++){
                const x = line * (CELL_SIZE + 1);
                ctx.moveTo(x, 0);
                ctx.lineTo(x, yMax);
            }
        }
        // Finalize draw
        ctx.stroke();
    };

    const drawCells = () => {
        // Initialize source
        const cellsPtr = source.cells();
        const cells = new Uint8Array(memory.buffer, cellsPtr, width * height);

        const prevCellsPtr = source.prev();
        const prevCells = new Uint8Array(memory.buffer, prevCellsPtr, width * height);

        // Initialize pen
        ctx.beginPath();
        ctx.moveTo(1, 1);

        // Draw squares
        for (let row = 0; row < height; row++){
            const y = row * (CELL_SIZE + 1) + 1;
            for (let column = 0; column < width; column++){
                const index = getIndex(row, column);
                const cell = (cells[getGroupIndex(index)] >> getCellIndex(index)) & 1;
                const prevCell = (prevCells[getGroupIndex(index)] >> getCellIndex(index)) & 1;
                if (prevCell !== cell){
                    ctx.fillStyle = cell ? ALIVE_COLOR : DEAD_COLOR;
                    const x = column * (CELL_SIZE + 1) + 1;
                    ctx.fillRect(x, y, CELL_SIZE, CELL_SIZE)
                }
            }
        }

        // Finalize draw
        ctx.stroke();
    };

    const renderLoop = () => {
        drawGrid();
        drawCells();

        source.tick();

        requestAnimationFrame(renderLoop);
    }
    return renderLoop;
}

// Create the Universe. Should only take a moment
const universe = Universe.new();
const width = universe.width();
const height = universe.height();

// Size the canvas
const canvas = document.getElementById("game-canvas");
canvas.height = (CELL_SIZE + 1) * height + 1;
canvas.width = (CELL_SIZE + 1) * width + 1;

const ctx = canvas.getContext('2d');

const renderLoop = defineRenderLoop(universe, ctx);
renderLoop();
