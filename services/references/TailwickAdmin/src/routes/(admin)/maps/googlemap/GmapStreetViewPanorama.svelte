<script>
  import { onMount } from "svelte";

  onMount(() => {
    setTimeout(() => {
      const myLatLng = { lat: 42.3455, lng: -71.0983 };
      // Create a panorama
      const panorama = new google.maps.StreetViewPanorama(
        document.getElementById("Panorama"),
        {
          position: myLatLng,
          pov: { heading: 165, pitch: 0 },
          zoom: 1,
        }
      );
    }, 500);
  });
</script>

<svelte:head>
    <script src="https://maps.googleapis.com/maps/api/js?key=AIzaSyCtSAR45TFgZjOs4nBFFZnII-6mMHLfSYI"></script>
</svelte:head>

<div id="Panorama" class="gmaps" />
