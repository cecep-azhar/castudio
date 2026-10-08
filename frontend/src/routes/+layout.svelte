<script lang="ts">
  import "../app.css";
  import TitleBar from "$lib/components/TitleBar.svelte";
  import SettingsModal from "$lib/components/SettingsModal.svelte";
  import type { Snippet } from "svelte";

  let { children }: { children: Snippet } = $props();
  let showSettings = $state(false);
</script>

<div class="h-screen w-screen flex flex-col bg-[#0A0A0C] text-[#EDEDED] overflow-hidden select-none">
  <!-- Frameless TitleBar -->
  <TitleBar
    onOpenSettings={() => (showSettings = true)}
  />

  <!-- Main View -->
  <div class="flex-1 flex overflow-hidden">
    {@render children()}
  </div>

  <!-- Settings Modal -->
  <SettingsModal
    isOpen={showSettings}
    onClose={() => (showSettings = false)}
  />
</div>
