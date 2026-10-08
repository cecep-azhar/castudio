<script lang="ts">
  import "../app.css";
  import { page } from "$app/state";
  import { onMount, type Snippet } from "svelte";
  import { windowMinimize, windowMaximize, windowClose } from "$lib/api";
  import { projectStore } from "$lib/stores/projectStore.svelte";
  import FloatingAiCard from "$lib/components/FloatingAiCard.svelte";
  import SettingsModal from "$lib/components/SettingsModal.svelte";

  let { children }: { children: Snippet } = $props();

  let isCollapsed = $state(false);
  let showSettingsModal = $state(false);

  onMount(async () => {
    await projectStore.init();
  });

  const menus = [
    {
      key: "projects",
      label: "Proyek & Dokumen",
      href: "/",
      icon: "M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z",
    },
    {
      key: "content",
      label: "Content Builder",
      href: "/content",
      icon: "M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z",
    },
    {
      key: "instagram",
      label: "Instagram Suite",
      href: "/instagram",
      icon: "M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z",
    },
    {
      key: "ebook",
      label: "E-Book Studio",
      href: "/ebook",
      icon: "M12 6.253v13m0-13C10.832 5.477 9.246 5 7.5 5S4.168 5.477 3 6.253v13C4.168 18.477 5.754 18 7.5 18s3.332.477 4.5 1.253m0-13C13.168 5.477 14.754 5 16.5 5c1.747 0 3.332.477 4.5 1.253v13C19.832 18.477 18.247 18 16.5 18c-1.746 0-3.332.477-4.5 1.253",
    },
    {
      key: "pitchdeck",
      label: "Pitchdeck & SPK",
      href: "/pitchdeck",
      icon: "M7 12l3-3 3 3 4-4M8 21l4-4 4 4M3 4h18M4 4h16v12a1 1 0 01-1 1H5a1 1 0 01-1-1V4z",
    },
    {
      key: "repurpose",
      label: "Omni-Repurpose",
      href: "/repurpose",
      icon: "M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15",
    },
    {
      key: "automation",
      label: "Automation & Drip",
      href: "/automation",
      icon: "M13 10V3L4 14h7v7l9-11h-7z",
    },
    {
      key: "prompt_studio",
      label: "Prompt Studio",
      href: "/prompt-studio",
      icon: "M5 3v4M3 5h4M6 17v4m-2-2h4m5-16l2.286 6.857L21 12l-5.714 2.286L13 21l-2.286-6.857L5 12l5.714-2.286L13 3z",
    },
    {
      key: "settings",
      label: "Pengaturan",
      href: "/settings",
      icon: "M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z M15 12a3 3 0 11-6 0 3 3 0 016 0z",
    },
  ];

  function isActive(href: string): boolean {
    if (href === "/") {
      return page.url.pathname === "/";
    }
    return page.url.pathname.startsWith(href);
  }
</script>

<div class="h-screen w-screen flex flex-col bg-[#0A0A0C] text-[#EDEDED] overflow-hidden select-none font-sans">
  <!-- CADS v1.0 Frameless TitleBar -->
  <header
    data-tauri-drag-region
    class="h-11 bg-[#0A0A0C] border-b border-[#272732] flex items-center justify-between px-3 select-none text-xs text-[#9CA3AF] z-50 shrink-0"
  >
    <!-- Left: Brand Logo & Breadcrumb Navigation -->
    <div class="flex items-center gap-2.5">
      <div class="w-5 h-5 rounded-md bg-[#0A0A0C] border border-[#8B5CF6]/50 flex items-center justify-center shadow-[0_0_10px_rgba(139,92,246,0.3)]">
        <svg class="w-3.5 h-3.5" viewBox="0 0 512 512" fill="none">
          <defs>
            <linearGradient id="studioSymbol" x1="0%" y1="0%" x2="100%" y2="100%">
              <stop offset="0%" stop-color="#C084FC" />
              <stop offset="100%" stop-color="#8B5CF6" />
            </linearGradient>
          </defs>
          <polygon points="256,92 396,256 256,420 116,256" stroke="url(#studioSymbol)" stroke-width="32" stroke-linejoin="round" stroke-linecap="round"/>
          <line x1="256" y1="92" x2="256" y2="420" stroke="#C084FC" stroke-width="28" stroke-linecap="round"/>
          <line x1="116" y1="256" x2="396" y2="256" stroke="#A855F7" stroke-width="28" stroke-linecap="round"/>
        </svg>
      </div>
      <span class="font-bold tracking-wider text-white text-xs">CASTUDIO</span>
      <span class="text-[#4B5563]">/</span>
      <span class="text-[#D1D5DB] font-medium truncate max-w-[240px]">
        {projectStore.activeProject?.title || "Sovereign Engineering Studio"}
      </span>
    </div>

    <!-- Center Status Indicator -->
    <div class="hidden md:flex items-center gap-2">
      <div class="w-1.5 h-1.5 rounded-full bg-emerald-400 animate-pulse"></div>
      <span class="text-[11px] text-[#6B7280] font-mono">CADS v1.0 · Zero-Knowledge Local Engine</span>
    </div>

    <!-- Right Controls: Pro Badge, Settings, Window Controls -->
    <div class="flex items-center gap-1.5">
      <!-- Pro Status Badge with Aurora Glow -->
      <div class="hidden sm:flex items-center gap-1 px-2 py-0.5 rounded text-[10px] font-bold text-amber-300 bg-amber-500/15 border border-amber-500/30 shadow-xs mr-1">
        <span>★</span>
        <span>PRO EDITION</span>
      </div>

      <!-- Settings Quick Trigger -->
      <button
        onclick={() => (showSettingsModal = true)}
        title="Pengaturan AI & Model"
        class="p-1.5 hover:bg-[#1E1E28] rounded text-[#9CA3AF] hover:text-white transition-colors"
      >
        <svg class="w-3.5 h-3.5" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <circle cx="12" cy="12" r="3"></circle>
          <path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z"></path>
        </svg>
      </button>

      <!-- Divider -->
      <div class="h-3.5 w-px bg-neutral-800 mx-0.5"></div>

      <!-- Frameless Window Controls -->
      <button
        onclick={windowMinimize}
        class="p-1.5 hover:bg-[#1E1E28] rounded text-[#9CA3AF] hover:text-white transition-colors"
        title="Minimize"
      >
        <svg class="w-3 h-3" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
          <line x1="5" y1="12" x2="19" y2="12"></line>
        </svg>
      </button>
      <button
        onclick={windowMaximize}
        class="p-1.5 hover:bg-[#1E1E28] rounded text-[#9CA3AF] hover:text-white transition-colors"
        title="Maximize"
      >
        <svg class="w-3 h-3" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <rect x="3" y="3" width="18" height="18" rx="2" ry="2"></rect>
        </svg>
      </button>
      <button
        onclick={windowClose}
        class="p-1.5 hover:bg-rose-500 rounded text-[#9CA3AF] hover:text-white transition-colors"
        title="Close"
      >
        <svg class="w-3 h-3" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
          <line x1="18" y1="6" x2="6" y2="18"></line>
          <line x1="6" y1="6" x2="18" y2="18"></line>
        </svg>
      </button>
    </div>
  </header>

  <!-- Shell Body: Collapsible Sidebar + Raised Content Container -->
  <div class="flex-1 flex min-h-0 overflow-hidden">
    <!-- Collapsible Sidebar -->
    <aside
      class="border-r border-[#272732] bg-[#0A0A0C] flex flex-col justify-between py-2 shrink-0 transition-all duration-200 ease-in-out z-20 {isCollapsed ? 'w-16' : 'w-56'}"
    >
      <div class="space-y-1 px-2">
        <!-- Collapse Toggle Button -->
        <div class="flex items-center justify-between pb-2 mb-1 border-b border-[#272732]/60 px-1">
          {#if !isCollapsed}
            <span class="text-[10px] uppercase font-bold text-neutral-500 font-mono tracking-wider">
              Menu Studio
            </span>
          {/if}
          <button
            onclick={() => (isCollapsed = !isCollapsed)}
            class="p-1.5 rounded-lg text-neutral-400 hover:text-white hover:bg-[#18181F] transition-colors ml-auto"
            title={isCollapsed ? "Perluas sidebar" : "Ciutkan sidebar"}
          >
            <svg class="w-3.5 h-3.5 transition-transform duration-200 {isCollapsed ? 'rotate-180' : ''}" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 19l-7-7 7-7m8 14l-7-7 7-7" />
            </svg>
          </button>
        </div>

        <!-- Navigation Links -->
        <nav class="space-y-1">
          {#each menus as m}
            {@const active = isActive(m.href)}
            <a
              href={m.href}
              title={isCollapsed ? m.label : ""}
              class="relative flex items-center gap-3 px-2.5 py-2 rounded-xl text-xs font-medium transition-all group overflow-hidden {active ? 'bg-[#8B5CF6]/15 text-[#C4B5FD] font-bold shadow-xs' : 'text-neutral-400 hover:text-white hover:bg-[#18181F]'}"
            >
              {#if active}
                <!-- Pinned Active Marker -->
                <span class="absolute left-0 top-1.5 bottom-1.5 w-1 rounded-r bg-[#8B5CF6]"></span>
              {/if}
              <div class="w-5 h-5 flex items-center justify-center shrink-0">
                <svg class="w-4 h-4 transition-transform group-hover:scale-110" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.8" d={m.icon} />
                </svg>
              </div>
              {#if !isCollapsed}
                <span class="truncate whitespace-nowrap">{m.label}</span>
              {/if}
            </a>
          {/each}
        </nav>
      </div>

      <!-- Sidebar Footer (Profile / Project Indicator) -->
      <div class="p-2 border-t border-[#272732]/60">
        {#if !isCollapsed}
          <div class="p-2 rounded-xl bg-[#121217] border border-[#272732] flex items-center gap-2 text-xs">
            <div class="w-7 h-7 rounded-lg bg-[#8B5CF6]/20 border border-[#8B5CF6]/40 flex items-center justify-center text-xs font-bold text-[#A78BFA] shrink-0">
              CA
            </div>
            <div class="truncate">
              <span class="font-bold text-white text-[11px] block truncate">Prof. Cecep</span>
              <span class="text-[9px] text-neutral-400 font-mono block">Sovereign Author</span>
            </div>
          </div>
        {:else}
          <div class="w-8 h-8 mx-auto rounded-lg bg-[#8B5CF6]/20 border border-[#8B5CF6]/40 flex items-center justify-center text-xs font-bold text-[#A78BFA]" title="Prof. Cecep (Sovereign Author)">
            CA
          </div>
        {/if}
      </div>
    </aside>

    <!-- Raised Content Container (CADS v1.0 standard: #0e0e0e outer, elevated card #161616 with rounded-tl-xl) -->
    <main class="flex-1 min-w-0 flex flex-col overflow-hidden bg-[#0A0A0C]">
      <div class="flex-1 min-w-0 flex flex-col overflow-hidden bg-[#121217] rounded-tl-2xl border-t border-l border-[#272732] shadow-2xl relative">
        {@render children()}
      </div>
    </main>
  </div>

  <!-- Global Floating Smart Card AI Assistant -->
  <FloatingAiCard />

  <!-- Global Settings Modal -->
  <SettingsModal
    isOpen={showSettingsModal}
    onClose={() => (showSettingsModal = false)}
  />
</div>
