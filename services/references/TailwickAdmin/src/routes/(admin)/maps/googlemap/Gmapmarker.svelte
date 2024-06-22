<script>
    import { onMount } from "svelte";

    onMount(() => {
        setTimeout(() => {
            const myLatLng = { lat: -12.043333, lng: -77.028333 };
            const map = new google.maps.Map(document.getElementById("map"), {
                zoom: 15,
                center: myLatLng,
            });

            // marker
            new google.maps.Marker({
                position: myLatLng,
                map,
                title: "Hello World!",
            });
        }, 500);
    });
</script>

<svelte:head>
    <script src="https://maps.googleapis.com/maps/api/js?key=AIzaSyCtSAR45TFgZjOs4nBFFZnII-6mMHLfSYI"></script>
</svelte:head>

<div id="map" class="gmaps" />
