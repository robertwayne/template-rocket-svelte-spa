import type { Writable } from "svelte/store"

let stores: Record<string, Writable<unknown>> = {}

export function registerStore<T>(id: string, store: Writable<T>): void {
    stores[id] = store
}

// preserve the store across HMR updates
if (import.meta.hot) {
    if (import.meta.hot.data.stores) {
        stores = import.meta.hot.data.stores
    }
    import.meta.hot.accept()
    import.meta.hot.dispose(() => {
        if (import.meta.hot) {
            import.meta.hot.data.stores = stores
        }
    })
}
