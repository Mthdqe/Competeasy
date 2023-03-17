<script>
    export let name = "";
    export let url = "";

    let regions = [];

    function onClick() {
        let uri = "http://localhost:8000/regions?value=";
        console.log(encodeURIComponent(url));
        fetch(uri + encodeURIComponent(url)).then((response) => {
            response.json().then((json) => {
                regions = json;
            });
        });
    }
</script>

<div>
    <button on:click={onClick}>
        <p>{name}</p>
    </button>
</div>

<style>
    div {
        display: inline-block;
        width: 400px;
        background-color: var(--blue);
        margin: 15px;
        padding: 10px;
        border-radius: 10px;
    }

    div button {
        background-color: var(--blue);
    }

    div button p {
        color: var(--fg);
    }
</style>
