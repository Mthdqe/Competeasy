<script>
    export let first_team = "DJAM";
    export let second_team = "CLAMART2";
    export let date = "19/11/2022";
    export let hour = "20:30";
    export let place = "GYMNASE DE LA CITADELLE";
</script>

<div>
    <h2>{first_team} - {second_team}</h2>
    <p><span>Date:</span> {date}</p>
    <p><span>Heure:</span> {hour}</p>
    <p><span>Lieu:</span> {place}</p>
</div>

<style>
    div {
        display: inline-block;
        width: 400px;
        background-color: #e5a50a;
        margin: 15px;
        padding: 10px;
    }

    div h2 {
        text-align: center;
        font-size: 18px;
        font-weight: bold;
    }

    div p {
        color: white;
    }

    div span {
        text-decoration: underline;
    }
</style>
