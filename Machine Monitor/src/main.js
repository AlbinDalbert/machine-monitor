const { invoke } = window.__TAURI__.tauri;


let tempEl;
let cpuUtilEl;
let memoryEl;

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  greetMsgEl.textContent = await invoke("greet", { name: greetInputEl.value });
}

async function update() {
  tempEl.textContent = await invoke("get_temp");
  cpuUtilEl.textContent = await invoke("get_cpu_util");
  memoryEl.textContent = await invoke("get_memory");
}

window.addEventListener("DOMContentLoaded", async () => {
  tempEl = document.querySelector("#temp-text");
  cpuUtilEl = document.querySelector("#cpu-text");
  memoryEl = document.querySelector("#memory-text");
  document
    .querySelector("#update-button")
    .addEventListener("click", () => update());
});
