<script lang="ts">
  import {
    listDocuments,
    createDocument,
    generateEbookOutline,
    exportEpub,
    exportHtml,
    exportMarkdown,
    type Document,
  } from "$lib/api";
  import ContentBuilder from "./ContentBuilder.svelte";

  interface Props {
    projectId: string;
  }

  let { projectId }: Props = $props();

  let chapters = $state<Document[]>([]);
  let activeChapter = $state<Document | null>(null);
  let bookTopic = $state("7 AI Agent Otomatis untuk Software Engineer");
  let chaptersCount = $state(7);
  let generatingOutline = $state(false);
  let generatedOutlineText = $state<string | null>(null);
  let exportStatus = $state<string | null>(null);

  $effect(() => {
    if (projectId) {
      loadChapters();
    }
  });

  async function loadChapters() {
    try {
      const docs = await listDocuments(projectId);
      chapters = docs.filter((d) => d.doc_type === "ebook");
      if (chapters.length > 0 && !activeChapter) {
        activeChapter = chapters[0];
      }
    } catch (e) {
      console.error(e);
    }
  }

  async function handleGenerateOutline() {
    generatingOutline = true;
    try {
      const res = await generateEbookOutline(projectId, bookTopic, chaptersCount);
      generatedOutlineText = res;
    } catch (e: any) {
      alert(`Gagal membuat outline: ${e}`);
    } finally {
      generatingOutline = false;
    }
  }

  async function handleCreateChapter(title: string, content?: string) {
    try {
      const doc = await createDocument({
        project_id: projectId,
        title,
        doc_type: "ebook",
        initial_content: content || "Mulai menulis isi bab di sini...",
      });
      await loadChapters();
      activeChapter = doc;
    } catch (e: any) {
      alert(`Gagal membuat bab: ${e}`);
    }
  }

  async function handleExportBook(format: "epub" | "html" | "md") {
    if (!activeChapter) return;
    exportStatus = "Mengekspor e-book...";
    try {
      let path = "";
      if (format === "epub") path = await exportEpub(activeChapter.id);
      if (format === "html") path = await exportHtml(activeChapter.id);
      if (format === "md") path = await exportMarkdown(activeChapter.id);
      exportStatus = `Berhasil diekspor: ${path}`;
      setTimeout(() => (exportStatus = null), 6000);
    } catch (e: any) {
      exportStatus = `Gagal ekspor: ${e}`;
    }
  }
</script>

<div class="flex-1 flex h-full bg-[#0A0A0C] overflow-hidden">
  <!-- Left Chapters Sidebar -->
  <div class="w-72 border-r border-[#272732] bg-[#121217] flex flex-col shrink-0">
    <!-- Header -->
    <div class="p-4 border-b border-[#272732] flex items-center justify-between">
      <div>
        <h3 class="font-bold text-white text-xs">E-Book Studio</h3>
        <span class="text-[10px] text-[#9CA3AF]">{chapters.length} Bab Terdaftar</span>
      </div>
      <button
        onclick={() => handleCreateChapter(`Bab ${chapters.length + 1}`)}
        class="p-1.5 bg-[#8B5CF6] hover:bg-[#7C3AED] text-white rounded-lg text-xs"
        title="Tambah Bab Baru"
      >
        + Bab
      </button>
    </div>

    <!-- Chapter List -->
    <div class="flex-1 overflow-y-auto p-2 space-y-1">
      {#each chapters as ch, i}
        <button
          onclick={() => (activeChapter = ch)}
          class="w-full text-left p-2.5 rounded-lg text-xs transition-all flex items-center justify-between {activeChapter?.id === ch.id ? 'bg-[#8B5CF6]/20 border border-[#8B5CF6]/50 text-white font-semibold' : 'text-[#9CA3AF] hover:bg-[#18181F] hover:text-white'}"
        >
          <span class="truncate">#{i + 1} {ch.title}</span>
          <span class="text-[10px] text-[#6B7280]">{ch.word_count} kata</span>
        </button>
      {/each}

      {#if chapters.length === 0}
        <div class="p-4 text-center text-xs text-[#6B7280]">
          Belum ada bab. Buat bab manual atau gunakan AI Outline Generator di sebelah kanan.
        </div>
      {/if}
    </div>

    <!-- Book Export Strip -->
    {#if activeChapter}
      <div class="p-3 border-t border-[#272732] bg-[#0A0A0C] space-y-2">
        <span class="text-[10px] font-bold text-[#6B7280] uppercase block">Ekspor E-Book:</span>
        <div class="grid grid-cols-3 gap-1.5 text-[11px]">
          <button onclick={() => handleExportBook("epub")} class="p-1.5 rounded bg-[#8B5CF6]/20 text-[#A78BFA] border border-[#8B5CF6]/40 hover:bg-[#8B5CF6]/30 font-bold">EPUB</button>
          <button onclick={() => handleExportBook("html")} class="p-1.5 rounded bg-[#18181F] text-[#D1D5DB] border border-[#272732] hover:bg-[#272732]">HTML</button>
          <button onclick={() => handleExportBook("md")} class="p-1.5 rounded bg-[#18181F] text-[#D1D5DB] border border-[#272732] hover:bg-[#272732]">MD</button>
        </div>
      </div>
    {/if}
  </div>

  <!-- Right Editor or Outline Generator Area -->
  <div class="flex-1 flex flex-col h-full overflow-hidden">
    {#if exportStatus}
      <div class="bg-[#18181F] border-b border-[#8B5CF6]/40 px-6 py-2 text-xs text-[#A78BFA]">
        {exportStatus}
      </div>
    {/if}

    {#if activeChapter}
      <!-- Visual Content Builder for Active Chapter -->
      <ContentBuilder activeDoc={activeChapter} {projectId} />
    {:else}
      <!-- AI Outline Generation View -->
      <div class="flex-1 overflow-y-auto p-8 max-w-3xl mx-auto space-y-6">
        <div class="bg-[#121217] border border-[#272732] rounded-xl p-6 space-y-4">
          <div class="flex items-center gap-2">
            <span class="text-xl">📚</span>
            <h3 class="font-bold text-white text-base">AI E-Book Outline Architect</h3>
          </div>
          <p class="text-xs text-[#9CA3AF]">
            Susun cetak biru e-book teknis end-to-end lengkap dengan judul bab dan sub-topik mendalam berdasarkan Project Bible.
          </p>

          <div>
            <label for="ebook-topic-input" class="block text-xs text-[#9CA3AF] mb-1">Topik & Judul Buku</label>
            <input
              id="ebook-topic-input"
              type="text"
              bind:value={bookTopic}
              class="w-full bg-[#18181F] border border-[#272732] rounded-lg px-3 py-2 text-xs text-white focus:border-[#8B5CF6] focus:outline-none"
            />
          </div>

          <div class="flex items-center justify-between">
            <div class="flex items-center gap-2">
              <label for="chapters-count-select" class="text-xs text-[#9CA3AF]">Target Bab:</label>
              <select
                id="chapters-count-select"
                bind:value={chaptersCount}
                class="bg-[#18181F] border border-[#272732] rounded px-2 py-1 text-xs text-white focus:outline-none"
              >
                <option value={5}>5 Bab</option>
                <option value={7}>7 Bab</option>
                <option value={10}>10 Bab</option>
                <option value={12}>12 Bab</option>
              </select>
            </div>

            <button
              onclick={handleGenerateOutline}
              disabled={generatingOutline}
              class="px-4 py-2 bg-[#8B5CF6] hover:bg-[#7C3AED] text-white rounded-lg text-xs font-semibold shadow-lg shadow-[#8B5CF6]/30 transition-all disabled:opacity-50"
            >
              {generatingOutline ? "Menyusun Outline..." : "Generate Cetak Biru"}
            </button>
          </div>
        </div>

        {#if generatedOutlineText}
          <div class="bg-[#121217] border border-[#272732] rounded-xl p-6 space-y-4">
            <div class="flex items-center justify-between">
              <h4 class="font-bold text-white text-xs">Hasil Outline AI:</h4>
              <button
                onclick={() => handleCreateChapter("Bab 1: Pendahuluan", generatedOutlineText || "")}
                class="px-3 py-1 bg-[#10B981] hover:bg-[#059669] text-white rounded text-xs font-semibold"
              >
                Jadikan Bab Pertama ➔
              </button>
            </div>
            <div class="font-mono text-xs text-[#D1D5DB] whitespace-pre-wrap leading-relaxed bg-[#0A0A0C] p-4 rounded-lg border border-[#272732]">
              {generatedOutlineText}
            </div>
          </div>
        {/if}
      </div>
    {/if}
  </div>
</div>
