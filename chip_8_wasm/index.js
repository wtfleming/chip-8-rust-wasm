const canvas = document.getElementById('chip8-canvas');
const ctx = canvas.getContext('2d');


let isRunning = false;

import("./crate/pkg/index.js").then(wasm => {

  loadRom(wasm, 'PONG2')
  //  loadRom(wasm, 'WIPEOFF')
    .then(() => {
      console.log('ROM finished loading');
      runLoop(wasm);

      wasm.draw_canvas(ctx);
      wasm.update_ui();
    });


  const startButton = document.getElementById("start-button");
  const stopButton = document.getElementById("stop-button");

  startButton.addEventListener('click', () => {
    isRunning = true;
    startButton.disabled = true;
    stopButton.disabled = false;
  });


  stopButton.addEventListener('click', () => {
    isRunning = false;
    startButton.disabled = false;
    stopButton.disabled = true;
  });


  const stepButton = document.getElementById("step-button");
  stepButton.addEventListener('click', () => {
    // button.disabled = true;
    // wasm.draw_single_threaded(context, CANVAS_WIDTH, CANVAS_HEIGHT);
    // button.disabled = false;

    doStep(wasm);
  });



}).catch(console.error);

function doStep(wasm) {
    wasm.emulate_cycle();
    wasm.draw_canvas(ctx);
    wasm.update_ui();
}

function runLoop(wasm) {
  if (isRunning) {
    doStep(wasm);
    // window.requestAnimationFrame(() => {
    //   runLoop(wasm);
    // });
  }

  window.requestAnimationFrame(() => {
    runLoop(wasm);
  });
}


// function emulateCycle(wasm) {
//   wasm.emulate_cycle();
//   wasm.draw_canvas(ctx);
//   wasm.update_ui();

//   window.requestAnimationFrame(() => {
//     emulateCycle(wasm);
//   });
// }


async function loadRom(wasm, name) {
  let i = await fetch(`roms/${name}`);
  let buffer = await i.arrayBuffer();
  const rom = new DataView(buffer, 0, buffer.byteLength);

  // TODO wasm.reset_cpu();   

  wasm.load_game_js(rom);
}




// Alternative implementation without using Web Workers
// import("../pkg/index.js").then(wasm => {
//     const canvas = document.getElementById('raytracer-canvas');
//     canvas.height = CANVAS_HEIGHT;
//     canvas.width = CANVAS_WIDTH;

//     const context = canvas.getContext('2d');

//     const button = document.getElementById("raytracer-button");
//     button.addEventListener('click', () => {
//         button.disabled = true;
//         wasm.draw_single_threaded(context, CANVAS_WIDTH, CANVAS_HEIGHT);
//         button.disabled = false;
//     });

// }).catch(console.error);
