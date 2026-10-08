<script lang="ts">
  import {
    createProject,
    deleteProject,
    saveBible,
    type Project,
  } from "$lib/api";
  import { projectStore } from "$lib/stores/projectStore.svelte";
  import { goto } from "$app/navigation";

  // Modals & form state
  let showNewProjectModal = $state(false);
  let showBibleDrawer = $state(false);
  let newProjectTitle = $state("");
  let newProjectDesc = $state("");
  let newProjectAudience = $state("Software engineers & tech founders");
  let newProjectTone = $state("Sovereign, Direct, Minimalist");

  async function handleCreateProject() {
    if (!newProjectTitle.trim()) return;
    try {
      const p = await createProject({
        title: newProjectTitle,
        description: newProjectDesc ? newProjectDesc : undefined,
        target_audience: newProjectAudience,
        default_tone: newProjectTone,
      });
      projectStore.projects.unshift(p);
      await projectStore.selectProject(p);
      showNewProjectModal = false;
      newProjectTitle = "";
      newProjectDesc = "";
    } catch (e) {
      alert(`Gagal membuat proyek: ${e}`);
    }
  }

  async function handleSaveBible() {
    if (!projectStore.activeBible) return;
    try {
      await saveBible(projectStore.activeBible);
      showBibleDrawer = false;
      alert("Project Bible tersimpan!");
    } catch (e) {
      alert(`Gagal: ${e}`);
    }
  }

  async function handleDeleteProject(id: string, e: MouseEvent) {
    e.stopPropagation();
    if (!confirm("Hapus project ini beserta seluruh dokumen di dalamnya?")) return;
    try {
      await deleteProject(id);
      await projectStore.loadProjects();
    } catch (err) {
      alert(`Gagal: ${err}`);
    }
  }

  async function handleCreateNewDoc() {
    if (!projectStore.activeProject) return;
    const title = prompt("Judul dokumen baru:", "Draf Baru");
    if (!title) return;
    try {
      await projectStore.createNewDocument(title);
      goto("/content");
    } catch (e) {
      alert(`Gagal: ${e}`);
    }
  }
</script>

<div class="flex-1 flex flex-col h-full bg-[#0A0A0C] overflow-hidden select-none">
  <!-- Header -->
  <div class="h-14 border-b border-[#272732] px-8 flex items-center justify-between bg-[#121217] shrink-0">
    <div>
      <h1 class="font-bold text-white text-base">Dashboard Proyek & Project Bible</h1>
      <p class="text-[11px] text-[#9CA3AF]">Satu Sumber Kebenaran untuk seluruh format konten turunan CADS v1.0.</p>
    </div>

    <div class="flex items-center gap-2">
      {#if projectStore.activeProject}
        <button
          onclick={() => (showBibleDrawer = true)}
          class="px-3.5 py-1.5 rounded-lg border border-[#8B5CF6]/40 bg-[#8B5CF6]/20 hover:bg-[#8B5CF6]/30 text-[#C4B5FD] text-xs font-semibold flex items-center gap-1.5 transition-all cursor-pointer"
        >
          <span>📖</span> Edit Project Bible
        </button>
      {/if}
      <button
        onclick={() => (showNewProjectModal = true)}
        class="px-4 py-1.5 rounded-lg bg-[#8B5CF6] hover:bg-[#7C3AED] text-white text-xs font-semibold shadow-md shadow-[#8B5CF6]/30 transition-all cursor-pointer"
      >
        + Proyek Baru
      </button>
    </div>
  </div>

  <!-- Projects Grid & Documents Area -->
  <div class="flex-1 overflow-y-auto p-8 max-w-6xl mx-auto w-full space-y-8">
    <!-- Projects Row -->
    <div class="space-y-3">
      <h3 class="font-bold text-white text-xs uppercase tracking-wider text-[#9CA3AF]">Daftar Proyek Aktif</h3>
      <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
        {#each projectStore.projects as p}
          <div
            onclick={() => projectStore.selectProject(p)}
            role="button"
            tabindex="0"
            onkeydown={(e) => {
              if (e.key === "Enter" || e.key === " ") {
                projectStore.selectProject(p);
              }
            }}
            class="group p-4 rounded-xl border transition-all text-left flex flex-col justify-between cursor-pointer {projectStore.activeProject?.id === p.id ? 'bg-[#18181F] border-[#8B5CF6] shadow-lg shadow-[#8B5CF6]/10' : 'bg-[#121217] border-[#272732] hover:border-[#374151]'}"
          >
            <div>
              <div class="flex items-center justify-between mb-2">
                <span class="text-xs font-bold text-white truncate pr-2">{p.title}</span>
                <button onclick={(e) => handleDeleteProject(p.id, e)} class="opacity-0 group-hover:opacity-100 text-[#9CA3AF] hover:text-red-400 p-0.5 text-xs">🗑️</button>
              </div>
              <p class="text-xs text-[#9CA3AF] line-clamp-2 leading-relaxed">{p.description || "Tanpa deskripsi."}</p>
            </div>
            <div class="mt-4 pt-3 border-t border-[#272732] flex items-center justify-between text-[10px] text-[#6B7280]">
              <span>{p.default_tone}</span>
              <span class="text-[#8B5CF6] font-semibold">{projectStore.activeProject?.id === p.id ? 'AKTIF ✓' : 'Pilih'}</span>
            </div>
          </div>
        {/each}
      </div>
    </div>

    <!-- Documents of Active Project -->
    {#if projectStore.activeProject}
      <div class="space-y-4 pt-4 border-t border-[#272732]">
        <div class="flex items-center justify-between">
          <div>
            <h3 class="font-bold text-white text-xs uppercase tracking-wider text-[#9CA3AF]">
              Dokumen Konten ({projectStore.activeProject.title})
            </h3>
          </div>
          <button
            onclick={handleCreateNewDoc}
            class="px-3 py-1 bg-[#18181F] hover:bg-[#272732] border border-[#272732] text-white text-xs rounded-lg font-medium cursor-pointer"
          >
            + Dokumen Baru
          </button>
        </div>

        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-3">
          {#each projectStore.documents as doc}
            <div
              onclick={() => {
                projectStore.activeDoc = doc;
                goto("/content");
              }}
              role="button"
              tabindex="0"
              onkeydown={(e) => {
                if (e.key === "Enter" || e.key === " ") {
                  projectStore.activeDoc = doc;
                  goto("/content");
                }
              }}
              class="p-4 bg-[#121217] hover:bg-[#18181F] border border-[#272732] hover:border-[#8B5CF6]/50 rounded-xl cursor-pointer transition-all space-y-2 text-left"
            >
              <div class="flex items-center justify-between">
                <span class="text-[10px] uppercase font-bold text-[#A78BFA] px-2 py-0.5 rounded bg-[#8B5CF6]/20">
                  {doc.doc_type}
                </span>
                <span class="text-[10px] text-[#6B7280]">{doc.word_count} kata</span>
              </div>
              <h4 class="font-bold text-white text-xs truncate">{doc.title}</h4>
              <div class="text-[10px] text-[#8B5CF6] font-medium pt-1">Buka di Editor ➔</div>
            </div>
          {/each}

          {#if projectStore.documents.length === 0}
            <div class="col-span-full py-8 text-center text-xs text-[#6B7280]">
              Belum ada dokumen di proyek ini. Klik tombol "+ Dokumen Baru" di atas.
            </div>
          {/if}
        </div>
      </div>
    {/if}
  </div>
</div>

<!-- Modal Create Project -->
{#if showNewProjectModal}
  <div class="fixed inset-0 z-50 flex items-center justify-center bg-black/70 backdrop-blur-sm p-4">
    <div class="bg-[#121217] border border-[#272732] rounded-xl w-full max-w-md p-5 space-y-4 shadow-2xl">
      <div class="flex items-center justify-between border-b border-[#272732] pb-3">
        <h3 class="font-bold text-white text-sm">Buat Proyek Baru</h3>
        <button onclick={() => (showNewProjectModal = false)} class="text-[#9CA3AF] hover:text-white cursor-pointer">✕</button>
      </div>

      <div class="space-y-3 text-xs">
        <div>
          <label for="project-title-input" class="block text-[#9CA3AF] mb-1 font-medium">Judul Proyek</label>
          <input id="project-title-input" bind:value={newProjectTitle} placeholder="e.g. Masterclass AI Coding 2026" class="w-full bg-[#18181F] border border-[#272732] rounded p-2 text-white" />
        </div>
        <div>
          <label for="project-desc-input" class="block text-[#9CA3AF] mb-1 font-medium">Deskripsi Singkat</label>
          <textarea id="project-desc-input" bind:value={newProjectDesc} rows="2" placeholder="Fokus & tujuan konten..." class="w-full bg-[#18181F] border border-[#272732] rounded p-2 text-white resize-none"></textarea>
        </div>
        <div>
          <label for="project-aud-input" class="block text-[#9CA3AF] mb-1 font-medium">Target Audiens</label>
          <input id="project-aud-input" bind:value={newProjectAudience} class="w-full bg-[#18181F] border border-[#272732] rounded p-2 text-white" />
        </div>
        <div>
          <label for="project-tone-input" class="block text-[#9CA3AF] mb-1 font-medium">Default Tone of Voice</label>
          <input id="project-tone-input" bind:value={newProjectTone} class="w-full bg-[#18181F] border border-[#272732] rounded p-2 text-white" />
        </div>
      </div>

      <div class="flex items-center justify-end gap-2 pt-2 border-t border-[#272732]">
        <button onclick={() => (showNewProjectModal = false)} class="px-3 py-1.5 rounded hover:bg-[#18181F] text-xs text-[#9CA3AF] cursor-pointer">Batal</button>
        <button onclick={handleCreateProject} class="px-4 py-1.5 bg-[#8B5CF6] hover:bg-[#7C3AED] text-white rounded text-xs font-semibold cursor-pointer">Simpan Proyek</button>
      </div>
    </div>
  </div>
{/if}

<!-- Project Bible Drawer -->
{#if showBibleDrawer && projectStore.activeBible}
  <div class="fixed inset-0 z-50 flex justify-end bg-black/60 backdrop-blur-sm">
    <div class="w-full max-w-xl h-full bg-[#121217] border-l border-[#272732] flex flex-col p-6 space-y-4 shadow-2xl overflow-y-auto">
      <div class="flex items-center justify-between border-b border-[#272732] pb-3">
        <div>
          <h3 class="font-bold text-white text-sm">📖 Project Bible: {projectStore.activeProject?.title}</h3>
          <p class="text-[10px] text-[#9CA3AF]">Konteks ini otomatis disuntikkan ke setiap pemanggilan AI untuk menjaga konsistensi gaya bahasa dan fakta.</p>
        </div>
        <button onclick={() => (showBibleDrawer = false)} class="text-[#9CA3AF] hover:text-white cursor-pointer">✕</button>
      </div>

      <div class="space-y-4 text-xs">
        <div>
          <label for="bible-voice-input" class="block text-[#A78BFA] font-bold mb-1">Gaya Bahasa & Nada Suara (Voice & Tone)</label>
          <textarea id="bible-voice-input" bind:value={projectStore.activeBible.voice_sample} rows="3" placeholder="Contoh: Terse, tegas, teknis presisi, tanpa basa-basi pembuka..." class="w-full bg-[#18181F] border border-[#272732] rounded p-2.5 text-white resize-y font-mono"></textarea>
        </div>

        <div>
          <label for="bible-facts-input" class="block text-[#A78BFA] font-bold mb-1">Fakta & Angka Inti (Core Facts)</label>
          <textarea id="bible-facts-input" bind:value={projectStore.activeBible.core_facts} rows="3" placeholder="Fakta baku yang tidak boleh diubah atau dikarang..." class="w-full bg-[#18181F] border border-[#272732] rounded p-2.5 text-white resize-y font-mono"></textarea>
        </div>

        <div>
          <label for="bible-terms-input" class="block text-[#A78BFA] font-bold mb-1">Istilah Kunci Baku (Key Terms)</label>
          <textarea id="bible-terms-input" bind:value={projectStore.activeBible.key_terms} rows="3" placeholder="Daftar istilah teknis wajib..." class="w-full bg-[#18181F] border border-[#272732] rounded p-2.5 text-white resize-y font-mono"></textarea>
        </div>

        <div>
          <label for="bible-guidelines-input" class="block text-[#A78BFA] font-bold mb-1">Batasan & Panduan (Strict Guidelines)</label>
          <textarea id="bible-guidelines-input" bind:value={projectStore.activeBible.guidelines} rows="3" placeholder="Hal-hal yang dilarang ditulis..." class="w-full bg-[#18181F] border border-[#272732] rounded p-2.5 text-white resize-y font-mono"></textarea>
        </div>
      </div>

      <div class="pt-4 border-t border-[#272732] flex items-center justify-end gap-2">
        <button onclick={() => (showBibleDrawer = false)} class="px-4 py-2 rounded hover:bg-[#18181F] text-xs text-[#9CA3AF] cursor-pointer">Batal</button>
        <button onclick={handleSaveBible} class="px-5 py-2 bg-[#8B5CF6] hover:bg-[#7C3AED] text-white rounded text-xs font-semibold shadow-lg shadow-[#8B5CF6]/30 cursor-pointer">Simpan Project Bible</button>
      </div>
    </div>
  </div>
{/if}
