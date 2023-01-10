<script lang="ts">
    import { Route } from "svelte-navigator"
    import LazyRouteGuard from "./LazyRouteGuard.svelte"
    import type { RouteProps } from "svelte-navigator/types/Route"
    import type { SvelteComponentDev } from "svelte/internal"
    import type AnyObject from "svelte-navigator/types/AnyObject"

    export let component: () => Promise<{
        default: typeof SvelteComponentDev | null
    }>
    export let delay = 500

    let props: svelte.JSX.IntrinsicAttributes & RouteProps<AnyObject>

    $: {
        // eslint-disable-next-line
        const { component, ...restProps } = $$props
        props = restProps
    }
</script>

<Route {...props}>
    <LazyRouteGuard {component} {delay} let:focus>
        <slot {focus} />
    </LazyRouteGuard>
</Route>
