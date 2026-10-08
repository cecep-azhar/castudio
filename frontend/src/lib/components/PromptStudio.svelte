<script lang="ts">
  import {
    listPrompts,
    createPrompt,
    togglePromptFavorite,
    deletePrompt,
    generateAi,
    type PromptTemplate,
  } from "$lib/api";
  import { onMount } from "svelte";

  interface Props {
    projectId?: string;
  }

  let { projectId }: Props = $props();

  let prompts = $state<PromptTemplate[]>([]);
  let activeCategory = $state<string>("all");
  let selectedPrompt = $state<PromptTemplate | null>(null);
  let variableInputs = $state<Record<string, string>>({});
  let running = $state(false);
  let runResult = $state<string | null>(null);
  let showNewModal = $state(false);

  // New prompt state
  let newTitle = $state("");
  let newCategory = $state("software_dev");
  let newSystem = $state("");
  let newUserPrompt = $state("");

  onMount(loadPrompts);

  async function loadPrompts() {
    try {
      const list = await listPrompts(activeCategory === "all" ? undefined : activeCategory);
      prompts = list;
      if (prompts.length > 0 && !selectedPrompt) {
        selectPrompt(prompts[0]);
      }
    } catch (e) {
      console.error(e);
    }
  }

  function selectPrompt(p: PromptTemplate) {
    selectedPrompt = p;
    runResult = null;
    variableInputs = {};
    try {
      const vars: string[] = JSON.parse(p.variables || "[]");
      for (const v of vars) {
        variableInputs[v] = "";
      }
    } catch (e) {
      console.error(e);
    }
  }

  async function handleToggleFav(p: PromptTemplate, e: MouseEvent) {
    e.stopPropagation();
    try {
      await togglePromptFavorite(p.id);
      await loadPrompts();
    } catch (err) {
      alert(`Gagal: ${err}`);
    }
  }

  async function handleDelete(p: PromptTemplate, e: MouseEvent) {
    e.stopPropagation();
    if (!confirm(`Hapus template '${p.title}'?`)) return;
    try {
      await deletePrompt(p.id);
      await loadPrompts();
      if (selectedPrompt?.id === p.id) {
        selectedPrompt = null;
      }
    } catch (err) {
      alert(`Gagal hapus: ${err}`);
    }
  }

  async function handleRunPrompt() {
    if (!selectedPrompt) return;
    running = true;
    runResult = null;
    try {
      let finalUserPrompt = selectedPrompt.user_prompt;
      for (const [key, val] of Object.entries(variableInputs)) {
        finalUserPrompt = finalUserPrompt.replaceAll(`{{${key}}}`, val);
      }

      const res = await generateAi({
        project_id: projectId,
        system_instruction: selectedPrompt.system_prompt,
        user_prompt: finalUserPrompt,
      });

      runResult = res.text;
    } catch (err: any) {
      alert(`Eksekusi prompt gagal: ${err}`);
    } finally {
      running = false;
    }
  }

  async function handleCreateNewPrompt() {
    try {
      // detect variables like {{variable}}
      const re = /\{\{([^}]+)\}\}/g;
      const foundVars: string[] = [];
      let m;
      while ((m = re.exec(newUserPrompt)) !== null) {
        if (!foundVars.includes(m[1].trim())) {
          foundVars.push(m[1].trim());
        }
      }

      await createPrompt({
        title: newTitle,
        category: newCategory,
        system_prompt: newSystem,
        user_prompt: newUserPrompt,
        variables: JSON.stringify(foundVars),
      });

      showNewModal = false;
      newTitle = "";
      newSystem = "";
      newUserPrompt = "";
      await loadPrompts();
    } catch (err) {
      alert(`Gagal membuat prompt: ${err}`);
    }
  }
</script>

<div class="flex-1 flex h-full bg-[#0A0A0C] overflow-hidden">
  <!-- Left Prompts Sidebar -->
  <div class="w-80 border-r border-[#272732] bg-[#121217] flex flex-col shrink-0">
    <!-- Header -->
    <div class="p-4 border-b border-[#272732] flex items-center justify-between">
      <div>
        <h3 class="font-bold text-white text-xs">Prompt Studio</h3>
        <span class="text-[10px] text-[#9CA3AF]">{prompts.length} Template Siap Pakai</span>
      </div>
      <button
        onclick={() => (showNewModal = true)}
        class="px-2 py-1 bg-[#8B5CF6] hover:bg-[#7C3AED] text-white rounded-lg text-xs font-semibold"
      >
        + Buat
      </button>
    </div>

    <!-- Category Filters -->
    <div class="p-2 border-b border-[#272732] flex items-center gap-1 overflow-x-auto text-[11px]">
      <button
        onclick={() => {
          activeCategory = "all";
          loadPrompts();
        }}
        class="px-2.5 py-1 rounded-md transition-colors {activeCategory === 'all' ? 'bg-[#8B5CF6]/30 text-white font-bold' : 'text-[#9CA3AF] hover:text-white'}"
      >
        Semua
      </button>
      <button
        onclick={() => {
          activeCategory = "software_dev";
          loadPrompts();
        }}
        class="px-2.5 py-1 rounded-md transition-colors {activeCategory === 'software_dev' ? 'bg-[#8B5CF6]/30 text-white font-bold' : 'text-[#9CA3AF] hover:text-white'}"
      >
        Dev / QA
      </button>
      <button
        onclick={() => {
          activeCategory = "media";
          loadPrompts();
        }}
        class="px-2.5 py-1 rounded-md transition-colors {activeCategory === 'media' ? 'bg-[#8B5CF6]/30 text-white font-bold' : 'text-[#9CA3AF] hover:text-white'}"
      >
        Media / Icon
      </button>
      <button
        onclick={() => {
          activeCategory = "content";
          loadPrompts();
        }}
        class="px-2.5 py-1 rounded-md transition-colors {activeCategory === 'content' ? 'bg-[#8B5CF6]/30 text-white font-bold' : 'text-[#9CA3AF] hover:text-white'}"
      >
        Konten
      </button>
    </div>

    <!-- Prompt List -->
    <div class="flex-1 overflow-y-auto p-2 space-y-1">
      {#each prompts as p}
        <div
          onclick={() => selectPrompt(p)}
          role="button"
          tabindex="0"
          onkeydown={(e) => {
            if (e.key === "Enter" || e.key === " ") {
              selectPrompt(p);
            }
          }}
          class="w-full text-left p-2.5 rounded-lg text-xs transition-all flex items-start justify-between group cursor-pointer {selectedPrompt?.id === p.id ? 'bg-[#8B5CF6]/20 border border-[#8B5CF6]/50 text-white' : 'text-[#9CA3AF] hover:bg-[#18181F] hover:text-white'}"
        >
          <div class="pr-2 truncate">
            <span class="font-semibold block truncate">{p.title}</span>
            <span class="text-[10px] text-[#6B7280] uppercase">{p.category}</span>
          </div>
          <div class="flex items-center gap-1 shrink-0">
            <button onclick={(e) => handleToggleFav(p, e)} class="p-0.5 hover:text-yellow-400">
              {p.is_favorite ? '⭐' : '☆'}
            </button>
            <button onclick={(e) => handleDelete(p, e)} class="p-0.5 opacity-0 group-hover:opacity-100 hover:text-red-400">
              ✕
            </button>
          </div>
        </div>
      {/each}
    </div>
  </div>

  <!-- Right Execution & Variable Runner Area -->
  <div class="flex-1 flex flex-col h-full overflow-hidden">
    {#if selectedPrompt}
      <div class="flex-1 overflow-y-auto p-6 md:p-8 max-w-4xl mx-auto w-full space-y-6">
        <!-- Template Meta Card -->
        <div class="bg-[#121217] border border-[#272732] rounded-xl p-5 space-y-3">
          <div class="flex items-center justify-between">
            <span class="text-[10px] uppercase font-bold text-[#A78BFA] px-2 py-0.5 rounded bg-[#8B5CF6]/20 border border-[#8B5CF6]/30">
              {selectedPrompt.category}
            </span>
            <span class="text-xs text-[#6B7280]">Eksekusi via AI Model</span>
          </div>
          <h2 class="text-base font-bold text-white">{selectedPrompt.title}</h2>
          <div class="p-3 bg-[#0A0A0C] border border-[#272732] rounded-lg text-xs font-mono text-[#9CA3AF]">
            <span class="text-[10px] text-[#8B5CF6] font-bold block mb-1">SYSTEM INSTRUCTION:</span>
            {selectedPrompt.system_prompt}
          </div>
        </div>

        <!-- Variable Fill-in Form -->
        <div class="bg-[#121217] border border-[#272732] rounded-xl p-5 space-y-4">
          <h3 class="font-bold text-white text-xs">Variabel Parameter Template:</h3>
          {#each Object.keys(variableInputs) as varKey}
            <div>
              <label for="var-input-{varKey}" class="block text-xs text-[#9CA3AF] mb-1 font-mono">{`{{${varKey}}}`}</label>
              <input
                id="var-input-{varKey}"
                type="text"
                bind:value={variableInputs[varKey]}
                placeholder={`Isi nilai untuk ${varKey}...`}
                class="w-full bg-[#18181F] border border-[#272732] rounded-lg px-3 py-2 text-xs text-white focus:border-[#8B5CF6] focus:outline-none"
              />
            </div>
          {/each}

          {#if Object.keys(variableInputs).length === 0}
            <p class="text-xs text-[#6B7280]">Template ini tidak memerlukan variabel dinamis tambahan.</p>
          {/if}

          <div class="text-right pt-2">
            <button
              onclick={handleRunPrompt}
              disabled={running}
              class="px-5 py-2 bg-[#8B5CF6] hover:bg-[#7C3AED] text-white rounded-lg text-xs font-semibold shadow-lg shadow-[#8B5CF6]/30 transition-all disabled:opacity-50"
            >
              {running ? "Mengeksekusi Prompt..." : "Jalankan Prompt AI"}
            </button>
          </div>
        </div>

        <!-- Execution Result Output -->
        {#if runResult}
          <div class="bg-[#121217] border border-[#272732] rounded-xl p-6 space-y-3">
            <div class="flex items-center justify-between">
              <h4 class="font-bold text-white text-xs">Hasil Eksekusi:</h4>
              <button
                onclick={() => {
                  navigator.clipboard.writeText(runResult || "");
                  alert("Hasil tersalin!");
                }}
                class="px-3 py-1 bg-[#10B981] hover:bg-[#059669] text-white rounded text-xs font-semibold"
              >
                Salin ke Clipboard
              </button>
            </div>
            <div class="font-mono text-xs text-[#D1D5DB] whitespace-pre-wrap leading-relaxed bg-[#0A0A0C] p-4 rounded-lg border border-[#272732]">
              {runResult}
            </div>
          </div>
        {/if}
      </div>
    {:else}
      <div class="flex-1 flex items-center justify-center text-xs text-[#6B7280]">
        Pilih template prompt di sebelah kiri.
      </div>
    {/if}
  </div>
</div>

<!-- Modal Create New Prompt -->
{#if showNewModal}
  <div class="fixed inset-0 z-50 flex items-center justify-center bg-black/70 backdrop-blur-sm p-4">
    <div class="bg-[#121217] border border-[#272732] rounded-xl w-full max-w-lg p-5 space-y-4 shadow-2xl">
      <div class="flex items-center justify-between border-b border-[#272732] pb-3">
        <h3 class="font-bold text-white text-sm">Buat Template Prompt Baru</h3>
        <button onclick={() => (showNewModal = false)} class="text-[#9CA3AF] hover:text-white">✕</button>
      </div>

      <div class="space-y-3 text-xs">
        <div>
          <label for="new-title-input" class="block text-[#9CA3AF] mb-1">Judul Template</label>
          <input id="new-title-input" type="text" bind:value={newTitle} class="w-full bg-[#18181F] border border-[#272732] rounded p-2 text-white" />
        </div>
        <div>
          <label for="new-category-select" class="block text-[#9CA3AF] mb-1">Kategori</label>
          <select id="new-category-select" bind:value={newCategory} class="w-full bg-[#18181F] border border-[#272732] rounded p-2 text-white">
            <option value="software_dev">Software Dev / QA Audit</option>
            <option value="media">Media / Icon Generator</option>
            <option value="content">Konten / Marketing</option>
          </select>
        </div>
        <div>
          <label for="new-system-input" class="block text-[#9CA3AF] mb-1">Instruksi Sistem (System Prompt)</label>
          <textarea id="new-system-input" bind:value={newSystem} rows="2" class="w-full bg-[#18181F] border border-[#272732] rounded p-2 text-white resize-none"></textarea>
        </div>
        <div>
          <label for="new-user-prompt-input" class="block text-[#9CA3AF] mb-1">Isi Prompt Pengguna (Gunakan kurung ganda seperti &#123;&#123;topik&#125;&#125;)</label>
          <textarea id="new-user-prompt-input" bind:value={newUserPrompt} rows="4" class="w-full bg-[#18181F] border border-[#272732] rounded p-2 text-white resize-none"></textarea>
        </div>
      </div>

      <div class="flex items-center justify-end gap-2 pt-2 border-t border-[#272732]">
        <button onclick={() => (showNewModal = false)} class="px-3 py-1.5 rounded hover:bg-[#18181F] text-xs text-[#9CA3AF]">Batal</button>
        <button onclick={handleCreateNewPrompt} class="px-4 py-1.5 bg-[#8B5CF6] hover:bg-[#7C3AED] text-white rounded text-xs font-semibold">Simpan Template</button>
      </div>
    </div>
  </div>
{/if}
