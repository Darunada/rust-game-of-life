import { Universe } from "wasm-game-of-life";

const stage = document.getElementById("game-of-life-canvas");
const universe = Universe.new();

const renderLoop = () => {
    stage.textContent = universe.render();
    setTimeout(() => {
        // console.log(stage.textContent);
        universe.tick();
        requestAnimationFrame(renderLoop);
    }, 100);


}

requestAnimationFrame(renderLoop);