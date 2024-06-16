<script>
    /* Component properties */
    export let text = "";
    export let fct = null;
</script>

<!----------------------------------------------------------------------------->
<button on:click={fct}>
    {text}
</button>

<!----------------------------------------------------------------------------->
<style>
    button {
        display: inline-block;

        border: 3px solid var(--lightblue);
        border-radius: 7px;

        background-color: var(--bg);
        color: var(--lightblue);

        padding: 12px;
        margin: 10px;

        font-size: 20px;
    }

    button:hover {
        transition: 0.15s;

        cursor: pointer;

        background-color: var(--lightblue);
        color: var(--bg);
    }
</style>
