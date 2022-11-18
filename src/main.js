const { invoke } = window.__TAURI__.tauri;
const { listen } = window.__TAURI__.event;


let tempEl;
let cpuUtilEl;
let memoryEl;
let total_memory = 0.0;

window.addEventListener("DOMContentLoaded", async () => {
  tempEl = document.querySelector("#temp-text");
  cpuUtilEl = document.querySelector("#cpu-text");
  memoryEl = document.querySelector("#memory-text");
  document
    .querySelector("#update-button")
    .addEventListener("click", () => update());
});


await listen('temp', (event) => {
  console.log("js: measure: " + event.payload);
  tempEl.textContent = event.payload;
})

await listen('memory', (event) => {
  console.log("js: measure: " + event.payload);
  memoryEl.textContent = event.payload;
})

await listen('cpu_util', (event) => {
  console.log("js: measure: " + event.payload);
  cpuUtilEl.textContent = event.payload;
})