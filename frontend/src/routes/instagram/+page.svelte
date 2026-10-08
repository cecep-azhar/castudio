<script lang="ts">
  import InstagramStudio from "$lib/components/InstagramStudio.svelte";
  import { projectStore } from "$lib/stores/projectStore.svelte";
</script>

<InstagramStudio projectId={projectStore.activeProject?.id} />
