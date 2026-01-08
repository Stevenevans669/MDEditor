const { invoke } = window.__TAURI__.core;
const { ask } = window.__TAURI__.dialog;

let filePath = '';
let originalContent = '';
let editor;

function isDirty() {
  return editor.value !== originalContent;
}

async function init() {
  editor = document.getElementById('editor');

  try {
    filePath = await invoke('get_file_path');
    originalContent = await invoke('read_file', { path: filePath });
    editor.value = originalContent;
    editor.placeholder = '';
    editor.focus();
  } catch (error) {
    await window.__TAURI__.dialog.message(`Error: ${error}`, { title: 'MDEditor', kind: 'error' });
    await invoke('exit_app', { code: 2 });
  }
}

async function saveAndExit() {
  try {
    await invoke('save_file', { path: filePath, content: editor.value });
    originalContent = editor.value;
    await invoke('exit_app', { code: 0 });
  } catch (error) {
    await window.__TAURI__.dialog.message(`Save failed: ${error}`, { title: 'MDEditor', kind: 'error' });
  }
}

async function cancelAndExit() {
  await invoke('exit_app', { code: 1 });
}

async function handleClose() {
  if (!isDirty()) {
    await invoke('exit_app', { code: 0 });
    return;
  }

  const result = await ask('You have unsaved changes. Do you want to save before closing?', {
    title: 'MDEditor',
    kind: 'warning',
    okLabel: 'Save',
    cancelLabel: "Don't Save"
  });

  if (result) {
    await saveAndExit();
  } else {
    await cancelAndExit();
  }
}

document.addEventListener('DOMContentLoaded', init);

document.addEventListener('keydown', async (e) => {
  if (e.metaKey && e.key === 'Enter') {
    e.preventDefault();
    await saveAndExit();
  }

  if (e.metaKey && e.key === 'w') {
    e.preventDefault();
    await handleClose();
  }
});

document.getElementById('save-btn')?.addEventListener('click', saveAndExit);
document.getElementById('cancel-btn')?.addEventListener('click', cancelAndExit);

window.addEventListener('beforeunload', (e) => {
  if (isDirty()) {
    e.preventDefault();
    e.returnValue = '';
  }
});
