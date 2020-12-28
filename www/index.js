import { Universe } from "game-of-life";

const pre = document.getElementById("game-of-life-canvas");

const digest_history_retention = 150;
const width = 100;
const height = 32;
const mod_1 = 5;
const mod_2 = 11;
const pause_in_ms = 25;

const universe = Universe.new(width, height, mod_1, mod_2, digest_history_retention);
console.log(pre);
const renderLoop = () => {
    pre.textContent = universe.render();
    if (universe.tick() == "stop") {
        return;
    }
    setTimeout(functionAfterSleep, pause_in_ms);

};

function functionAfterSleep() {
    requestAnimationFrame(renderLoop);
}

requestAnimationFrame(renderLoop);