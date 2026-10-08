<script lang="ts">
  import { generatePitchdeck, generateAi } from "$lib/api";

  interface Props {
    projectId?: string;
  }

  let { projectId }: Props = $props();

  let mode = $state<"pitchdeck" | "spk">("pitchdeck");
  let companyName = $state("Fathforce / CAStudio");
  let problemSolution = $state(
    "Masalah: Kreator & developer kehilangan waktu membuat materi pemasaran dari banyak alat terpisah. Solusi: CAStudio menyatukan pembuatan e-book, carousel IG, slide pitchdeck, dan kalender promosi dalam 1 canvas bertenaga AI lokal."
  );
  let loading = $state(false);
  let resultOutput = $state<string | null>(null);

  // SPK inputs
  let clientName = $state("PT Klien Mitra Solusi");
  let projectScope = $state("Pengembangan sistem POS & Webstore monorepo Go, migrasi database, dan integrasi payment gateway.");
  let projectBudget = $state("Rp 35.000.000 (Termin: 40% DP, 30% UAT, 30% Go-Live)");

  async function handleGenerateDeck() {
    loading = true;
    resultOutput = null;
    try {
      const res = await generatePitchdeck(projectId, companyName, problemSolution);
      resultOutput = res;
    } catch (e: any) {
      alert(`Gagal generate pitchdeck: ${e}`);
    } finally {
      loading = false;
    }
  }

  async function handleGenerateSpk() {
    loading = true;
    resultOutput = null;
    try {
      const prompt = `Buatkan draf Surat Perjanjian Kerja (SPK) / Kontrak Jasa Profesional resmi:
Pihak Pertama (Penyedia): Cecep Saeful Azhar Hidayat (Fathforce)
Pihak Kedua (Klien): ${clientName}
Ruang Lingkup: ${projectScope}
Nilai Kontrak: ${projectBudget}

Sertakan pasal kewajiban, hak cipta (source code menjadi milik klien setelah lunas), garansi perbaikan bug 30 hari, dan klausul kerahasiaan NDA.
Bahasa: Bahasa Indonesia formal hukum legal.`;

      const res = await generateAi({
        project_id: projectId,
        user_prompt: prompt,
      });

      resultOutput = res.text;
    } catch (e: any) {
      alert(`Gagal generate SPK: ${e}`);
    } finally {
      loading = false;
    }
  }
</script>

<div class="flex-1 flex flex-col h-full bg-[#0A0A0C] overflow-hidden">
  <!-- Topbar -->
  <div class="h-12 border-b border-[#272732] px-6 flex items-center justify-between bg-[#121217] shrink-0">
    <div class="flex items-center gap-1 bg-[#18181F] p-0.5 rounded-lg border border-[#272732] text-xs">
      <button
        onclick={() => (mode = "pitchdeck")}
        class="px-3 py-1 rounded-md font-medium transition-all {mode === 'pitchdeck' ? 'bg-[#8B5CF6] text-white shadow-sm' : 'text-[#9CA3AF] hover:text-white'}"
      >
        📊 Pitchdeck 10-Slide
      </button>
      <button
        onclick={() => (mode = "spk")}
        class="px-3 py-1 rounded-md font-medium transition-all {mode === 'spk' ? 'bg-[#8B5CF6] text-white shadow-sm' : 'text-[#9CA3AF] hover:text-white'}"
      >
        📄 SPK & Proposal Closing
      </button>
    </div>

    <span class="text-xs text-[#9CA3AF]">Pitchdeck & Closing Suite</span>
  </div>

  <!-- Body -->
  <div class="flex-1 overflow-y-auto p-6 md:p-8 max-w-4xl mx-auto w-full space-y-6">
    {#if mode === "pitchdeck"}
      <div class="bg-[#121217] border border-[#272732] rounded-xl p-5 space-y-4">
        <h3 class="font-bold text-white text-xs">Arsitektur Pitchdeck Investor & Klien</h3>
        <div>
          <label for="company-name-input" class="block text-xs text-[#9CA3AF] mb-1">Nama Perusahaan / Produk</label>
          <input
            id="company-name-input"
            type="text"
            bind:value={companyName}
            class="w-full bg-[#18181F] border border-[#272732] rounded-lg px-3 py-2 text-xs text-white focus:border-[#8B5CF6] focus:outline-none"
          />
        </div>

        <div>
          <label for="problem-solution-input" class="block text-xs text-[#9CA3AF] mb-1">Konteks Masalah & Solusi</label>
          <textarea
            id="problem-solution-input"
            bind:value={problemSolution}
            rows="4"
            class="w-full bg-[#18181F] border border-[#272732] rounded-lg p-3 text-xs text-white focus:border-[#8B5CF6] focus:outline-none resize-none leading-relaxed"
          ></textarea>
        </div>

        <div class="text-right">
          <button
            onclick={handleGenerateDeck}
            disabled={loading}
            class="px-5 py-2 rounded-lg bg-[#8B5CF6] hover:bg-[#7C3AED] text-white text-xs font-semibold shadow-lg shadow-[#8B5CF6]/30 transition-all disabled:opacity-50"
          >
            {loading ? "Menyusun Deck..." : "Generate 10-Slide Pitchdeck"}
          </button>
        </div>
      </div>
    {:else}
      <div class="bg-[#121217] border border-[#272732] rounded-xl p-5 space-y-4">
        <h3 class="font-bold text-white text-xs">Draf Kontrak SPK & Dokumen Penawaran</h3>
        <div>
          <label for="client-name-input" class="block text-xs text-[#9CA3AF] mb-1">Nama Perusahaan Klien</label>
          <input
            id="client-name-input"
            type="text"
            bind:value={clientName}
            class="w-full bg-[#18181F] border border-[#272732] rounded-lg px-3 py-2 text-xs text-white focus:border-[#8B5CF6] focus:outline-none"
          />
        </div>

        <div>
          <label for="project-scope-input" class="block text-xs text-[#9CA3AF] mb-1">Ruang Lingkup Proyek</label>
          <textarea
            id="project-scope-input"
            bind:value={projectScope}
            rows="3"
            class="w-full bg-[#18181F] border border-[#272732] rounded-lg p-3 text-xs text-white focus:border-[#8B5CF6] focus:outline-none resize-none leading-relaxed"
          ></textarea>
        </div>

        <div>
          <label for="project-budget-input" class="block text-xs text-[#9CA3AF] mb-1">Nilai Kontrak & Termin Pembayaran</label>
          <input
            id="project-budget-input"
            type="text"
            bind:value={projectBudget}
            class="w-full bg-[#18181F] border border-[#272732] rounded-lg px-3 py-2 text-xs text-white focus:border-[#8B5CF6] focus:outline-none"
          />
        </div>

        <div class="text-right">
          <button
            onclick={handleGenerateSpk}
            disabled={loading}
            class="px-5 py-2 rounded-lg bg-[#8B5CF6] hover:bg-[#7C3AED] text-white text-xs font-semibold shadow-lg shadow-[#8B5CF6]/30 transition-all disabled:opacity-50"
          >
            {loading ? "Menyusun SPK..." : "Generate Draf Kontrak SPK"}
          </button>
        </div>
      </div>
    {/if}

    {#if resultOutput}
      <div class="bg-[#121217] border border-[#272732] rounded-xl p-6 space-y-3">
        <div class="flex items-center justify-between">
          <h4 class="font-bold text-white text-xs">Hasil Dokumen:</h4>
          <button
            onclick={() => {
              navigator.clipboard.writeText(resultOutput || "");
              alert("Hasil tersalin!");
            }}
            class="px-3 py-1 bg-[#10B981] hover:bg-[#059669] text-white rounded text-xs font-semibold"
          >
            Salin ke Clipboard
          </button>
        </div>
        <div class="font-mono text-xs text-[#D1D5DB] whitespace-pre-wrap leading-relaxed bg-[#0A0A0C] p-4 rounded-lg border border-[#272732]">
          {resultOutput}
        </div>
      </div>
    {/if}
  </div>
</div>
