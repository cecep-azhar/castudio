<script lang="ts">
  import RepurposingStudio from "$lib/components/RepurposingStudio.svelte";
  import { projectStore } from "$lib/stores/projectStore.svelte";
</script>

<RepurposingStudio projectId={projectStore.activeProject?.id} />
