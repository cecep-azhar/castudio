<script lang="ts">
  import PitchdeckStudio from "$lib/components/PitchdeckStudio.svelte";
  import { projectStore } from "$lib/stores/projectStore.svelte";
</script>

<PitchdeckStudio projectId={projectStore.activeProject?.id} />
