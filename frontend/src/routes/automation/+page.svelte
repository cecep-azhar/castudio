<script lang="ts">
  import AutomationStudio from "$lib/components/AutomationStudio.svelte";
  import { projectStore } from "$lib/stores/projectStore.svelte";
</script>

<AutomationStudio projectId={projectStore.activeProject?.id} />
