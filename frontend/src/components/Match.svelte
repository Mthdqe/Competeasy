<script>
    export let first_team = "";
    export let second_team = "";
    export let date = "";
    export let hour = "";
    export let place = "";
    export let match_score = { lhs: 0, rhs: 0 };
</script>

<div>
    <h2>{first_team} - {second_team}</h2>
    <p><span>Date:</span> {date}</p>
    <p><span>Heure:</span> {hour}</p>
    {#if place === ""}
        <p><span>Score:</span> {match_score.lhs} - {match_score.rhs}</p>
    {:else}
        <p><span>Lieu:</span> {place}</p>
    {/if}
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

    div h2 {
        text-align: center;
        font-size: 18px;
        font-weight: bold;
    }

    div p {
        color: var(--fg);
    }

    div span {
        font-weight: bold;
        text-decoration: underline;
    }
</style>
