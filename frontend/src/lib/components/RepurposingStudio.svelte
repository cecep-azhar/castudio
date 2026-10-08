<script lang="ts">
  import { generateAi } from "$lib/api";

  interface Props {
    projectId?: string;
  }

  let { projectId }: Props = $props();

  let sourceText = $state(
    "CAStudio adalah studio konten berbasis AI untuk desktop dan mobile yang memungkinkan satu Project (Project Bible) menghasilkan banyak output tanpa kehilangan konteks suara."
  );
  let channel = $state<"all" | "threads" | "email" | "slides">("all");
  let loading = $state(false);
  let resultOutput = $state<string | null>(null);

  async function handleRepurpose() {
    loading = true;
    resultOutput = null;
    try {
      let prompt = "";
      if (channel === "all") {
        prompt = `Repurpose materi berikut menjadi 3 format sekaligus:
1) THREAD TWITTER/X (5 cuitan bernomor)
2) EMAIL PROMOSI (Subject + Hook + Benefit + CTA)
3) OUTLINE SLIDE PRESENTASI (5 slide poin ringkas)

Materi Sumber:
${sourceText}

Gunakan gaya bahasa Indonesia modern, lugas, tajam.`;
      } else if (channel === "threads") {
        prompt = `Ubah teks berikut menjadi utas Twitter/Threads 5 cuitan berbobot teknis:\n\n${sourceText}`;
      } else if (channel === "email") {
        prompt = `Ubah teks berikut menjadi email newsletter promosi dengan subjek menarik dan CTA kuat:\n\n${sourceText}`;
      } else {
        prompt = `Ubah teks berikut menjadi 5 slide presentasi ringkas dengan catatan pembicara:\n\n${sourceText}`;
      }

      const res = await generateAi({
        project_id: projectId,
        user_prompt: prompt,
      });

      resultOutput = res.text;
    } catch (e: any) {
      alert(`Gagal me-repurpose konten: ${e}`);
    } finally {
      loading = false;
    }
  }
</script>

<div class="flex-1 flex flex-col h-full bg-[#0A0A0C] overflow-hidden">
  <!-- Topbar -->
  <div class="h-12 border-b border-[#272732] px-6 flex items-center justify-between bg-[#121217] shrink-0">
    <div class="flex items-center gap-2">
      <span class="text-base">🚀</span>
      <h2 class="font-bold text-white text-xs">Repurposing Studio (Launch Pack)</h2>
    </div>
    <span class="text-xs text-[#9CA3AF]">1 Sumber Kebenaran ➔ Berbagai Format</span>
  </div>

  <!-- Main Body -->
  <div class="flex-1 overflow-y-auto p-6 md:p-8 max-w-4xl mx-auto w-full space-y-6">
    <div class="bg-[#121217] border border-[#272732] rounded-xl p-5 space-y-4">
      <h3 class="font-bold text-white text-xs">Teks Sumber Materi (Bab / Artikel / Catatan)</h3>
      <textarea
        bind:value={sourceText}
        rows="5"
        class="w-full bg-[#18181F] border border-[#272732] rounded-lg p-3 text-xs text-white focus:border-[#8B5CF6] focus:outline-none resize-none leading-relaxed"
      ></textarea>

      <div class="flex items-center justify-between">
        <div class="flex items-center gap-2">
          <label for="channel-select" class="text-xs text-[#9CA3AF]">Target Distribusi:</label>
          <select
            id="channel-select"
            bind:value={channel}
            class="bg-[#18181F] border border-[#272732] rounded px-3 py-1.5 text-xs text-white focus:outline-none"
          >
            <option value="all">Semua Saluran (Launch Pack Lengkap)</option>
            <option value="threads">Twitter / X Thread</option>
            <option value="email">Email Sequence</option>
            <option value="slides">Slide Presentasi</option>
          </select>
        </div>

        <button
          onclick={handleRepurpose}
          disabled={loading}
          class="px-5 py-2 rounded-lg bg-[#8B5CF6] hover:bg-[#7C3AED] text-white text-xs font-semibold shadow-lg shadow-[#8B5CF6]/30 transition-all disabled:opacity-50"
        >
          {loading ? "Menurunkan Materi..." : "Turunkan Format Otomatis"}
        </button>
      </div>
    </div>

    {#if resultOutput}
      <div class="bg-[#121217] border border-[#272732] rounded-xl p-6 space-y-3">
        <div class="flex items-center justify-between">
          <h4 class="font-bold text-white text-xs">Hasil Turunan Materi:</h4>
          <button
            onclick={() => {
              navigator.clipboard.writeText(resultOutput || "");
              alert("Hasil tersalin!");
            }}
            class="px-3 py-1 bg-[#10B981] hover:bg-[#059669] text-white rounded text-xs font-semibold"
          >
            Salin Semua
          </button>
        </div>
        <div class="font-mono text-xs text-[#D1D5DB] whitespace-pre-wrap leading-relaxed bg-[#0A0A0C] p-4 rounded-lg border border-[#272732]">
          {resultOutput}
        </div>
      </div>
    {/if}
  </div>
</div>
