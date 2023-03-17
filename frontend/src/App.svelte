<script>
    import Competition from "./components/Competition.svelte";
    import Match from "./components/Match.svelte";

    let matchs = [];

    let competitions = [];

    let team = "";

    function onClick() {
        let uri = "http://localhost:8000/competitions";
        fetch(uri).then((response) => {
            response.json().then((json) => {
                competitions = json;
            });
        });
    }
</script>

<main>
    <div id="input">
        <input type="value" bind:value={team} />
        <button on:click={onClick}>Get</button>
    </div>
    <div class="list">
        {#each matchs as match}
            <Match
                first_team={match.first_team}
                second_team={match.second_team}
                date={match.date}
                hour={match.hour}
                place={match.place}
                match_score={match.match_score}
            />
        {/each}
    </div>
    <div class="list">
        {#each competitions as competition}
            <Competition name={competition.name} url={competition.url} />
        {/each}
    </div>
</main>

<style>
    main {
        width: 1200px;
        margin: auto;
    }

    .list {
        display: flex;
        flex-wrap: wrap;
        justify-content: center;
    }

    #input {
        display: flex;
        justify-content: center;
        width: 300px;
        margin: auto;
    }
</style>
