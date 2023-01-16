<script>
/** @type {Error|null} */
export let error = null;
</script>

<slot>
  {#if error}
    <div class="error">
      <b>{error.message}</b>
      <pre class="trace">
          {error.stack}
        </pre>
    </div>
  {/if}
</slot>
