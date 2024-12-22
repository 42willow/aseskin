<script lang="ts">
  import { Plus, Settings } from "lucide-svelte";

  let activeTab = 'Tab 1';

  function setActiveTab(tab: string) {
    activeTab = tab;
  }
</script>

<div class="bg-ctp-surface0 p-3 flex justify-between items-center">
  <div class="flex items-center space-x-2">
    <button class="tab {activeTab === 'Tab 1' ? 'active' : ''}" on:click={() => setActiveTab('Tab 1')}>Tab 1</button>
    <button class="tab {activeTab === 'Tab 2' ? 'active' : ''}" on:click={() => setActiveTab('Tab 2')}>Tab 2</button>
    <button class="tab {activeTab === 'Tab 3' ? 'active' : ''}" on:click={() => setActiveTab('Tab 3')}>Tab 3</button>
    <Plus />
  </div>
  <Settings />
</div>

<style lang="postcss">
  .tab {
    @apply text-ctp-text py-1 px-2 hover:bg-ctp-surface1 transition-colors duration-300;
  }

  .tab.active {
    @apply bg-ctp-pink text-ctp-mantle;
  }
</style>
