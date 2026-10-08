<script lang="ts">
  import {
    getBlocks,
    saveBlocks,
    generateAi,
    exportMarkdown,
    exportHtml,
    exportEpub,
    type Block,
    type Document,
  } from "$lib/api";

  interface Props {
    activeDoc: Document;
    projectId?: string;
  }

  let { activeDoc, projectId }: Props = $props();

  let blocks = $state<Block[]>([]);
  let editorMode = $state<"visual" | "markdown">("visual");
  let rawMarkdown = $state("");
  let saving = $state(false);
  let aiLoading = $state(false);
  let exportMessage = $state<string | null>(null);

  // Load blocks when activeDoc changes
  $effect(() => {
    if (activeDoc?.id) {
      loadBlocks();
    }
  });

  async function loadBlocks() {
    try {
      const b = await getBlocks(activeDoc.id);
      if (b.length === 0) {
        // default empty block
        blocks = [
          {
            id: "",
            document_id: activeDoc.id,
            parent_id: null,
            block_type: "heading",
            position: 0,
            content: activeDoc.title,
            meta: "1",
            updated_at: Date.now(),
          },
          {
            id: "",
            document_id: activeDoc.id,
            parent_id: null,
            block_type: "paragraph",
            position: 1,
            content: "Mulai menulis draf konten atau gunakan generator AI...",
            meta: "{}",
            updated_at: Date.now(),
          },
        ];
        await handleSaveBlocks();
      } else {
        blocks = b;
      }
      updateRawMarkdownFromBlocks();
    } catch (e) {
      console.error(e);
    }
  }

  function updateRawMarkdownFromBlocks() {
    rawMarkdown = blocks
      .map((b) => {
        if (b.block_type === "heading") {
          const lvl = parseInt(b.meta) || 2;
          return `${"#".repeat(lvl)} ${b.content}`;
        }
        if (b.block_type === "quote") return `> ${b.content}`;
        if (b.block_type === "callout") return `> **INFO:** ${b.content}`;
        if (b.block_type === "stat") return `**[METRIC: ${b.content}]**`;
        if (b.block_type === "slide_break") return `---`;
        if (b.block_type === "code") return `\`\`\`\n${b.content}\n\`\`\``;
        return b.content;
      })
      .join("\n\n");
  }

  async function handleSaveBlocks() {
    saving = true;
    try {
      blocks = await saveBlocks(activeDoc.id, blocks);
      updateRawMarkdownFromBlocks();
    } catch (e) {
      alert(`Gagal menyimpan: ${e}`);
    } finally {
      saving = false;
    }
  }

  function addBlock(type: string, afterIndex: number) {
    const newBlock: Block = {
      id: "",
      document_id: activeDoc.id,
      parent_id: null,
      block_type: type,
      position: afterIndex + 1,
      content: type === "heading" ? "Heading Baru" : type === "slide_break" ? "---" : "Tulis teks di sini...",
      meta: type === "heading" ? "2" : "{}",
      updated_at: Date.now(),
    };
    blocks.splice(afterIndex + 1, 0, newBlock);
    handleSaveBlocks();
  }

  function removeBlock(index: number) {
    if (blocks.length <= 1) return;
    blocks.splice(index, 1);
    handleSaveBlocks();
  }

  function moveBlock(index: number, direction: "up" | "down") {
    const target = direction === "up" ? index - 1 : index + 1;
    if (target < 0 || target >= blocks.length) return;
    const temp = blocks[index];
    blocks[index] = blocks[target];
    blocks[target] = temp;
    handleSaveBlocks();
  }

  async function handleAiAssist(blockIndex: number, action: "expand" | "polish" | "summarize") {
    aiLoading = true;
    try {
      const b = blocks[blockIndex];
      let prompt = "";
      if (action === "expand") prompt = `Kembangkan paragraf berikut secara mendalam, sertakan contoh teknis:\n\n${b.content}`;
      if (action === "polish") prompt = `Perbaiki gaya bahasa paragraf berikut agar lebih tajam, percaya diri, dan profesional:\n\n${b.content}`;
      if (action === "summarize") prompt = `Ringkas paragraf berikut menjadi 1-2 kalimat padat bernas:\n\n${b.content}`;

      const res = await generateAi({
        project_id: projectId,
        document_id: activeDoc.id,
        user_prompt: prompt,
      });

      b.content = res.text.trim();
      await handleSaveBlocks();
    } catch (e) {
      alert(`AI Assist gagal: ${e}`);
    } finally {
      aiLoading = false;
    }
  }

  async function handleExport(type: "md" | "html" | "epub") {
    exportMessage = "Mengekspor...";
    try {
      let path = "";
      if (type === "md") path = await exportMarkdown(activeDoc.id);
      if (type === "html") path = await exportHtml(activeDoc.id);
      if (type === "epub") path = await exportEpub(activeDoc.id);
      exportMessage = `Berhasil diekspor: ${path}`;
      setTimeout(() => (exportMessage = null), 6000);
    } catch (e: any) {
      exportMessage = `Gagal ekspor: ${e}`;
    }
  }

  // Count unfilled placeholders
  let placeholders = $derived(
    blocks.flatMap((b) => {
      const matches = b.content.match(/\[ISI:\s*([^\]]+)\]/g);
      return matches || [];
    })
  );
</script>

<div class="flex-1 flex flex-col h-full bg-[#0A0A0C] overflow-hidden">
  <!-- Content Builder Topbar -->
  <div class="h-12 border-b border-[#272732] px-6 flex items-center justify-between bg-[#121217] shrink-0">
    <div class="flex items-center gap-3">
      <span class="text-xs uppercase tracking-wider px-2 py-0.5 rounded bg-[#8B5CF6]/20 text-[#A78BFA] font-semibold border border-[#8B5CF6]/30">
        {activeDoc.doc_type}
      </span>
      <h2 class="font-bold text-white text-sm truncate max-w-md">{activeDoc.title}</h2>
      {#if placeholders.length > 0}
        <span class="text-[11px] px-2 py-0.5 rounded bg-[#EF4444]/20 text-[#F87171] font-medium border border-[#EF4444]/30">
          ⚠️ {placeholders.length} Placeholder [ISI: ...]
        </span>
      {/if}
    </div>

    <!-- Right Controls & Mode Toggle -->
    <div class="flex items-center gap-2 text-xs">
      <!-- Mode Toggle -->
      <div class="flex items-center bg-[#18181F] border border-[#272732] rounded-lg p-0.5">
        <button
          onclick={() => (editorMode = "visual")}
          class="px-3 py-1 rounded-md font-medium transition-all {editorMode === 'visual' ? 'bg-[#8B5CF6] text-white shadow-sm' : 'text-[#9CA3AF] hover:text-white'}"
        >
          Visual Canvas
        </button>
        <button
          onclick={() => {
            updateRawMarkdownFromBlocks();
            editorMode = "markdown";
          }}
          class="px-3 py-1 rounded-md font-medium transition-all {editorMode === 'markdown' ? 'bg-[#8B5CF6] text-white shadow-sm' : 'text-[#9CA3AF] hover:text-white'}"
        >
          Raw Markdown
        </button>
      </div>

      <!-- Export Dropdown -->
      <button
        onclick={() => handleExport("md")}
        title="Ekspor ke Markdown"
        class="px-2.5 py-1.5 rounded-lg border border-[#374151] hover:bg-[#1E1E28] text-[#D1D5DB] transition-colors"
      >
        MD
      </button>
      <button
        onclick={() => handleExport("html")}
        title="Ekspor ke HTML"
        class="px-2.5 py-1.5 rounded-lg border border-[#374151] hover:bg-[#1E1E28] text-[#D1D5DB] transition-colors"
      >
        HTML
      </button>
      <button
        onclick={() => handleExport("epub")}
        title="Ekspor ke EPUB E-Book"
        class="px-2.5 py-1.5 rounded-lg border border-[#8B5CF6]/50 bg-[#8B5CF6]/20 hover:bg-[#8B5CF6]/30 text-[#C4B5FD] font-semibold transition-colors"
      >
        EPUB
      </button>
    </div>
  </div>

  <!-- Export Feedback Toast -->
  {#if exportMessage}
    <div class="bg-[#18181F] border-b border-[#8B5CF6]/40 px-6 py-2 text-xs text-[#A78BFA] flex items-center justify-between">
      <span>{exportMessage}</span>
      <button onclick={() => (exportMessage = null)} class="text-white hover:text-red-400">✕</button>
    </div>
  {/if}

  <!-- Main Canvas / Editor Area -->
  <div class="flex-1 overflow-y-auto p-6 md:p-12 max-w-4xl mx-auto w-full">
    {#if editorMode === "visual"}
      <div class="space-y-4">
        {#each blocks as block, index}
          <div class="group relative bg-[#121217]/60 hover:bg-[#121217] border border-[#272732] hover:border-[#8B5CF6]/40 rounded-xl p-4 transition-all shadow-sm">
            <!-- Block Action Controls (Hover Toolbar) -->
            <div class="absolute -top-3 right-4 hidden group-hover:flex items-center gap-1 bg-[#18181F] border border-[#374151] rounded-lg px-2 py-0.5 shadow-md z-10 text-[10px]">
              <span class="text-[#9CA3AF] uppercase font-bold mr-1">{block.block_type}</span>
              <button onclick={() => moveBlock(index, "up")} disabled={index === 0} class="p-1 hover:text-white text-[#9CA3AF]">▲</button>
              <button onclick={() => moveBlock(index, "down")} disabled={index === blocks.length - 1} class="p-1 hover:text-white text-[#9CA3AF]">▼</button>
              <button onclick={() => handleAiAssist(index, "expand")} title="Kembangkan Teks" class="p-1 hover:text-[#A78BFA] text-[#9CA3AF]">✨ Kembangkan</button>
              <button onclick={() => handleAiAssist(index, "polish")} title="Perbaiki Nada Bahasa" class="p-1 hover:text-[#A78BFA] text-[#9CA3AF]">💎 Polish</button>
              <button onclick={() => removeBlock(index)} title="Hapus Blok" class="p-1 hover:text-red-400 text-[#9CA3AF]">🗑️</button>
            </div>

            <!-- Block Content Render & Edit -->
            {#if block.block_type === "heading"}
              <div class="flex items-center gap-2">
                <span class="text-xs font-mono text-[#8B5CF6]">H{block.meta || '2'}</span>
                <input
                  type="text"
                  bind:value={block.content}
                  onblur={handleSaveBlocks}
                  class="w-full bg-transparent font-bold text-lg md:text-xl text-white focus:outline-none focus:border-b border-[#8B5CF6]"
                />
              </div>
            {:else if block.block_type === "quote"}
              <div class="border-l-4 border-[#8B5CF6] pl-4 italic text-[#D1D5DB]">
                <textarea
                  bind:value={block.content}
                  onblur={handleSaveBlocks}
                  rows="2"
                  class="w-full bg-transparent focus:outline-none resize-none"
                ></textarea>
              </div>
            {:else if block.block_type === "callout"}
              <div class="bg-[#1E1B4B]/40 border border-[#4338CA]/40 rounded-lg p-3 text-[#C7D2FE] flex gap-3">
                <span class="text-lg">💡</span>
                <textarea
                  bind:value={block.content}
                  onblur={handleSaveBlocks}
                  rows="2"
                  class="w-full bg-transparent focus:outline-none resize-none text-xs"
                ></textarea>
              </div>
            {:else if block.block_type === "stat"}
              <div class="bg-[#064E3B]/20 border border-[#059669]/30 rounded-lg p-4 flex flex-col items-center justify-center text-center">
                <input
                  type="text"
                  bind:value={block.content}
                  onblur={handleSaveBlocks}
                  class="w-full bg-transparent text-center font-extrabold text-xl text-[#34D399] focus:outline-none"
                />
                <span class="text-[10px] text-[#6EE7B7] uppercase tracking-wider mt-1">Metrik Unggulan</span>
              </div>
            {:else if block.block_type === "slide_break"}
              <div class="flex items-center justify-center py-2 text-xs text-[#6B7280] font-mono border-y border-dashed border-[#272732]">
                ─── SLIDE BREAK / HALAMAN BARU ───
              </div>
            {:else if block.block_type === "code"}
              <div class="bg-[#050507] rounded-lg p-3 font-mono text-xs border border-[#272732]">
                <textarea
                  bind:value={block.content}
                  onblur={handleSaveBlocks}
                  rows="3"
                  class="w-full bg-transparent text-[#22D3EE] focus:outline-none resize-y"
                ></textarea>
              </div>
            {:else}
              <!-- Paragraph default -->
              <textarea
                bind:value={block.content}
                onblur={handleSaveBlocks}
                rows="3"
                class="w-full bg-transparent text-sm text-[#E5E7EB] focus:outline-none resize-y leading-relaxed"
              ></textarea>
            {/if}
          </div>

          <!-- Quick Insert Button Between Blocks -->
          <div class="opacity-0 hover:opacity-100 flex items-center justify-center py-1 transition-opacity">
            <div class="flex items-center gap-1.5 bg-[#18181F] border border-[#272732] rounded-full px-3 py-1 shadow-lg text-[11px] text-[#9CA3AF]">
              <span class="text-[10px] font-bold text-[#8B5CF6]">+ Tambah:</span>
              <button onclick={() => addBlock("paragraph", index)} class="hover:text-white px-1">Teks</button>
              <button onclick={() => addBlock("heading", index)} class="hover:text-white px-1">Judul</button>
              <button onclick={() => addBlock("callout", index)} class="hover:text-white px-1">Callout</button>
              <button onclick={() => addBlock("quote", index)} class="hover:text-white px-1">Kutipan</button>
              <button onclick={() => addBlock("stat", index)} class="hover:text-white px-1">Metrik</button>
              <button onclick={() => addBlock("slide_break", index)} class="hover:text-white px-1">Pemisah</button>
            </div>
          </div>
        {/each}
      </div>
    {:else}
      <!-- Raw Markdown Mode -->
      <div class="h-full flex flex-col space-y-3">
        <textarea
          bind:value={rawMarkdown}
          class="w-full flex-1 bg-[#121217] border border-[#272732] rounded-xl p-5 text-sm font-mono text-[#E5E7EB] focus:border-[#8B5CF6] focus:outline-none resize-none leading-relaxed"
          rows="22"
        ></textarea>
        <div class="text-right">
          <span class="text-xs text-[#6B7280]">Mode Raw Markdown disinkronkan langsung ke canvas.</span>
        </div>
      </div>
    {/if}
  </div>
</div>
