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

  let fileOpened = false;

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
          name: 'Minecraft Skin Files',
          extensions: ['ase', 'aseprite', 'png']
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
      fileOpened = true;
    } else {
      warn('No file selected');
    }
  }
</script>

<div class="container">
  {#if !fileOpened}
  <button on:click={openFile}>Upload File</button>
  {/if}
  <canvas id="skin_container"></canvas>

</div>

<style lang="postcss">
  #skin_container {
    width: 600px;
  }
</style>
