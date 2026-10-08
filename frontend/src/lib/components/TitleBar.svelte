<script lang="ts">
  import { windowMinimize, windowMaximize, windowClose } from "$lib/api";

  interface Props {
    activeProjectTitle?: string;
    onOpenSettings?: () => void;
  }

  let { activeProjectTitle = "Belum Ada Proyek", onOpenSettings }: Props = $props();
</script>

<div
  data-tauri-drag-region
  class="h-10 bg-[#0A0A0C] border-b border-[#272732] flex items-center justify-between px-3 select-none text-xs text-[#9CA3AF] z-50 shrink-0"
>
  <!-- Logo & Title -->
  <div class="flex items-center gap-2.5">
    <div class="w-5 h-5 rounded-md bg-[#0A0A0C] border border-[#8B5CF6]/40 flex items-center justify-center shadow-[0_0_10px_rgba(139,92,246,0.3)]">
      <svg class="w-3.5 h-3.5" viewBox="0 0 512 512" fill="none">
        <defs>
          <linearGradient id="symbolGrad" x1="0%" y1="0%" x2="100%" y2="100%">
            <stop offset="0%" stop-color="#C084FC" />
            <stop offset="100%" stop-color="#8B5CF6" />
          </linearGradient>
        </defs>
        <polygon points="256,92 396,256 256,420 116,256" stroke="url(#symbolGrad)" stroke-width="28" stroke-linejoin="round" stroke-linecap="round"/>
        <line x1="256" y1="92" x2="256" y2="420" stroke="#C084FC" stroke-width="28" stroke-linecap="round"/>
        <line x1="116" y1="256" x2="396" y2="256" stroke="#A855F7" stroke-width="28" stroke-linecap="round"/>
      </svg>
    </div>
    <span class="font-bold tracking-wider text-white text-sm">CASTUDIO</span>
    <span class="text-[#4B5563]">/</span>
    <span class="text-[#D1D5DB] font-medium truncate max-w-[200px]">{activeProjectTitle}</span>
  </div>

  <!-- Center Status -->
  <div class="hidden md:flex items-center gap-2">
    <div class="w-2 h-2 rounded-full bg-[#10B981] animate-pulse"></div>
    <span class="text-[11px] text-[#6B7280]">AI Studio Online (OpenAI-compatible)</span>
  </div>

  <!-- Right Actions & Window Controls -->
  <div class="flex items-center gap-1">
    {#if onOpenSettings}
      <button
        onclick={onOpenSettings}
        title="Pengaturan AI & Model"
        class="p-1.5 hover:bg-[#1E1E28] rounded text-[#9CA3AF] hover:text-white transition-colors mr-2"
      >
        <svg class="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <circle cx="12" cy="12" r="3"></circle>
          <path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z"></path>
        </svg>
      </button>
    {/if}

    <!-- Minimize -->
    <button
      onclick={windowMinimize}
      class="p-1.5 hover:bg-[#1E1E28] rounded text-[#9CA3AF] hover:text-white transition-colors"
      title="Minimize"
    >
      <svg class="w-3.5 h-3.5" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
        <line x1="5" y1="12" x2="19" y2="12"></line>
      </svg>
    </button>

    <!-- Maximize -->
    <button
      onclick={windowMaximize}
      class="p-1.5 hover:bg-[#1E1E28] rounded text-[#9CA3AF] hover:text-white transition-colors"
      title="Maximize"
    >
      <svg class="w-3.5 h-3.5" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
        <rect x="3" y="3" width="18" height="18" rx="2" ry="2"></rect>
      </svg>
    </button>

    <!-- Close -->
    <button
      onclick={windowClose}
      class="p-1.5 hover:bg-[#EF4444] rounded text-[#9CA3AF] hover:text-white transition-colors"
      title="Close"
    >
      <svg class="w-3.5 h-3.5" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
        <line x1="18" y1="6" x2="6" y2="18"></line>
        <line x1="6" y1="6" x2="18" y2="18"></line>
      </svg>
    </button>
  </div>
</div>
