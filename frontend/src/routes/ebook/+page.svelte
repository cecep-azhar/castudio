<script lang="ts">
  import EbookStudio from "$lib/components/EbookStudio.svelte";
  import { projectStore } from "$lib/stores/projectStore.svelte";
</script>

<EbookStudio projectId={projectStore.activeProject?.id || ""} />
