<script lang="ts">
    let {data} = $props();
    $effect(() => {
        console.log(data);
    });
</script>
<p>hello, {JSON.stringify(data)}!</p>
