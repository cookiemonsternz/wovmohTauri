<script lang="ts">
    import { BaseEdge, getBezierPath, EdgeLabel, type EdgeProps, useEdges } from "@xyflow/svelte";

    let { id, sourceX, sourceY, targetX, targetY }: EdgeProps = $props();

    let [edgePath, labelX, labelY] = $derived(
        getBezierPath({
            sourceX,
            sourceY,
            targetX,
            targetY,
        })
    );

    const edges = useEdges();

    function onDeleteClicked() {
        edges.update((eds) => eds.filter((edge) => edge.id !== id));
    }
</script>

<BaseEdge {id} path={edgePath} />
<EdgeLabel x={labelX} y={labelY} style="background:transparent">
    <button class="nodrag nopan" id="delete-connection-button" onclick={ onDeleteClicked } aria-label="delete connection">
        <img id="button-img" src="/assets/cancel.svg" alt="delete connection"/>
    </button>
</EdgeLabel>

<style>
    #delete-connection-button {
        padding: 0;
        margin: 0;
        border-radius: 50%;
        max-height: 16px;
        max-width: 16px;

        background: #00000017;
        border: none;
    }

    #delete-connection-button:hover {
        cursor: pointer;
    }

    #button-img {
        width: 16px;
        height: 16px;

        opacity: 0;
        transition: opacity 0.5s;
    }

    #button-img:hover {
        opacity: 1;

        transition: opacity 0.5s;
    }
</style>