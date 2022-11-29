import { invoke } from "@tauri-apps/api/tauri";

//index.html
let classNameInputEl: HTMLInputElement | null;
let testMsgEl: HTMLElement | null;

async function addClass() {
  if (testMsgEl && classNameInputEl) {
    if (classNameInputEl.value == "") {
      console.log("Plz enter a class")
    } else {
      testMsgEl.textContent = await invoke("greet", {
        name: classNameInputEl.value,
      });
      change_page_to("addClass.html")
    }

  }
}

//addClass.html
let numOfBlitzInputEl: HTMLInputElement | null;
let roundsPerBlitzInputEl: HTMLInputElement | null;
async function saveClass() {
  if (numOfBlitzInputEl && roundsPerBlitzInputEl) {
    //TODO: ADD INPUT TYPE CHECK LATER
    if (numOfBlitzInputEl.value == "") {
      console.log("Plz enter num of blitzes")
    } else {
      change_page_to("index.html")
    }
  }
}

async function change_page_to(page: string) {
  window.location.href = page
}

//index.html
window.addEventListener("DOMContentLoaded", () => {
  classNameInputEl = document.querySelector("#className-input");
  testMsgEl = document.querySelector("#test-msg");
  document
    .querySelector("#addClass-button")
    ?.addEventListener("click", () => addClass());
});

//addClass.html
window.addEventListener("DOMContentLoaded", () => {
  numOfBlitzInputEl = document.querySelector("#numOfBlitz-input");
  roundsPerBlitzInputEl = document.querySelector("#numOfRounds-input");
  document
    .querySelector("#saveClass-button")
    ?.addEventListener("click", () => saveClass());
});
