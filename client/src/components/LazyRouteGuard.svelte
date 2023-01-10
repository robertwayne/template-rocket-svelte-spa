<script lang="ts">
    import { onMount } from "svelte"
    import { useFocus } from "svelte-navigator"
    import type { SvelteComponentDev } from "svelte/internal"

    export let component: () => Promise<{
        default: typeof SvelteComponentDev | null
    }>
    export let delay = 500

    const focus = useFocus()

    let loadedComponent: typeof SvelteComponentDev | null
    let timeout: NodeJS.Timeout
    let showFallback = !delay
    let props: svelte.JSX.IntrinsicAttributes & { [name: string]: unknown }

    $: {
        // eslint-disable-next-line
        const { component, delay, ...restProps } = $$props
        props = restProps
    }

    // Any page wrapped with this guard will be loaded lazily. During its load
    // time, whatever component/text is in between the tags will be displayed
    // (think loading animation / text). That loading 'content' will have a
    // half-second delay so we don't have pages flashing content while
    // downloading their payload.
    onMount(() => {
        if (delay) {
            timeout = setTimeout(() => {
                showFallback = true
            }, delay)
        }

        component().then(
            (module: { default: typeof SvelteComponentDev | null }) => {
                loadedComponent = module.default
            }
        )

        return () => clearTimeout(timeout)
    })
</script>

{#if loadedComponent}
    <svelte:component this={loadedComponent} {...props} />
{:else if showFallback}
    <slot {focus} />
{/if}
