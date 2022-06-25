<script lang="ts">
    import { Route } from "svelte-navigator"
    import AsyncRouteGuard from "./AsyncRouteGuard.svelte"
    import type { RouteProps } from "svelte-navigator/types/Route"
    import type { SvelteComponentDev } from "svelte/internal"
    import type AnyObject from "svelte-navigator/types/AnyObject"

    export let component: () => Promise<{
        default: typeof SvelteComponentDev | null
    }>
    export let delay = 500

    let props: svelte.JSX.IntrinsicAttributes & RouteProps<AnyObject>

    $: {
        // eslint-disable-next-line @typescript-eslint/no-unused-vars
        const { component, ...restProps } = $$props
        props = restProps
    }
</script>

<Route {...props}>
    <AsyncRouteGuard {component} {delay} let:focus>
        <slot />
    </AsyncRouteGuard>
</Route>
