<script lang="ts">
  import { generateAi, type AiResponse } from "$lib/api";
  import { projectStore } from "$lib/stores/projectStore.svelte";

  let isOpen = $state(false);
  let isMinimized = $state(false);
  let promptText = $state("");
  let isGenerating = $state(false);

  interface ChatMessage {
    id: string;
    role: "user" | "assistant";
    content: string;
    timestamp: number;
  }

  let messages = $state<ChatMessage[]>([
    {
      id: "msg_welcome",
      role: "assistant",
      content:
        "Halo Prof. Cecep! Saya Hana, asisten kepenulisan otonom CAStudio. Ada naskah, hook, atau ide yang ingin dipoles?",
      timestamp: Date.now(),
    },
  ]);

  const quickPrompts = [
    {
      label: "⚡ Hook 3s Viral",
      prompt:
        "Tuliskan 3 alternatif hook 3 detik pembuka yang kontrarian dan memicu rasa penasaran tinggi untuk materi ini:",
    },
    {
      label: "💎 Polish Tone",
      prompt:
        "Tingkatkan nada tulisan agar lebih berkarakter Sovereign Developer: padat, tegas, teknis presisi, tanpa basa-basi.",
    },
    {
      label: "📝 Ringkasan TL;DR",
      prompt:
        "Buatkan ringkasan eksekutif 3 poin penting (Key Takeaways) dari teks berikut:",
    },
    {
      label: "🌐 Terjemah Presisi",
      prompt:
        "Terjemahkan dengan terminologi software engineering baku yang natural:",
    },
  ];

  async function handleSend(customText?: string) {
    const textToSend = customText || promptText.trim();
    if (!textToSend || isGenerating) return;

    const userMsg: ChatMessage = {
      id: `usr_${Date.now()}`,
      role: "user",
      content: textToSend,
      timestamp: Date.now(),
    };
    messages.push(userMsg);
    promptText = "";
    isGenerating = true;

    try {
      const bible = projectStore.activeBible;
      const res: AiResponse = await generateAi({
        project_id: projectStore.activeProject?.id,
        user_prompt: textToSend,
        system_instruction: `Kamu adalah asisten kepenulisan CAStudio yang tajam dan berwibawa. Gaya bahasa: ${bible?.voice_sample || "Terse, tegas, presisi"}. Fakta inti: ${bible?.core_facts || "Sovereign developer, zero-knowledge, local-first"}.`,
      });

      const aiMsg: ChatMessage = {
        id: `ai_${Date.now()}`,
        role: "assistant",
        content: res.text,
        timestamp: Date.now(),
      };
      messages.push(aiMsg);
    } catch (e: any) {
      messages.push({
        id: `err_${Date.now()}`,
        role: "assistant",
        content: `Maaf, gagal memanggil AI engine: ${e}`,
        timestamp: Date.now(),
      });
    } finally {
      isGenerating = false;
    }
  }

  function handleKeyDown(e: KeyboardEvent) {
    if ((e.metaKey || e.ctrlKey) && e.key === "Enter") {
      handleSend();
    }
  }

  function copyText(txt: string) {
    navigator.clipboard.writeText(txt);
    alert("Teks disalin ke clipboard!");
  }
</script>

<!-- Floating Button (When Card is Closed) -->
{#if !isOpen}
  <button
    type="button"
    onclick={() => (isOpen = true)}
    class="fixed bottom-5 right-5 z-50 flex items-center gap-2.5 px-4 py-2.5 rounded-full bg-[#121217] hover:bg-[#18181F] text-white border border-[#8B5CF6]/50 shadow-2xl shadow-[#8B5CF6]/20 hover:scale-105 active:scale-95 transition-all text-xs font-semibold cursor-pointer group"
    title="Buka Hana AI Assistant"
  >
    <span class="w-2 h-2 rounded-full bg-[#A78BFA] animate-pulse"></span>
    <span
      class="bg-gradient-to-r from-[#C4B5FD] to-[#8B5CF6] bg-clip-text text-transparent font-bold"
    >
      ✨ Hana AI Assistant
    </span>
    <span
      class="px-1.5 py-0.5 rounded text-[10px] font-mono bg-[#8B5CF6]/20 text-[#C4B5FD] border border-[#8B5CF6]/30"
    >
      CADS
    </span>
  </button>
{:else}
  <!-- Floating Smart Card Panel -->
  <div
    class="fixed bottom-4 right-4 z-50 w-96 max-w-[calc(100vw-2rem)] bg-[#121217] border border-[#272732] shadow-2xl shadow-black/80 rounded-2xl flex flex-col overflow-hidden font-sans transition-all {isMinimized
      ? 'h-14'
      : 'h-[540px]'}"
  >
    <!-- Card Header -->
    <div
      role="toolbar"
      tabindex="0"
      class="h-14 border-b border-[#272732] px-4 bg-[#18181F] flex items-center justify-between shrink-0 select-none cursor-pointer"
      onclick={() => (isMinimized = !isMinimized)}
      onkeydown={(e) => {
        if (e.key === "Enter" || e.key === " ") {
          isMinimized = !isMinimized;
        }
      }}
    >
      <div class="flex items-center gap-2.5">
        <div
          class="w-6 h-6 rounded-md bg-[#8B5CF6]/20 border border-[#8B5CF6]/40 flex items-center justify-center text-xs text-[#A78BFA]"
        >
          ✨
        </div>
        <div>
          <span class="font-bold text-white text-xs block leading-tight"
            >Hana Smart Copilot</span
          >
          <span class="text-[10px] text-emerald-400 font-mono block"
            >● Online (Project Context Injected)</span
          >
        </div>
      </div>

      <div class="flex items-center gap-1" onclick={(e) => e.stopPropagation()} role="group">
        <button
          onclick={() => (isMinimized = !isMinimized)}
          class="p-1 rounded text-neutral-400 hover:text-white hover:bg-neutral-800 text-xs cursor-pointer"
          title={isMinimized ? "Perluas" : "Perkecil"}
        >
          {isMinimized ? "◻" : "–"}
        </button>
        <button
          onclick={() => (isOpen = false)}
          class="p-1 rounded text-neutral-400 hover:text-rose-400 hover:bg-neutral-800 text-xs cursor-pointer"
          title="Tutup Panel"
        >
          ✕
        </button>
      </div>
    </div>

    {#if !isMinimized}
      <!-- Quick Prompt Chips -->
      <div
        class="p-2.5 bg-[#0E0E12] border-b border-[#272732] flex items-center gap-1.5 overflow-x-auto scrollbar-none shrink-0"
      >
        {#each quickPrompts as qp}
          <button
            onclick={() => handleSend(qp.prompt)}
            disabled={isGenerating}
            class="px-2.5 py-1 rounded-full bg-[#18181F] hover:bg-[#8B5CF6]/20 border border-[#272732] hover:border-[#8B5CF6]/40 text-[10px] text-neutral-300 hover:text-white whitespace-nowrap transition-colors cursor-pointer"
          >
            {qp.label}
          </button>
        {/each}
      </div>

      <!-- Messages Stream -->
      <div class="flex-1 p-3 overflow-y-auto space-y-3 min-h-0 bg-[#0A0A0C]">
        {#each messages as msg}
          <div
            class="flex flex-col {msg.role === 'user'
              ? 'items-end'
              : 'items-start'}"
          >
            <div
              class="max-w-[88%] p-3 rounded-xl text-xs leading-relaxed {msg.role ===
              'user'
                ? 'bg-[#8B5CF6] text-white rounded-br-none'
                : 'bg-[#18181F] border border-[#272732] text-neutral-200 rounded-bl-none shadow-xs'}"
            >
              <div class="whitespace-pre-wrap">{msg.content}</div>
              {#if msg.role === 'assistant' && msg.id !== 'msg_welcome'}
                <div
                  class="mt-2 pt-2 border-t border-neutral-700/50 flex items-center justify-end gap-2 text-[10px] text-neutral-400"
                >
                  <button
                    onclick={() => copyText(msg.content)}
                    class="hover:text-white flex items-center gap-1 cursor-pointer"
                  >
                    <span>📋 Salin</span>
                  </button>
                </div>
              {/if}
            </div>
          </div>
        {/each}
        {#if isGenerating}
          <div
            class="flex items-center gap-2 p-2 text-xs text-[#A78BFA] animate-pulse"
          >
            <span>⏳</span>
            <span>Hana sedang merancang respon...</span>
          </div>
        {/if}
      </div>

      <!-- Prompt Input Footer -->
      <div
        class="p-3 bg-[#121217] border-t border-[#272732] space-y-2 shrink-0"
      >
        <div class="relative">
          <textarea
            bind:value={promptText}
            onkeydown={handleKeyDown}
            placeholder="Tanyakan atau instruksikan AI (Cmd+Enter kirim)..."
            rows="2"
            class="w-full bg-[#18181F] border border-[#272732] rounded-xl p-2.5 text-xs text-white placeholder-neutral-500 focus:border-[#8B5CF6] focus:outline-none resize-none leading-relaxed"
          ></textarea>
        </div>
        <div class="flex items-center justify-between text-[10px]">
          <span class="text-neutral-500">Cmd+Enter untuk kirim</span>
          <button
            onclick={() => handleSend()}
            disabled={isGenerating || !promptText.trim()}
            class="px-3.5 py-1.5 rounded-lg bg-[#8B5CF6] hover:bg-[#7C3AED] disabled:opacity-50 text-white font-bold transition-all shadow-md shadow-[#8B5CF6]/30 flex items-center gap-1 cursor-pointer"
          >
            <span>Kirim</span>
            <span>➔</span>
          </button>
        </div>
      </div>
    {/if}
  </div>
{/if}
