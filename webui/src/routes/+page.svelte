<script>
    import { onDestroy } from "svelte";

    let count = 0;

    const evtSource = new EventSource("/sse");

    evtSource.onmessage = (event) => {
        count = JSON.parse(event.data);
    };

    onDestroy(() => {
        evtSource.close();
    });
</script>

<style>
    main {
        font-family: sans-serif;
        text-align: center;
        color: #15222e
    }
</style>

<main>
    <h1>Hello Rust+Svelte</h1>
    <h2>Demonstrates the integration of svelte into a rust axum web-service</h2>
    <p>Counter: {count}</p>
</main>