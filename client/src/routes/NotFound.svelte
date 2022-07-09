<script lang="ts">
    import { navigate } from "svelte-navigator"
</script>

<div class="flex flex-col items-center">
    <h1 class="p-4 text-2xl font-bold">404 Not Found</h1>
    <p class="p-4">The page you're looking for doesn't exist!</p>

    <button on:click={() => navigate("/", { replace: true })}
        ><span
            class="font-bold underline hover:text-blue-500 hover:duration-300"
            >Go Home</span
        ></button
    >
</div>
