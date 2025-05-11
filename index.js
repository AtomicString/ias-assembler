import init, { MachineState, gen_encoding, step, rt_get_string } from './dist/assembler.js';
await init();

var states = [];
var state = new MachineState();
let canvas = document.getElementById("diagram");
let ctx = canvas.getContext("2d");
let ias_img = document.getElementById("arch");
const options = ["LOAD MQ","LOAD MQ,M(X)","STOR M(X)","LOAD M(X)","LOAD -M(X)","LOAD |M(X)|","LOAD -|M(X)|","JUMP M(X,0:19)","JUMP M(X,20:39)","JUMP +M(X,0:19)","JUMP +M(X,20:39)","ADD M(X)","ADD |M(X)|","SUB M(X)","SUB |M(X)|","MUL M(X)","DIV M(X)","LSH","RSH","STOR M(X, 8:19)","STOR M(X, 28:39)"];
const editableDiv = document.getElementById("editor");
const suggestionBox = document.getElementById("autocomplete-list");
ctx.drawImage(ias_img, 0, 0);

let selectedIndex = -1;

function getCaretPosition() {
    const selection = window.getSelection();
    if (!selection.rangeCount) return { x: 0, y: 0 };

    const range = selection.getRangeAt(0).cloneRange();
    range.collapse(false); // Collapse to the end

    const rect = range.getBoundingClientRect();
    const parentRect = editableDiv.getBoundingClientRect();

    // Adjust Y so that it aligns with the line rather than the caret
    const lineHeight = parseInt(window.getComputedStyle(editableDiv).lineHeight) || 20;
    
    return {
        x: 0,// + editableDiv.scrollLeft,
        y: rect.top - 40// Offset for line spacing
    };
}

function getCurrentLine() {
    const selection = window.getSelection();
    if (!selection.rangeCount) return "";

    const range = selection.getRangeAt(0);
    const textBeforeCursor = range.startContainer.textContent.substring(0, range.startOffset);

    return textBeforeCursor.split("\n").pop().trim();
}

function showSuggestions(suggestions) {
    suggestionBox.innerHTML = "";
    
    if (suggestions.length === 0) {
        suggestionBox.style.display = "none";
        return;
    }

    const { x, y } = getCaretPosition();
    suggestionBox.style.left = x + "px";
    suggestionBox.style.top = y + "px";

    suggestions.forEach((word, index) => {
        let item = document.createElement("div");
        item.classList.add("autocomplete-item");
        item.textContent = word;
        item.onclick = () => insertWord(word);
        suggestionBox.appendChild(item);
    });

    selectedIndex = -1;
    suggestionBox.style.display = "block";
}

function insertWord(word) {
    const selection = window.getSelection();
    if (!selection.rangeCount) return;

    const range = selection.getRangeAt(0);
    const textBeforeCursor = range.startContainer.textContent.substring(0, range.startOffset);

    // Get last typed word
    const words = textBeforeCursor.split(/\s+/);
    const lastTyped = words.pop(); // The partial word the user typed

    // Find the missing part of the word
    const completion = word.slice(lastTyped.length);

    // Insert only the missing part
    document.execCommand("insertText", false, completion);

    // Hide suggestions
    suggestionBox.style.display = "none";
}

editableDiv.addEventListener("input", function () {
    const currentLine = getCurrentLine();
    if (currentLine === "") {
        suggestionBox.style.display = "none";
        return;
    }

    const matchingOptions = options.filter(option => option.startsWith(currentLine.toUpperCase()));
    showSuggestions(matchingOptions);
});

function update_state(state) {
    console.log("Updating!");
    memRow.innerHTML = '';
    let memory = state.memory;
    console.log(state.memory);
    for(let i=0;i<1024;i++) {
        let clonedMem = memTemplate.content.cloneNode(true);
        let input = clonedMem.querySelector("input");
        input.addEventListener("change", (event) => {
            let hex_re = /[0-9A-Fa-f]+/g;
            if (hex_re.test(event.target.value)) {
                memory[i] = BigInt(parseInt(event.target.value.toLowerCase(), 16));
            } else {
                memory[i] = BigInt(0);
            }
            state.memory = memory
        });
        input.value = memory[i].toString(16).toUpperCase();
        let label = document.createElement("span");
        label.textContent = `${i}: `;
        clonedMem.firstElementChild.prepend(label);
        memRow.append(clonedMem)
    }

    let pcVal = registerCol.querySelector("#PC>.val");
    pcVal.innerText = state.get_reg_stack.pc;
    let mqVal = registerCol.querySelector("#MQ>.val");
    mqVal.innerText = state.get_reg_stack.mq;
    let acVal = registerCol.querySelector("#AC>.val");
    acVal.innerText = state.get_reg_stack.ac;

}

const compileBtn = document.getElementById("compile");
const memTemplate = document.getElementById("mem");
const memRow = document.getElementById("mem-row");
const registerCol = document.getElementById("register");

compileBtn.addEventListener("click", (event) => {
    state = new MachineState();
    let program = editor.innerText;
    if (program[program.length - 1] !== '\n') {
        program += "\n";
    }
    gen_encoding(program, state);
    update_state(state);
});

const nextBtn = document.getElementById("next");
nextBtn.addEventListener("click", (event) => {
    states.push(state.get_clone());
    console.log(states.length)
    step(state);
    update_state(state);
    document.getElementById("prev").disabled = false;
});

const prevBtn = document.getElementById("prev");
prevBtn.addEventListener("click", (event) => {
    if (states.length > 0) {
        state = states.pop();
        update_state(state);
        if (states.length == 0) {
            event.target.disabled = true;
        }
    } else {
        console.error("What happened?");
    }
});

editableDiv.addEventListener("keydown", function (e) {
    const items = document.querySelectorAll(".autocomplete-item");
    
    if (e.ctrlKey && e.key === " ") {
        e.preventDefault();
        const currentWord = getCurrentWord();
        const matchingOptions = options.filter(option => option.startsWith(currentWord));
        showSuggestions(matchingOptions);
        return;
    }
    
    if (items.length === 0) return;

    if (e.key === "ArrowDown" && items.length > 0) {
        e.preventDefault();
        selectedIndex = (selectedIndex + 1) % items.length;
    } else if (e.key === "ArrowUp" && items.length > 0) {
        e.preventDefault();
        selectedIndex = (selectedIndex - 1 + items.length) % items.length;
    } else if (e.key === "Enter" && selectedIndex >= 0) {
        e.preventDefault();
        insertWord(items[selectedIndex].textContent);
    }

    items.forEach((item, index) => {
        item.classList.toggle("active", index === selectedIndex);
    });

    if (selectedIndex >= 0) {
        const selectedItem = items[selectedIndex];
        const listRect = suggestionBox.getBoundingClientRect();
        const itemRect = selectedItem.getBoundingClientRect();

        if (itemRect.top < listRect.top) {
            // Scroll up
            suggestionBox.scrollTop -= listRect.top - itemRect.top;
        } else if (itemRect.bottom > listRect.bottom) {
            // Scroll down
            suggestionBox.scrollTop += itemRect.bottom - listRect.bottom;
        }
    }
});

 document.addEventListener("click", (e) => {
    if (!editableDiv.contains(e.target)) {
        suggestionBox.style.display = "none";
    }
});
