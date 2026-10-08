<script lang="ts">
  import PromptStudio from "$lib/components/PromptStudio.svelte";
  import { projectStore } from "$lib/stores/projectStore.svelte";
</script>

<PromptStudio projectId={projectStore.activeProject?.id} />
