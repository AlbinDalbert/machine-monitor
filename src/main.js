const { invoke } = window.__TAURI__.tauri;
const { listen } = window.__TAURI__.event;


let tempEl;
let cpuUtilEl;
let memoryEl;
let gpuTempEl;
let gpuUtilEl;
let vramEl;
let total_memory = 0.0;

window.addEventListener("DOMContentLoaded", async () => {
  tempEl = document.querySelector("#temp-text");
  cpuUtilEl = document.querySelector("#cpu-text");
  memoryEl = document.querySelector("#memory-text");
  vramEl = document.querySelector("#vram-text");
  gpuTempEl = document.querySelector('#gpu-temp-text');
  gpuUtilEl = document.querySelector('#gpu-util-text');
  document
    .querySelector("#update-button")
    .addEventListener("click", () => update());
});


await listen('temp', (event) => {
  console.log("js: measure: " + event.payload);
  tempEl.textContent = event.payload;
  if (parseInt(event.payload.substring(0, event.payload.length)) > 95) {
    tempEl.style.color = "red";
  } else {
    tempEl.style.color = "rebeccapurple";
  }
})

await listen('memory', (event) => {
  console.log("js: measure: " + event.payload);
  memoryEl.textContent = event.payload;
})

await listen('cpu_util', (event) => {
  console.log("js: measure: " + event.payload);
  cpuUtilEl.textContent = event.payload;
})

await listen('vram', (event) => {
  console.log("js: measure: " + event.payload);
  vramEl.textContent = event.payload;
})

await listen('gpu_util', (event) => {
  console.log("js: measure: " + event.payload);
  gpuUtilEl.textContent = event.payload;
})

await listen('gpu_temp', (event) => {
  console.log("js: measure: " + event.payload);
  gpuTempEl.textContent = event.payload;
})