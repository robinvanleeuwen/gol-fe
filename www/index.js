import { Universe } from "game-of-life";

const pre = document.getElementById("game-of-life-canvas");
const universe = Universe.new(17,5);
console.log(pre);
const renderLoop = () => {
    pre.textContent = universe.render();
    universe.tick();
    setTimeout(functionAfterSleep, 200);

};

function functionAfterSleep() {
    requestAnimationFrame(renderLoop);
}

requestAnimationFrame(renderLoop);