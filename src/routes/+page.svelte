<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { open as openDialog } from '@tauri-apps/plugin-dialog';
  import { open, BaseDirectory, readFile } from '@tauri-apps/plugin-fs';
  import { SkinViewer } from "skinview3d";
  import {
    warn,
    debug,
    info,
    error,
  } from '@tauri-apps/plugin-log';

  let name = "";
  let greetMsg = "";

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    greetMsg = await invoke("greet", { name });
  }

  async function loadSkin(skin: Uint8Array) {
    const base64String = btoa(String.fromCharCode(...new Uint8Array(skin)));
    const skinUrl = `data:image/png;base64,${base64String}`;
    // debug(skinUrl);

    let skinViewer = new SkinViewer({
      canvas: document.getElementById("skin_container") as HTMLCanvasElement,
      width: 300,
      height: 400,
      skin: skinUrl,
    });

    return skinViewer;
  }
  async function openFile() {
    const file = await openDialog({
      multiple: false,
      directory: false,
      filters: [
        {
          name: 'Aseprite Format',
          extensions: ['ase', 'aseprite']
        },
        {
          name: 'Minecraft Skin Format',
          extensions: ['png']
        },
        {
          name: 'All Files',
          extensions: ['*']
        }
      ]
    });
    if (file) {
      debug(file);
      const skin = await readFile(file);

      let skinViewer = loadSkin(skin);
    } else {
      warn('No file selected');
    }
  }
</script>

<div class="container">
  <h1>Aseskin</h1>

  <button on:click={openFile}>Upload File</button>

  <form class="row" on:submit|preventDefault={greet}>
    <input id="greet-input" placeholder="Enter a name..." bind:value={name} />
    <button type="submit">Greet</button>
  </form>

  <p>{greetMsg}</p>

  <canvas id="skin_container"></canvas>
</div>

<style>
:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;

  color: #0f0f0f;
  background-color: #f6f6f6;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

.container {
  margin: 0;
  padding-top: 10vh;
  display: flex;
  flex-direction: column;
  justify-content: center;
  text-align: center;
}


.row {
  display: flex;
  justify-content: center;
}

input,
button {
  border-radius: 8px;
  border: 1px solid transparent;
  padding: 0.6em 1.2em;
  font-size: 1em;
  font-weight: 500;
  font-family: inherit;
  color: #0f0f0f;
  background-color: #ffffff;
  transition: border-color 0.25s;
  box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
}

button {
  cursor: pointer;
}

button:hover {
  border-color: #396cd8;
}
button:active {
  border-color: #396cd8;
  background-color: #e8e8e8;
}

input,
button {
  outline: none;
}

#greet-input {
  margin-right: 5px;
}

@media (prefers-color-scheme: dark) {
  :root {
    color: #f6f6f6;
    background-color: #2f2f2f;
  }

  input,
  button {
    color: #ffffff;
    background-color: #0f0f0f98;
  }
  button:active {
    background-color: #0f0f0f69;
  }
}

</style>
