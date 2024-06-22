<svelte:head>
    <script src="https://maps.googleapis.com/maps/api/js?key=AIzaSyCtSAR45TFgZjOs4nBFFZnII-6mMHLfSYI"></script>
</svelte:head>
<script>
  import { onMount } from "svelte";

  onMount(() => {
    setTimeout(() => {
      const myLatLng = { lat: -12.043333, lng: -77.028333 };
      const map = new google.maps.Map(document.getElementById("mapoverlay"), {
        zoom: 15,
        center: myLatLng,
      });

      // Create a custom overlay using OverlayView
      function CustomOverlay(map, position, content) {
        this.map = map;
        this.position = position;
        this.content = content;
        this.div = null;

        this.setMap(map);
      }

      CustomOverlay.prototype = new google.maps.OverlayView();

      CustomOverlay.prototype.onAdd = function () {
        const div = document.createElement("div");
        div.style.position = "absolute";
        div.innerHTML = this.content;

        this.div = div;

        const panes = this.getPanes();
        panes.overlayLayer.appendChild(div);
      };

      CustomOverlay.prototype.draw = function () {
        const overlayProjection = this.getProjection();
        const position = overlayProjection.fromLatLngToDivPixel(this.position);

        this.div.style.left = position.x + "px";
        this.div.style.top = position.y + "px";
      };

      CustomOverlay.prototype.onRemove = function () {
        this.div.parentNode.removeChild(this.div);
        this.div = null;
      };

      // Usage of the custom overlay
      const customOverlay = new CustomOverlay(
        map,
        new google.maps.LatLng(myLatLng.lat, myLatLng.lng),
        '<div class="gmaps-overlay">Lima<div class="gmaps-overlay_arrow above"></div></div>'
      );
    }, 500);
  });
</script>


<div id="mapoverlay" class="gmaps" />
