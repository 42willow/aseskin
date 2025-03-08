<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { open as openDialog } from "@tauri-apps/plugin-dialog";
  import { watchImmediate, type WatchEvent } from "@tauri-apps/plugin-fs";
  import { SkinViewer } from "skinview3d";
  import { warn, debug, info, error } from "@tauri-apps/plugin-log";
  import { onMount } from "svelte";

  let fileOpened = false;

  let skinViewer: SkinViewer = new SkinViewer();

  onMount(() => {
    skinViewer = new SkinViewer({
      canvas: document.getElementById("skin_container") as HTMLCanvasElement,
      width: 300,
      height: 400,
    });
  });

  async function loadSkin(skin: Uint8Array) {
    const base64String = btoa(String.fromCharCode(...new Uint8Array(skin)));
    const skinUrl = `data:image/png;base64,${base64String}`;

    skinViewer.loadSkin(skinUrl);

    return skinViewer;
  }

  async function callCLI(path: string) {
    invoke("cli_export", { path: path })
      .then((res) => loadSkin(res as Uint8Array))
      .catch((e) => error(e));
  }

  async function openFile() {
    const file = await openDialog({
      multiple: false,
      directory: false,
      filters: [
        {
          name: "Aseprite Files",
          extensions: ["ase", "aseprite"],
        },
        {
          name: "All Files",
          extensions: ["*"],
        },
      ],
    });

    if (file) {
      debug(file);
      callCLI(file);

      const stopWatching = await watchImmediate(file, (event: WatchEvent) => {
        // if write event
        if (
          typeof event.type === "object" &&
          "access" in event.type &&
          "mode" in event.type.access &&
          event.type.access.mode === "write"
        ) {
          info(`Changed: ${file}`);
          callCLI(file);
        }
      });

      // TODO)) on destroy stop watching
      // stopWatching();
      // fileOpened = true;
    } else {
      warn("No file selected");
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
