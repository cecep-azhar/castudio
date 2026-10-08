<script lang="ts">
  import { onMount } from "svelte";
  import {
    listProjects,
    createProject,
    deleteProject,
    getBible,
    saveBible,
    listDocuments,
    createDocument,
    type Project,
    type ProjectBible,
    type Document,
  } from "$lib/api";

  import ContentBuilder from "$lib/components/ContentBuilder.svelte";
  import EbookStudio from "$lib/components/EbookStudio.svelte";
  import InstagramStudio from "$lib/components/InstagramStudio.svelte";
  import RepurposingStudio from "$lib/components/RepurposingStudio.svelte";
  import PitchdeckStudio from "$lib/components/PitchdeckStudio.svelte";
  import PromptStudio from "$lib/components/PromptStudio.svelte";

  type NavSection = "projects" | "builder" | "ebook" | "instagram" | "repurpose" | "pitchdeck" | "prompts";

  let activeSection = $state<NavSection>("projects");
  let projects = $state<Project[]>([]);
  let activeProject = $state<Project | null>(null);
  let activeBible = $state<ProjectBible | null>(null);
  let documents = $state<Document[]>([]);
  let activeDoc = $state<Document | null>(null);

  // Modals & form state
  let showNewProjectModal = $state(false);
  let showBibleDrawer = $state(false);
  let newProjectTitle = $state("");
  let newProjectDesc = $state("");
  let newProjectAudience = $state("Software engineers & tech founders");
  let newProjectTone = $state("Sovereign, Direct, Minimalist");

  onMount(async () => {
    await loadProjects();
  });

  async function loadProjects() {
    try {
      const list = await listProjects();
      projects = list;
      if (projects.length > 0 && !activeProject) {
        selectProject(projects[0]);
      } else if (projects.length === 0) {
        // Auto-create default starter project if brand new
        const starter = await createProject({
          title: "Sovereign Engineering Studio",
          description: "Studio produksi konten teknis, e-book, dan peluncuran produk digital.",
          target_audience: "Software engineers, tech founders, and developers",
          default_tone: "Direct, Sovereign, Minimalist",
        });
        projects = [starter];
        selectProject(starter);
      }
    } catch (e) {
      console.error(e);
    }
  }

  async function selectProject(p: Project) {
    activeProject = p;
    try {
      const bible = await getBible(p.id);
      activeBible = bible;
      const docs = await listDocuments(p.id);
      documents = docs;
      if (docs.length > 0) {
        activeDoc = docs[0];
      } else {
        // create starter document
        const starterDoc = await createDocument({
          project_id: p.id,
          title: "Dokumen Perdana",
          doc_type: "custom",
          initial_content: "Selamat datang di CAStudio. Mulai susun ide konten berdaulat Anda di sini.",
        });
        documents = [starterDoc];
        activeDoc = starterDoc;
      }
    } catch (e) {
      console.error(e);
    }
  }

  async function handleCreateProject() {
    if (!newProjectTitle.trim()) return;
    try {
      const p = await createProject({
        title: newProjectTitle,
        description: newProjectDesc,
        target_audience: newProjectAudience,
        default_tone: newProjectTone,
      });
      projects.unshift(p);
      selectProject(p);
      showNewProjectModal = false;
      newProjectTitle = "";
      newProjectDesc = "";
    } catch (e) {
      alert(`Gagal membuat project: ${e}`);
    }
  }

  async function handleSaveBible() {
    if (!activeBible) return;
    try {
      activeBible = await saveBible(activeBible);
      showBibleDrawer = false;
      alert("Project Bible berhasil diperbarui!");
    } catch (e) {
      alert(`Gagal menyimpan Bible: ${e}`);
    }
  }

  async function handleDeleteProject(id: string, e: MouseEvent) {
    e.stopPropagation();
    if (!confirm("Hapus project ini beserta seluruh dokumen di dalamnya?")) return;
    try {
      await deleteProject(id);
      activeProject = null;
      await loadProjects();
    } catch (err) {
      alert(`Gagal: ${err}`);
    }
  }

  async function handleCreateNewDoc() {
    if (!activeProject) return;
    const title = prompt("Judul dokumen baru:", "Draf Baru");
    if (!title) return;
    try {
      const d = await createDocument({
        project_id: activeProject.id,
        title,
        doc_type: "custom",
        initial_content: "Tulis isi dokumen...",
      });
      documents.unshift(d);
      activeDoc = d;
      activeSection = "builder";
    } catch (e) {
      alert(`Gagal: ${e}`);
    }
  }
</script>

<!-- Outer Container -->
<div class="flex-1 flex overflow-hidden">
  <!-- Left Main Navigation Bar (Slim & Clean) -->
  <aside class="w-16 border-r border-[#272732] bg-[#0A0A0C] flex flex-col items-center py-4 space-y-3 shrink-0 select-none z-20">
    <button
      onclick={() => (activeSection = "projects")}
      title="Proyek & Project Bible"
      class="w-11 h-11 rounded-xl flex items-center justify-center transition-all {activeSection === 'projects' ? 'bg-[#8B5CF6] text-white shadow-lg shadow-[#8B5CF6]/30' : 'text-[#9CA3AF] hover:bg-[#18181F] hover:text-white'}"
    >
      <span class="text-lg">📁</span>
    </button>

    <button
      onclick={() => (activeSection = "builder")}
      title="Content Builder (Modular Blocks)"
      class="w-11 h-11 rounded-xl flex items-center justify-center transition-all {activeSection === 'builder' ? 'bg-[#8B5CF6] text-white shadow-lg shadow-[#8B5CF6]/30' : 'text-[#9CA3AF] hover:bg-[#18181F] hover:text-white'}"
    >
      <span class="text-lg">✍️</span>
    </button>

    <button
      onclick={() => (activeSection = "ebook")}
      title="E-Book Studio"
      class="w-11 h-11 rounded-xl flex items-center justify-center transition-all {activeSection === 'ebook' ? 'bg-[#8B5CF6] text-white shadow-lg shadow-[#8B5CF6]/30' : 'text-[#9CA3AF] hover:bg-[#18181F] hover:text-white'}"
    >
      <span class="text-lg">📚</span>
    </button>

    <button
      onclick={() => (activeSection = "instagram")}
      title="Instagram Suite (Carousel 4:5 & Reels 9:16)"
      class="w-11 h-11 rounded-xl flex items-center justify-center transition-all {activeSection === 'instagram' ? 'bg-[#8B5CF6] text-white shadow-lg shadow-[#8B5CF6]/30' : 'text-[#9CA3AF] hover:bg-[#18181F] hover:text-white'}"
    >
      <span class="text-lg">📸</span>
    </button>

    <button
      onclick={() => (activeSection = "repurpose")}
      title="Repurposing (Launch Pack)"
      class="w-11 h-11 rounded-xl flex items-center justify-center transition-all {activeSection === 'repurpose' ? 'bg-[#8B5CF6] text-white shadow-lg shadow-[#8B5CF6]/30' : 'text-[#9CA3AF] hover:bg-[#18181F] hover:text-white'}"
    >
      <span class="text-lg">🚀</span>
    </button>

    <button
      onclick={() => (activeSection = "pitchdeck")}
      title="Pitchdeck & SPK Closing"
      class="w-11 h-11 rounded-xl flex items-center justify-center transition-all {activeSection === 'pitchdeck' ? 'bg-[#8B5CF6] text-white shadow-lg shadow-[#8B5CF6]/30' : 'text-[#9CA3AF] hover:bg-[#18181F] hover:text-white'}"
    >
      <span class="text-lg">📊</span>
    </button>

    <button
      onclick={() => (activeSection = "prompts")}
      title="Prompt Studio"
      class="w-11 h-11 rounded-xl flex items-center justify-center transition-all {activeSection === 'prompts' ? 'bg-[#8B5CF6] text-white shadow-lg shadow-[#8B5CF6]/30' : 'text-[#9CA3AF] hover:bg-[#18181F] hover:text-white'}"
    >
      <span class="text-lg">⚡</span>
    </button>

    <!-- Bottom Spacing -->
    <div class="mt-auto"></div>
  </aside>

  <!-- Section 1: Dashboard / Projects & Documents Overview -->
  {#if activeSection === "projects"}
    <div class="flex-1 flex flex-col h-full bg-[#0A0A0C] overflow-hidden">
      <!-- Header -->
      <div class="h-14 border-b border-[#272732] px-8 flex items-center justify-between bg-[#121217] shrink-0">
        <div>
          <h1 class="font-bold text-white text-base">Dashboard Proyek & Project Bible</h1>
          <p class="text-[11px] text-[#9CA3AF]">Satu Sumber Kebenaran untuk seluruh format konten turunan.</p>
        </div>

        <div class="flex items-center gap-2">
          {#if activeProject}
            <button
              onclick={() => (showBibleDrawer = true)}
              class="px-3.5 py-1.5 rounded-lg border border-[#8B5CF6]/40 bg-[#8B5CF6]/20 hover:bg-[#8B5CF6]/30 text-[#C4B5FD] text-xs font-semibold flex items-center gap-1.5 transition-all"
            >
              <span>📖</span> Edit Project Bible
            </button>
          {/if}
          <button
            onclick={() => (showNewProjectModal = true)}
            class="px-4 py-1.5 rounded-lg bg-[#8B5CF6] hover:bg-[#7C3AED] text-white text-xs font-semibold shadow-md shadow-[#8B5CF6]/30 transition-all"
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
            {#each projects as p}
              <div
                onclick={() => selectProject(p)}
                role="button"
                tabindex="0"
                onkeydown={(e) => {
                  if (e.key === "Enter" || e.key === " ") {
                    selectProject(p);
                  }
                }}
                class="group p-4 rounded-xl border transition-all text-left flex flex-col justify-between {activeProject?.id === p.id ? 'bg-[#18181F] border-[#8B5CF6] shadow-lg shadow-[#8B5CF6]/10' : 'bg-[#121217] border-[#272732] hover:border-[#374151]'}"
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
                  <span class="text-[#8B5CF6] font-semibold">{activeProject?.id === p.id ? 'AKTIF ✓' : 'Pilih'}</span>
                </div>
              </div>
            {/each}
          </div>
        </div>

        <!-- Documents of Active Project -->
        {#if activeProject}
          <div class="space-y-4 pt-4 border-t border-[#272732]">
            <div class="flex items-center justify-between">
              <div>
                <h3 class="font-bold text-white text-xs uppercase tracking-wider text-[#9CA3AF]">
                  Dokumen Konten ({activeProject.title})
                </h3>
              </div>
              <button
                onclick={handleCreateNewDoc}
                class="px-3 py-1 bg-[#18181F] hover:bg-[#272732] border border-[#272732] text-white text-xs rounded-lg font-medium"
              >
                + Dokumen Baru
              </button>
            </div>

            <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-3">
              {#each documents as doc}
                <div
                  onclick={() => {
                    activeDoc = doc;
                    activeSection = "builder";
                  }}
                  role="button"
                  tabindex="0"
                  onkeydown={(e) => {
                    if (e.key === "Enter" || e.key === " ") {
                      activeDoc = doc;
                      activeSection = "builder";
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

              {#if documents.length === 0}
                <div class="col-span-full py-8 text-center text-xs text-[#6B7280]">
                  Belum ada dokumen di proyek ini. Klik tombol "+ Dokumen Baru" di atas.
                </div>
              {/if}
            </div>
          </div>
        {/if}
      </div>
    </div>
  {:else if activeSection === "builder"}
    {#if activeDoc}
      <ContentBuilder activeDoc={activeDoc} projectId={activeProject?.id} />
    {:else}
      <div class="flex-1 flex items-center justify-center text-xs text-[#6B7280]">
        Pilih atau buat dokumen terlebih dahulu di menu Proyek.
      </div>
    {/if}
  {:else if activeSection === "ebook"}
    <EbookStudio projectId={activeProject?.id || ""} />
  {:else if activeSection === "instagram"}
    <InstagramStudio projectId={activeProject?.id} />
  {:else if activeSection === "repurpose"}
    <RepurposingStudio projectId={activeProject?.id} />
  {:else if activeSection === "pitchdeck"}
    <PitchdeckStudio projectId={activeProject?.id} />
  {:else if activeSection === "prompts"}
    <PromptStudio projectId={activeProject?.id} />
  {/if}
</div>

<!-- Modal Create Project -->
{#if showNewProjectModal}
  <div class="fixed inset-0 z-50 flex items-center justify-center bg-black/70 backdrop-blur-sm p-4">
    <div class="bg-[#121217] border border-[#272732] rounded-xl w-full max-w-md p-5 space-y-4 shadow-2xl">
      <div class="flex items-center justify-between border-b border-[#272732] pb-3">
        <h3 class="font-bold text-white text-sm">Buat Proyek Baru</h3>
        <button onclick={() => (showNewProjectModal = false)} class="text-[#9CA3AF] hover:text-white">✕</button>
      </div>

      <div class="space-y-3 text-xs">
        <div>
          <label for="new-project-title-input" class="block text-[#9CA3AF] mb-1">Judul Proyek</label>
          <input id="new-project-title-input" type="text" bind:value={newProjectTitle} placeholder="Nama proyek/koleksi materi..." class="w-full bg-[#18181F] border border-[#272732] rounded p-2 text-white" />
        </div>
        <div>
          <label for="new-project-desc-input" class="block text-[#9CA3AF] mb-1">Deskripsi Singkat</label>
          <textarea id="new-project-desc-input" bind:value={newProjectDesc} rows="2" placeholder="Fokus & tujuan materi..." class="w-full bg-[#18181F] border border-[#272732] rounded p-2 text-white resize-none"></textarea>
        </div>
        <div>
          <label for="new-project-audience-input" class="block text-[#9CA3AF] mb-1">Audiens Target</label>
          <input id="new-project-audience-input" type="text" bind:value={newProjectAudience} class="w-full bg-[#18181F] border border-[#272732] rounded p-2 text-white" />
        </div>
        <div>
          <label for="new-project-tone-input" class="block text-[#9CA3AF] mb-1">Nada Suara Default</label>
          <input id="new-project-tone-input" type="text" bind:value={newProjectTone} class="w-full bg-[#18181F] border border-[#272732] rounded p-2 text-white" />
        </div>
      </div>

      <div class="flex items-center justify-end gap-2 pt-2 border-t border-[#272732]">
        <button onclick={() => (showNewProjectModal = false)} class="px-3 py-1.5 rounded hover:bg-[#18181F] text-xs text-[#9CA3AF]">Batal</button>
        <button onclick={handleCreateProject} class="px-4 py-1.5 bg-[#8B5CF6] hover:bg-[#7C3AED] text-white rounded text-xs font-semibold">Simpan Proyek</button>
      </div>
    </div>
  </div>
{/if}

<!-- Project Bible Drawer -->
{#if showBibleDrawer && activeBible}
  <div class="fixed inset-0 z-50 flex justify-end bg-black/60 backdrop-blur-sm">
    <div class="w-full max-w-xl h-full bg-[#121217] border-l border-[#272732] flex flex-col p-6 space-y-4 shadow-2xl overflow-y-auto">
      <div class="flex items-center justify-between border-b border-[#272732] pb-3">
        <div>
          <h3 class="font-bold text-white text-sm">📖 Project Bible: {activeProject?.title}</h3>
          <p class="text-[10px] text-[#9CA3AF]">Konteks ini otomatis disuntikkan ke setiap pemanggilan AI untuk menjaga konsistensi gaya bahasa dan fakta.</p>
        </div>
        <button onclick={() => (showBibleDrawer = false)} class="text-[#9CA3AF] hover:text-white">✕</button>
      </div>

      <div class="space-y-4 text-xs">
        <div>
          <label for="bible-voice-input" class="block text-[#A78BFA] font-bold mb-1">Gaya Bahasa & Nada Suara (Voice & Tone)</label>
          <textarea id="bible-voice-input" bind:value={activeBible.voice_sample} rows="3" placeholder="Contoh: Terse, tegas, teknis presisi, tanpa basa-basi pembuka..." class="w-full bg-[#18181F] border border-[#272732] rounded p-2.5 text-white resize-y font-mono"></textarea>
        </div>

        <div>
          <label for="bible-facts-input" class="block text-[#A78BFA] font-bold mb-1">Fakta & Angka Inti (Core Facts)</label>
          <textarea id="bible-facts-input" bind:value={activeBible.core_facts} rows="3" placeholder="Fakta baku yang tidak boleh diubah atau dikarang..." class="w-full bg-[#18181F] border border-[#272732] rounded p-2.5 text-white resize-y font-mono"></textarea>
        </div>

        <div>
          <label for="bible-terms-input" class="block text-[#A78BFA] font-bold mb-1">Istilah Kunci Baku (Key Terms)</label>
          <textarea id="bible-terms-input" bind:value={activeBible.key_terms} rows="3" placeholder="Daftar istilah teknis wajib..." class="w-full bg-[#18181F] border border-[#272732] rounded p-2.5 text-white resize-y font-mono"></textarea>
        </div>

        <div>
          <label for="bible-guidelines-input" class="block text-[#A78BFA] font-bold mb-1">Batasan & Panduan (Strict Guidelines)</label>
          <textarea id="bible-guidelines-input" bind:value={activeBible.guidelines} rows="3" placeholder="Hal-hal yang dilarang ditulis..." class="w-full bg-[#18181F] border border-[#272732] rounded p-2.5 text-white resize-y font-mono"></textarea>
        </div>
      </div>

      <div class="pt-4 border-t border-[#272732] flex items-center justify-end gap-2">
        <button onclick={() => (showBibleDrawer = false)} class="px-4 py-2 rounded hover:bg-[#18181F] text-xs text-[#9CA3AF]">Batal</button>
        <button onclick={handleSaveBible} class="px-5 py-2 bg-[#8B5CF6] hover:bg-[#7C3AED] text-white rounded text-xs font-semibold shadow-lg shadow-[#8B5CF6]/30">Simpan Project Bible</button>
      </div>
    </div>
  </div>
{/if}
