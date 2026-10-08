<script lang="ts">
  import ContentBuilder from "$lib/components/ContentBuilder.svelte";
  import { projectStore } from "$lib/stores/projectStore.svelte";
  import { goto } from "$app/navigation";

  async function handleCreateNewDoc() {
    const title = prompt("Judul dokumen baru:", "Draf Baru");
    if (!title) return;
    await projectStore.createNewDocument(title);
  }
</script>

{#if projectStore.activeDoc}
  <ContentBuilder
    activeDoc={projectStore.activeDoc}
    projectId={projectStore.activeProject?.id}
  />
{:else}
  <div class="flex-1 flex flex-col items-center justify-center p-12 text-center text-neutral-500 space-y-4">
    <div class="w-16 h-16 rounded-2xl bg-[#18181F] border border-[#272732] flex items-center justify-center text-2xl">
      📄
    </div>
    <h3 class="font-bold text-neutral-300 text-sm">Belum Ada Dokumen Terpilih</h3>
    <p class="text-xs max-w-md text-neutral-500 leading-relaxed">
      Pilih dokumen yang sudah ada di halaman Proyek atau buat dokumen baru untuk mulai menyusun naskah modular.
    </p>
    <div class="flex items-center gap-3">
      <button
        onclick={() => goto("/")}
        class="px-4 py-2 bg-[#18181F] hover:bg-[#272732] border border-[#272732] text-white text-xs rounded-xl font-medium cursor-pointer"
      >
        Lihat Semua Proyek
      </button>
      <button
        onclick={handleCreateNewDoc}
        class="px-4 py-2 bg-[#8B5CF6] hover:bg-[#7C3AED] text-white text-xs rounded-xl font-bold cursor-pointer"
      >
        + Buat Dokumen Baru
      </button>
    </div>
  </div>
{/if}
