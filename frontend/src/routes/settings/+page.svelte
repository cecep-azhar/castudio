<script lang="ts">
  import { onMount } from "svelte";
  import { getAiSettings, saveAiSettings, type AiSettings } from "$lib/api";

  let settings = $state<AiSettings>({
    base_url: "http://localhost:20128/v1",
    model: "gpt-4o",
    api_key: "",
  });

  let isSaving = $state(false);
  let statusMessage = $state<string | null>(null);

  onMount(async () => {
    try {
      const s = await getAiSettings();
      settings = s;
    } catch (e) {
      console.warn("Gagal load AI settings:", e);
    }
  });

  async function handleSave() {
    isSaving = true;
    statusMessage = null;
    try {
      await saveAiSettings(settings);
      statusMessage = "Pengaturan berhasil disimpan ke SQLite lokal!";
    } catch (e: any) {
      statusMessage = `Gagal menyimpan: ${e}`;
    } finally {
      isSaving = false;
    }
  }
</script>

<div class="flex-1 flex flex-col h-full bg-[#0A0A0C] overflow-hidden select-none font-sans">
  <header class="h-14 border-b border-[#272732] px-8 flex items-center justify-between bg-[#121217] shrink-0">
    <div>
      <h1 class="font-bold text-white text-base">Pengaturan & Konfigurasi Studio</h1>
      <p class="text-[11px] text-[#9CA3AF]">Konfigurasi endpoint AI (9Router / OpenAI-compatible) & kredensial lokal.</p>
    </div>
  </header>

  <div class="flex-1 overflow-y-auto p-8 max-w-2xl mx-auto w-full space-y-6">
    <div class="p-6 bg-[#121217] border border-[#272732] rounded-2xl space-y-4">
      <div class="flex items-center gap-3 border-b border-[#272732] pb-3">
        <span class="text-xl">🤖</span>
        <div>
          <h3 class="font-bold text-white text-sm">AI Engine Provider</h3>
          <p class="text-[11px] text-neutral-400">Hubungkan ke 9Router lokal (port 20128) atau penyedia OpenAI.</p>
        </div>
      </div>

      <div class="space-y-4 text-xs">
        <div>
          <label for="settings-base-url-input" class="block font-medium text-neutral-300 mb-1">Base URL Endpoint:</label>
          <input
            id="settings-base-url-input"
            type="text"
            bind:value={settings.base_url}
            placeholder="http://localhost:20128/v1"
            class="w-full bg-[#18181F] border border-[#272732] rounded-lg px-3 py-2 text-white font-mono focus:border-[#8B5CF6] focus:outline-none"
          />
          <p class="text-[10px] text-neutral-500 mt-1">Default 9Router CA: <code>http://localhost:20128/v1</code></p>
        </div>

        <div>
          <label for="settings-model-input" class="block font-medium text-neutral-300 mb-1">Model Name:</label>
          <input
            id="settings-model-input"
            type="text"
            bind:value={settings.model}
            placeholder="gpt-4o"
            class="w-full bg-[#18181F] border border-[#272732] rounded-lg px-3 py-2 text-white font-mono focus:border-[#8B5CF6] focus:outline-none"
          />
        </div>

        <div>
          <label for="settings-api-key-input" class="block font-medium text-neutral-300 mb-1">API Key (Opsional untuk 9Router):</label>
          <input
            id="settings-api-key-input"
            type="password"
            bind:value={settings.api_key}
            placeholder="sk-..."
            class="w-full bg-[#18181F] border border-[#272732] rounded-lg px-3 py-2 text-white font-mono focus:border-[#8B5CF6] focus:outline-none"
          />
        </div>

        {#if statusMessage}
          <div class="p-3 rounded-lg bg-[#0A0A0C] border border-[#272732] text-[11px] {statusMessage.includes('berhasil') ? 'text-emerald-400' : 'text-red-400'}">
            {statusMessage}
          </div>
        {/if}

        <button
          onclick={handleSave}
          disabled={isSaving}
          class="w-full py-2.5 rounded-xl bg-[#8B5CF6] hover:bg-[#7C3AED] text-white text-xs font-bold transition-all shadow-md shadow-[#8B5CF6]/30 cursor-pointer disabled:opacity-50"
        >
          {isSaving ? "Menyimpan..." : "Simpan Konfigurasi"}
        </button>
      </div>
    </div>
  </div>
</div>
