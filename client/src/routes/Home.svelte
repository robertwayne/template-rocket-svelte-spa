<script lang="ts">
    import { onMount } from "svelte"

    onMount(async () => {
        const source = new EventSource("http://127.0.0.1:3000/events")

        source.addEventListener("message", (event) => {
            console.log(event)
        })

        window.addEventListener("beforeunload", () => {
            source.close()
        })
    })
</script>

<div class="flex flex-col items-center">
    <h1 class="p-4 text-2xl font-bold">Home Page!</h1>
    <p>Welcome to the home page!</p>
</div>
