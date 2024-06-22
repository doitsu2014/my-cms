<script>
	export let value = 0; // Set the default value
	export let className = '';
	export let striped;
	export let animate;
	export let max = 100;
	// export let color = 'bg-gray-50';
	// export let barColor = 'bg-violet-500'; // Set the default bar color
	export let barClass;

	$: percent = (parseInt(value, 10) / parseInt(max, 10)) * 100;
	barClass = striped ? barClass + ' progress-bar-striped' : barClass;
	barClass = animate ? barClass + 'animate-progress' : barClass;
</script>

<div class="w-full bg-slate-200 rounded-full h-2.54 dark:bg-zink-600  {className}">
	<div class="rounded-full {barClass}" style="width: {percent}%;" role="progressbar">
		<slot />
	</div>
</div>
