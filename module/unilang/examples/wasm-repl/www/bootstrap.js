// Bootstrap script for UniLang WebAssembly REPL

let wasm_module;
let repl_instance;

async function init() {
  try {
    // Import the WASM module
    wasm_module = await import('../pkg/unilang_wasm_repl.js');

    // Create a new REPL instance
    repl_instance = new wasm_module.UniLangWasmRepl();

    console.log('UniLang WASM REPL initialized successfully!');

    // Set up event listeners
    setupEventListeners();

    // Show initial message
    appendOutput('system', 'UniLang WebAssembly REPL loaded successfully!');

  } catch (error) {
    console.error('Failed to initialize WASM module:', error);
    appendOutput('error', `Failed to load WebAssembly module: ${error.message}`);
  }
}

function setupEventListeners() {
  const commandInput = document.getElementById('command-input');
  const executeBtn = document.getElementById('execute-btn');

  // Execute command on button click
  executeBtn.addEventListener('click', executeCommand);

  // Command history
  let commandHistory = [];
  let historyIndex = -1;

  commandInput.addEventListener('keydown', (event) => {
    if (event.key === 'Enter') {
      const command = commandInput.value.trim();
      if (command && commandHistory[commandHistory.length - 1] !== command) {
        commandHistory.push(command);
        if (commandHistory.length > 100) {
          commandHistory.shift();
        }
      }
      historyIndex = -1;
      executeCommand();
    } else if (event.key === 'ArrowUp') {
      event.preventDefault();
      if (historyIndex < commandHistory.length - 1) {
        historyIndex++;
        commandInput.value = commandHistory[commandHistory.length - 1 - historyIndex];
      }
    } else if (event.key === 'ArrowDown') {
      event.preventDefault();
      if (historyIndex > 0) {
        historyIndex--;
        commandInput.value = commandHistory[commandHistory.length - 1 - historyIndex];
      } else if (historyIndex === 0) {
        historyIndex = -1;
        commandInput.value = '';
      }
    }
  });
}

function executeCommand() {
  const commandInput = document.getElementById('command-input');
  const command = commandInput.value.trim();

  if (!command) return;

  if (!repl_instance) {
    appendOutput('error', 'REPL not initialized. Please refresh the page.');
    return;
  }

  // Show the command being executed
  appendOutput('command', `> ${command}`);

  try {
    const result = repl_instance.execute_command(command);

    if (result.startsWith('Error:') || result.startsWith('Unknown')) {
      appendOutput('error', result);
    } else {
      appendOutput('success', result);
    }

  } catch (error) {
    console.error('Command execution error:', error);
    appendOutput('error', `Execution error: ${error.message}`);
  }

  // Clear the input
  commandInput.value = '';
}

function appendOutput(type, text) {
  const outputDiv = document.getElementById('output');
  const lineDiv = document.createElement('div');
  lineDiv.className = `command-line`;

  const contentDiv = document.createElement('div');

  switch (type) {
    case 'command':
      contentDiv.className = 'command-input';
      contentDiv.textContent = text;
      break;
    case 'success':
      contentDiv.className = 'command-output';
      contentDiv.textContent = text;
      break;
    case 'error':
      contentDiv.className = 'command-error';
      contentDiv.textContent = text;
      break;
    case 'system':
      contentDiv.className = 'command-output';
      contentDiv.style.color = '#68d391';
      contentDiv.textContent = text;
      break;
  }

  lineDiv.appendChild(contentDiv);
  outputDiv.appendChild(lineDiv);

  // Auto-scroll to bottom
  outputDiv.scrollTop = outputDiv.scrollHeight;
}

document.addEventListener('DOMContentLoaded', () => {
  init();
});
