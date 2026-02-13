<script lang="ts">
    import { Position, useSvelteFlow, type NodeProps } from "@xyflow/svelte";

    let { id }: NodeProps = $props();

    let { updateNodeData } = useSvelteFlow();
</script>

<div class="text-updater-node">
    <div>
        <label for="text">Text:</label>
    </div>
</div>
