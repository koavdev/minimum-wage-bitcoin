<script lang="ts">
  export let data: { wageData: any };
</script>

{#if data?.wageData}
  <pre>{JSON.stringify(data.wageData, null, 2)}</pre>
{:else}
  <p>Carregando ou sem dados</p>
{/if}