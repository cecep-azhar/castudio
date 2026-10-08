<script lang="ts">
  import { getAiSettings, saveAiSettings, generateAi, type AiSettings } from "$lib/api";
  import { onMount } from "svelte";

  interface Props {
    isOpen: boolean;
    onClose: () => void;
  }

  let { isOpen, onClose }: Props = $props();

  let settings = $state<AiSettings>({
    base_url: "http://localhost:20128/v1",
    api_key: "",
    model: "gpt-4o",
  });

  let saving = $state(false);
  let testing = $state(false);
  let testResult = $state<string | null>(null);
  let testSuccess = $state<boolean | null>(null);

  onMount(async () => {
    try {
      const s = await getAiSettings();
      settings = s;
    } catch (e) {
      console.error(e);
    }
  });

  async function handleSave() {
    saving = true;
    try {
      await saveAiSettings(settings);
      onClose();
    } catch (e) {
      alert(`Gagal menyimpan: ${e}`);
    } finally {
      saving = false;
    }
  }

  async function handleTest() {
    testing = true;
    testResult = null;
    testSuccess = null;
    try {
      await saveAiSettings(settings);
      const res = await generateAi({
        user_prompt: "Respond with exactly 'CAStudio AI OK' and nothing else.",
        temperature: 0.1,
      });
      testResult = res.text.trim();
      testSuccess = true;
    } catch (e: any) {
      testResult = `Error: ${e}`;
      testSuccess = false;
    } finally {
      testing = false;
    }
  }
</script>

{#if isOpen}
  <div class="fixed inset-0 z-50 flex items-center justify-center bg-black/70 backdrop-blur-sm p-4">
    <div class="bg-[#121217] border border-[#272732] rounded-xl w-full max-w-md overflow-hidden shadow-2xl flex flex-col">
      <!-- Modal Header -->
      <div class="px-5 py-4 border-b border-[#272732] flex items-center justify-between">
        <div class="flex items-center gap-2">
          <div class="w-3 h-3 rounded-full bg-[#8B5CF6]"></div>
          <h3 class="font-bold text-white text-sm">Pengaturan AI & Model</h3>
        </div>
        <button onclick={onClose} class="text-[#9CA3AF] hover:text-white p-1 rounded">
          <svg class="w-4 h-4" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
            <line x1="18" y1="6" x2="6" y2="18"></line>
            <line x1="6" y1="6" x2="18" y2="18"></line>
          </svg>
        </button>
      </div>

      <!-- Modal Body -->
      <div class="p-5 space-y-4 text-xs">
        <div>
          <label for="ai-base-url-input" class="block text-[#D1D5DB] font-medium mb-1.5">Base URL (OpenAI-Compatible)</label>
          <input
            id="ai-base-url-input"
            type="text"
            bind:value={settings.base_url}
            placeholder="http://localhost:20128/v1"
            class="w-full bg-[#18181F] border border-[#272732] rounded-lg px-3 py-2 text-white placeholder-[#6B7280] focus:border-[#8B5CF6] focus:outline-none"
          />
          <span class="text-[10px] text-[#6B7280] mt-1 block">Default 9Router lokal: http://localhost:20128/v1</span>
        </div>

        <div>
          <label for="ai-model-input" class="block text-[#D1D5DB] font-medium mb-1.5">Model Identifier</label>
          <input
            id="ai-model-input"
            type="text"
            bind:value={settings.model}
            placeholder="gpt-4o / deepseek-chat / claude-3-5-sonnet"
            class="w-full bg-[#18181F] border border-[#272732] rounded-lg px-3 py-2 text-white placeholder-[#6B7280] focus:border-[#8B5CF6] focus:outline-none"
          />
        </div>

        <div>
          <label for="ai-key-input" class="block text-[#D1D5DB] font-medium mb-1.5">API Key (Opsional jika 9Router lokal)</label>
          <input
            id="ai-key-input"
            type="password"
            bind:value={settings.api_key}
            placeholder="sk-..."
            class="w-full bg-[#18181F] border border-[#272732] rounded-lg px-3 py-2 text-white placeholder-[#6B7280] focus:border-[#8B5CF6] focus:outline-none"
          />
        </div>

        {#if testResult}
          <div class="p-3 rounded-lg border text-[11px] {testSuccess ? 'bg-[#064E3B]/40 border-[#059669] text-[#34D399]' : 'bg-[#7F1D1D]/40 border-[#DC2626] text-[#F87171]'}">
            {testResult}
          </div>
        {/if}
      </div>

      <!-- Modal Footer -->
      <div class="px-5 py-3 border-t border-[#272732] bg-[#0A0A0C] flex items-center justify-between">
        <button
          onclick={handleTest}
          disabled={testing}
          class="px-3 py-1.5 rounded-lg border border-[#374151] hover:bg-[#1F2937] text-[#D1D5DB] text-xs font-medium transition-colors disabled:opacity-50"
        >
          {testing ? "Menguji..." : "Test Koneksi"}
        </button>

        <div class="flex items-center gap-2">
          <button
            onclick={onClose}
            class="px-3 py-1.5 rounded-lg hover:bg-[#1E1E28] text-[#9CA3AF] text-xs transition-colors"
          >
            Batal
          </button>
          <button
            onclick={handleSave}
            disabled={saving}
            class="px-4 py-1.5 rounded-lg bg-[#8B5CF6] hover:bg-[#7C3AED] text-white text-xs font-semibold shadow-lg shadow-[#8B5CF6]/30 transition-all disabled:opacity-50"
          >
            {saving ? "Menyimpan..." : "Simpan"}
          </button>
        </div>
      </div>
    </div>
  </div>
{/if}
