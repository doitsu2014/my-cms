<script>
  import { onMount } from "svelte";

  onMount(() => {
    setTimeout(() => {
      const map = new google.maps.Map(document.getElementById("maptype"), {
        center: { lat: -12.043333, lng: -77.028333 },
        zoom: 16,
        mapTypeControlOptions: {
          mapTypeIds: ["hybrid", "roadmap", "satellite", "terrain", "osm"],
        },
      });

      map.mapTypes.set(
        "osm",
        new google.maps.ImageMapType({
          getTileUrl: function (coord, zoom) {
            return (
              "https://a.tile.openstreetmap.org/" +
              zoom +
              "/" +
              coord.x +
              "/" +
              coord.y +
              ".png"
            );
          },
          tileSize: new google.maps.Size(256, 256),
          name: "OpenStreetMap",
          maxZoom: 18,
        })
      );

      map.setMapTypeId("osm");
    }, 500);
  });
</script>

<svelte:head>
    <script src="https://maps.googleapis.com/maps/api/js?key=AIzaSyCtSAR45TFgZjOs4nBFFZnII-6mMHLfSYI"></script>
</svelte:head>

<div id="maptype" class="gmaps" />
