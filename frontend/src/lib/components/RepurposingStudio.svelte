<script lang="ts">
  import { onMount } from "svelte";
  import {
    fetchYoutubeContent,
    createCampaign,
    getCampaign,
    listCampaigns,
    runRepurpose,
    updateAsset,
    setAssetStatus,
    queueCampaign,
    deleteCampaign,
    type ContentCampaign,
    type ContentAsset,
  } from "$lib/api";

  interface Props {
    projectId?: string;
  }

  let { projectId }: Props = $props();

  // Input Hub State
  let sourceType = $state<"markdown" | "raw_text" | "youtube_url">("markdown");
  let campaignTitle = $state("Peluncuran Arsitektur Sovereign Developer");
  let sourceText = $state(
    `# Arsitektur Sovereign Developer: Mandiri Tanpa Jebakan Cloud

Di era komputasi modern, ketergantungan berlebihan pada layanan cloud terpusat (vendor lock-in) membawa risiko serius: lonjakan biaya tak terduga, latensi jaringan, serta hilangnya kedaulatan data pengguna.

Filosofi Sovereign Developer mengusung 3 prinsip inti:
1. Local-First & Zero-Knowledge: Data milik pengguna, tersimpan di perangkat lokal dengan enkripsi client-side (Argon2id + SQLCipher).
2. Arsitektur Biner Tunggal (Single Binary): Meminimalkan runtime bloat dengan Rust dan antarmuka Svelte 5 desktop yang instan (0 ms boot).
3. Kedaulatan Finansial: Model lisensi perpetual atau utility credit terdesentralisasi, bukan langganan cloud yang memeras.

Ketika developer memiliki kedaulatan atas perkakasnya, produktivitas meningkat tanpa rasa cemas akan perubahan kebijakan vendor.`
  );
  let youtubeUrl = $state("https://youtu.be/dQw4w9WgXcQ");
  let fetchingYoutube = $state(false);
  let generating = $state(false);

  // Active Campaign & Channel State
  let currentCampaign = $state<ContentCampaign | null>(null);
  let recentCampaigns = $state<ContentCampaign[]>([]);
  let activeChannel = $state<"blog" | "shorts" | "carousel" | "tweets" | "newsletter">("blog");
  let editingContent = $state("");
  let savingAsset = $state(false);

  // Stats calculation
  let sourceWordCount = $derived(
    sourceText.trim() ? sourceText.trim().split(/\s+/).length : 0
  );

  let activeAsset = $derived.by(() => {
    if (!currentCampaign) return null;
    return currentCampaign.assets.find((a) => a.channel === activeChannel) || null;
  });

  // Watch activeAsset to sync editingContent
  $effect(() => {
    if (activeAsset) {
      editingContent = activeAsset.content;
    } else {
      editingContent = "";
    }
  });

  onMount(async () => {
    await refreshCampaigns();
  });

  async function refreshCampaigns() {
    try {
      recentCampaigns = await listCampaigns(projectId);
      if (recentCampaigns.length > 0 && !currentCampaign) {
        currentCampaign = recentCampaigns[0];
      }
    } catch (e) {
      console.warn("Gagal memuat kampanye:", e);
    }
  }

  async function handleFetchYoutube() {
    if (!youtubeUrl.trim()) return;
    fetchingYoutube = true;
    try {
      const content = await fetchYoutubeContent(youtubeUrl.trim());
      sourceText = content;
      campaignTitle = "Rangkuman Transkrip: " + youtubeUrl.trim();
    } catch (e: any) {
      alert(`Gagal menarik transkrip YouTube: ${e}`);
    } finally {
      fetchingYoutube = false;
    }
  }

  async function handleRunOmniRepurpose() {
    if (!sourceText.trim()) {
      alert("Masukkan teks sumber materi terlebih dahulu!");
      return;
    }

    generating = true;
    try {
      // 1. Create campaign draft
      const newCamp = await createCampaign({
        project_id: projectId,
        title: campaignTitle.trim() || "Kampanye Omni-Channel Baru",
        source_type: sourceType,
        source_content: sourceText,
        source_url: sourceType === "youtube_url" ? youtubeUrl.trim() : undefined,
      });

      // 2. Run generator engine for all 5 channels
      const completedCamp = await runRepurpose({
        campaign_id: newCamp.id,
      });

      currentCampaign = completedCamp;
      await refreshCampaigns();
    } catch (e: any) {
      alert(`Gagal menjalankan repurposing engine: ${e}`);
    } finally {
      generating = false;
    }
  }

  async function handleSaveCurrentAsset() {
    if (!activeAsset) return;
    savingAsset = true;
    try {
      await updateAsset(activeAsset.id, editingContent);
      if (currentCampaign) {
        currentCampaign = await getCampaign(currentCampaign.id);
      }
    } catch (e: any) {
      alert(`Gagal menyimpan perubahan: ${e}`);
    } finally {
      savingAsset = false;
    }
  }

  async function handleApproveAndQueueActiveAsset() {
    if (!activeAsset) return;
    try {
      await updateAsset(activeAsset.id, editingContent);
      await setAssetStatus(activeAsset.id, "queued");
      if (currentCampaign) {
        currentCampaign = await getCampaign(currentCampaign.id);
      }
    } catch (e: any) {
      alert(`Gagal mengirim ke antrean: ${e}`);
    }
  }

  async function handleQueueEntireCampaign() {
    if (!currentCampaign) return;
    try {
      currentCampaign = await queueCampaign(currentCampaign.id);
      await refreshCampaigns();
    } catch (e: any) {
      alert(`Gagal mengirim kampanye ke antrean: ${e}`);
    }
  }

  async function handleDeleteCampaign(id: string) {
    if (!confirm("Hapus seluruh aset dan draf kampanye ini?")) return;
    try {
      await deleteCampaign(id);
      if (currentCampaign?.id === id) {
        currentCampaign = null;
      }
      await refreshCampaigns();
    } catch (e: any) {
      alert(`Gagal menghapus kampanye: ${e}`);
    }
  }

  // Parse Tweets for character count validation
  let parsedTweets = $derived.by(() => {
    if (activeChannel !== "tweets" || !editingContent) return [];
    // Split by numbered tweets like "1/", "2/", or "1.", "2."
    const parts = editingContent
      .split(/\n(?=(?:\d+\/|\d+\.)\s*)/)
      .map((t) => t.trim())
      .filter((t) => t.length > 0);

    return parts.map((t, idx) => ({
      index: idx + 1,
      text: t,
      chars: t.length,
      isValid: t.length <= 280,
    }));
  });

  const channelsList = [
    { id: "blog", label: "Blog Teknis", icon: "📝" },
    { id: "shorts", label: "Viral Shorts (9:16)", icon: "⚡" },
    { id: "carousel", label: "Carousel IG (4:5)", icon: "📱" },
    { id: "tweets", label: "Utas X / Threads", icon: "🐦" },
    { id: "newsletter", label: "Newsletter Digest", icon: "✉️" },
  ] as const;
</script>

<div class="flex-1 flex flex-col h-full bg-[#0A0A0C] overflow-hidden select-none font-sans">
  <!-- Topbar Header -->
  <header class="h-12 border-b border-[#272732] px-6 flex items-center justify-between bg-[#121217] shrink-0">
    <div class="flex items-center gap-3">
      <div class="w-6 h-6 rounded-md bg-[#8B5CF6]/20 border border-[#8B5CF6]/40 flex items-center justify-center text-xs text-[#A78BFA]">
        🚀
      </div>
      <h2 class="font-bold text-white text-xs tracking-wide">
        Omni-Channel Content Repurposing Engine
      </h2>
      <span class="px-2 py-0.5 rounded text-[10px] font-mono bg-[#8B5CF6]/20 text-[#A78BFA] border border-[#8B5CF6]/30">
        1 Source ➔ 5 Formats
      </span>
    </div>

    <!-- Right Controls: Queue Entire Campaign -->
    <div class="flex items-center gap-2">
      {#if currentCampaign}
        <span class="text-[10px] px-2.5 py-0.5 rounded font-mono uppercase font-bold {currentCampaign.status === 'queued' ? 'bg-emerald-500/20 text-emerald-400 border border-emerald-500/30' : 'bg-neutral-800 text-neutral-400'}">
          Status: {currentCampaign.status}
        </span>
        <button
          onclick={handleQueueEntireCampaign}
          class="px-3 py-1.5 rounded-lg bg-emerald-600 hover:bg-emerald-500 text-white text-xs font-bold transition-all shadow-md shadow-emerald-600/30 flex items-center gap-1.5"
          title="Setujui dan antrekan seluruh 5 saluran sekaligus"
        >
          <span>✓</span>
          <span>Approve All to Queue</span>
        </button>
      {/if}
    </div>
  </header>

  <!-- Two-Column Workspace: Left Input Hub (40%) & Right Multi-Channel Output Hub (60%) -->
  <div class="flex-1 flex overflow-hidden min-h-0">
    <!-- Left Column: Input Hub -->
    <div class="w-2/5 border-r border-[#272732] bg-[#0E0E12] flex flex-col justify-between overflow-y-auto p-5 space-y-4 shrink-0">
      <div class="space-y-4">
        <div class="flex items-center justify-between">
          <span class="text-[10px] uppercase font-bold text-[#6B7280] tracking-wider">
            1. Input Hub (Multi-Sumber)
          </span>
          <span class="text-[10px] text-neutral-400 font-mono">
            {sourceWordCount} kata
          </span>
        </div>

        <!-- Campaign Title -->
        <div>
          <label for="campaign-title-input" class="block text-[11px] font-medium text-neutral-300 mb-1">
            Judul Kampanye:
          </label>
          <input
            id="campaign-title-input"
            type="text"
            bind:value={campaignTitle}
            placeholder="Contoh: Rilis Framework Sovereign Developer..."
            class="w-full bg-[#18181F] border border-[#272732] rounded-lg px-3 py-2 text-xs text-white focus:border-[#8B5CF6] focus:outline-none"
          />
        </div>

        <!-- Source Type Selector Tabs -->
        <div class="grid grid-cols-3 p-1 rounded-lg bg-[#18181F] border border-[#272732] text-xs">
          <button
            onclick={() => (sourceType = "markdown")}
            class="py-1.5 rounded-md font-medium transition-colors {sourceType === 'markdown' ? 'bg-[#8B5CF6] text-white shadow-xs' : 'text-neutral-400 hover:text-white'}"
          >
            Markdown
          </button>
          <button
            onclick={() => (sourceType = "raw_text")}
            class="py-1.5 rounded-md font-medium transition-colors {sourceType === 'raw_text' ? 'bg-[#8B5CF6] text-white shadow-xs' : 'text-neutral-400 hover:text-white'}"
          >
            Raw Text
          </button>
          <button
            onclick={() => (sourceType = "youtube_url")}
            class="py-1.5 rounded-md font-medium transition-colors {sourceType === 'youtube_url' ? 'bg-[#8B5CF6] text-white shadow-xs' : 'text-neutral-400 hover:text-white'}"
          >
            YouTube
          </button>
        </div>

        <!-- YouTube URL Input Hub -->
        {#if sourceType === "youtube_url"}
          <div class="p-3 rounded-lg bg-[#18181F] border border-[#272732] space-y-2">
            <span class="text-[10px] uppercase font-bold text-[#8B5CF6]">Ekstraksi URL Video YouTube</span>
            <div class="flex items-center gap-2">
              <input
                type="text"
                bind:value={youtubeUrl}
                placeholder="https://youtu.be/..."
                class="flex-1 bg-[#121217] border border-[#272732] rounded px-2.5 py-1.5 text-xs text-white focus:border-[#8B5CF6] focus:outline-none"
              />
              <button
                onclick={handleFetchYoutube}
                disabled={fetchingYoutube || !youtubeUrl.trim()}
                class="px-3 py-1.5 bg-[#8B5CF6] hover:bg-[#7C3AED] disabled:opacity-50 text-white text-xs font-semibold rounded transition-colors shrink-0"
              >
                {fetchingYoutube ? "Menarik..." : "Tarik Transkrip"}
              </button>
            </div>
            <p class="text-[10px] text-neutral-400">
              Otomatis membaca judul, kreator, dan transkrip teks video untuk diolah ke seluruh kanal.
            </p>
          </div>
        {/if}

        <!-- Source Content Area -->
        <div>
          <label for="source-text-input" class="block text-[11px] font-medium text-neutral-300 mb-1">
            Isi Naskah / Transkrip Sumber Materi:
          </label>
          <textarea
            id="source-text-input"
            bind:value={sourceText}
            rows="12"
            placeholder="Ketik atau tempel teks naskah / artikel utama di sini..."
            class="w-full bg-[#18181F] border border-[#272732] rounded-lg p-3 text-xs text-white font-mono leading-relaxed focus:border-[#8B5CF6] focus:outline-none resize-none"
          ></textarea>
        </div>

        <!-- Repurpose Trigger Button -->
        <button
          onclick={handleRunOmniRepurpose}
          disabled={generating || !sourceText.trim()}
          class="w-full py-3 rounded-xl bg-gradient-to-r from-[#8B5CF6] to-[#6D28D9] hover:from-[#7C3AED] hover:to-[#5B21B6] text-white text-xs font-bold tracking-wide shadow-lg shadow-[#8B5CF6]/30 transition-all disabled:opacity-50 flex items-center justify-center gap-2"
        >
          {#if generating}
            <span class="animate-spin text-base">⏳</span>
            <span>Menghasilkan 5 Kanal Sekaligus...</span>
          {:else}
            <span>⚡</span>
            <span>Eksekusi Omni-Channel Repurposing</span>
          {/if}
        </button>
      </div>

      <!-- Recent Campaigns Drawer List -->
      <div class="pt-4 border-t border-[#272732] space-y-2">
        <span class="text-[10px] uppercase font-bold text-[#6B7280] tracking-wider block">
          Riwayat Kampanye Tersimpan:
        </span>
        <div class="space-y-1.5 max-h-40 overflow-y-auto scrollbar-none">
          {#each recentCampaigns as c}
            <div
              class="w-full p-2 rounded-lg text-left transition-colors flex items-center justify-between text-xs {currentCampaign?.id === c.id ? 'bg-[#18181F] border border-[#8B5CF6]/40 text-white' : 'hover:bg-[#18181F]/60 text-neutral-400'}"
            >
              <button
                onclick={() => (currentCampaign = c)}
                class="flex-1 text-left truncate mr-2"
                title={c.title}
              >
                <span class="font-medium truncate block">{c.title}</span>
                <span class="text-[10px] text-neutral-500 font-mono">
                  {c.assets.length} aset · {c.status}
                </span>
              </button>
              <button
                onclick={() => handleDeleteCampaign(c.id)}
                class="text-neutral-500 hover:text-red-400 p-1 transition-colors"
                title="Hapus Kampanye"
              >
                ✕
              </button>
            </div>
          {/each}
        </div>
      </div>
    </div>

    <!-- Right Column: Multi-Channel Output Preview & Editor Hub -->
    <div class="flex-1 flex flex-col bg-[#0A0A0C] overflow-hidden min-h-0">
      <!-- Channel Switcher Tabs -->
      <div class="h-11 border-b border-[#272732] px-4 bg-[#121217] flex items-center justify-between shrink-0">
        <div class="flex items-center gap-1 overflow-x-auto scrollbar-none">
          {#each channelsList as ch}
            <button
              onclick={() => (activeChannel = ch.id as any)}
              class="px-3 py-1.5 rounded-lg text-xs font-semibold transition-all flex items-center gap-1.5 {activeChannel === ch.id ? 'bg-[#8B5CF6] text-white shadow-xs' : 'text-neutral-400 hover:text-white hover:bg-[#18181F]'}"
            >
              <span>{ch.icon}</span>
              <span>{ch.label}</span>
              {#if currentCampaign?.assets.find((a) => a.channel === ch.id)}
                <span class="w-1.5 h-1.5 rounded-full bg-emerald-400"></span>
              {/if}
            </button>
          {/each}
        </div>

        <!-- Channel Actions -->
        {#if activeAsset}
          <div class="flex items-center gap-2">
            <span class="text-[10px] px-2 py-0.5 rounded font-mono uppercase {activeAsset.status === 'queued' ? 'bg-emerald-500/20 text-emerald-400' : 'bg-neutral-800 text-neutral-400'}">
              {activeAsset.status}
            </span>
            <button
              onclick={handleSaveCurrentAsset}
              disabled={savingAsset}
              class="px-2.5 py-1 bg-[#18181F] hover:bg-[#272732] border border-[#272732] text-xs text-neutral-300 hover:text-white rounded transition-colors"
            >
              {savingAsset ? "Menyimpan..." : "Simpan Draf"}
            </button>
            <button
              onclick={handleApproveAndQueueActiveAsset}
              class="px-3 py-1 bg-emerald-600 hover:bg-emerald-500 text-white text-xs font-bold rounded shadow-sm transition-all"
            >
              Approve & Queue ✓
            </button>
          </div>
        {/if}
      </div>

      <!-- Channel Main Viewport -->
      <div class="flex-1 flex flex-col p-6 overflow-hidden min-h-0 space-y-4">
        {#if activeAsset}
          <!-- Metadata Bar -->
          <div class="p-3 bg-[#121217] border border-[#272732] rounded-xl flex items-center justify-between text-xs shrink-0">
            <div>
              <span class="font-bold text-white block">{activeAsset.title}</span>
              <span class="text-[10px] text-neutral-400 font-mono">
                Metadata: {activeAsset.meta}
              </span>
            </div>
            <button
              onclick={() => {
                navigator.clipboard.writeText(editingContent);
                alert("Konten berhasil disalin ke clipboard!");
              }}
              class="px-3 py-1 bg-[#18181F] hover:bg-[#272732] border border-[#272732] text-white text-xs font-medium rounded transition-colors"
            >
              Salin Konten
            </button>
          </div>

          <!-- Special Channel View for Tweetstorm Validation -->
          {#if activeChannel === "tweets" && parsedTweets.length > 0}
            <div class="grid grid-cols-2 gap-4 flex-1 min-h-0">
              <!-- Inline Raw Editor -->
              <div class="flex flex-col h-full bg-[#121217] border border-[#272732] rounded-xl p-3">
                <span class="text-[10px] uppercase font-bold text-neutral-400 mb-2">Editor Mentah (CodeMirror):</span>
                <textarea
                  bind:value={editingContent}
                  class="flex-1 w-full bg-[#0A0A0C] border border-[#272732] rounded-lg p-3 text-xs text-white font-mono leading-relaxed focus:border-[#8B5CF6] focus:outline-none resize-none"
                ></textarea>
              </div>

              <!-- Validator Live Cards -->
              <div class="flex flex-col h-full bg-[#121217] border border-[#272732] rounded-xl p-3 overflow-y-auto space-y-2.5">
                <span class="text-[10px] uppercase font-bold text-neutral-400 mb-1">Preview Cuitan & Validator 280 Chars:</span>
                {#each parsedTweets as tw}
                  <div class="p-3 rounded-lg bg-[#0A0A0C] border {tw.isValid ? 'border-[#272732]' : 'border-red-500/50 bg-red-950/10'} space-y-1.5">
                    <div class="flex items-center justify-between text-[10px]">
                      <span class="font-bold text-cyan-400">Tweet {tw.index}</span>
                      <span class="font-mono font-bold {tw.isValid ? 'text-emerald-400' : 'text-red-400'}">
                        {tw.chars}/280 {tw.isValid ? '✓' : '⚠️ Melebihi Batas'}
                      </span>
                    </div>
                    <p class="text-xs text-neutral-300 leading-relaxed font-sans">{tw.text}</p>
                  </div>
                {/each}
              </div>
            </div>
          {:else}
            <!-- Standard Inline Editor View -->
            <div class="flex-1 flex flex-col bg-[#121217] border border-[#272732] rounded-xl p-4 min-h-0">
              <span class="text-[10px] uppercase font-bold text-neutral-400 mb-2">Editor Inline Langsung (Markdown & Format):</span>
              <textarea
                bind:value={editingContent}
                class="flex-1 w-full bg-[#0A0A0C] border border-[#272732] rounded-lg p-4 text-xs text-white font-mono leading-relaxed focus:border-[#8B5CF6] focus:outline-none resize-none"
              ></textarea>
            </div>
          {/if}
        {:else}
          <!-- Empty State when no asset generated yet -->
          <div class="flex-1 flex flex-col items-center justify-center p-12 text-center text-neutral-500 space-y-3">
            <div class="w-16 h-16 rounded-2xl bg-[#18181F] border border-[#272732] flex items-center justify-center text-2xl">
              📦
            </div>
            <h3 class="font-bold text-neutral-300 text-sm">Belum Ada Aset Tergenerasi</h3>
            <p class="text-xs max-w-md text-neutral-500 leading-relaxed">
              Pilih atau masukkan materi sumber di panel kiri, lalu klik "Eksekusi Omni-Channel Repurposing" untuk memproduksi 5 kanal konten secara otomatis.
            </p>
          </div>
        {/if}
      </div>
    </div>
  </div>
</div>
