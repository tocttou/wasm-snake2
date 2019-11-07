import {Board, Cell, Direction, get_memory as getMemory} from "../pkg";

const CELL_SIZE = 25;
const GRID_COLOR = "#585858";
const DEAD_COLOR = "#FFFFFF";
const LIT_COLOR = "#000000";

const width = 32;
const height = 32;
const board = new Board(width, height);
const memory = getMemory();

const canvas = document.getElementById('snake-canvas');
canvas.height = (CELL_SIZE + 1) * height + 1;
canvas.width = (CELL_SIZE + 1) * width + 1;

const scoreBoard = document.getElementById('scoreboard');

const ctx = canvas.getContext('2d');

const keyDirectionMap = {
    ArrowUp: Direction.Up,
    ArrowRight: Direction.Right,
    ArrowDown: Direction.Down,
    ArrowLeft: Direction.Left,
};

let renderLoopId;

document.addEventListener('keyup', e => {
    const newDirection = keyDirectionMap[e.key];
    const direction = board.direction();
    if (!Number.isInteger(newDirection) || newDirection === (direction + 2) % 4) {
        return;
    }
    clearTimeout(renderLoopId);
    board.change_direction(newDirection);
    renderLoop();
});

const renderLoop = () => {
    board.tick();
    drawCells();
    scoreBoard.textContent = `Score: ${board.score()}`;
    renderLoopId = setTimeout(renderLoop, 200);
};


const drawGrid = () => {
    ctx.beginPath();
    ctx.strokeStyle = GRID_COLOR;

    for (let i = 0; i <= width; i++) {
        ctx.moveTo(i * (CELL_SIZE + 1) + 1, 0);
        ctx.lineTo(i * (CELL_SIZE + 1) + 1, (CELL_SIZE + 1) * height + 1);
    }

    for (let j = 0; j <= height; j++) {
        ctx.moveTo(0, j * (CELL_SIZE + 1) + 1);
        ctx.lineTo((CELL_SIZE + 1) * width + 1, j * (CELL_SIZE + 1) + 1);
    }

    ctx.stroke();
};

const getIndex = (row, column) => {
    return row * width + column;
};

const drawCells = () => {
    const cellsPtr = board.cells();
    const cells = new Uint8Array(memory.buffer, cellsPtr, width * height);

    ctx.beginPath();
    for (let row = 0; row < height; row++) {
        for (let col = 0; col < width; col++) {
            const idx = getIndex(row, col);

            ctx.fillStyle = cells[idx] === Cell.Dead ? DEAD_COLOR : LIT_COLOR;

            ctx.fillRect(
                col * (CELL_SIZE + 1) + 1,
                row * (CELL_SIZE + 1) + 1,
                CELL_SIZE,
                CELL_SIZE
            )
        }
    }
    ctx.stroke();
};

drawGrid();
requestAnimationFrame(renderLoop);


