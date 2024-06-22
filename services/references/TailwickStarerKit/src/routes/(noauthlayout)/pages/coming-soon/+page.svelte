<script>
	import HeadTitle from '../../../../common/components/HeadTitle.svelte';
	import { classList } from 'svelte-body';
	import { browser } from '$app/environment';
	import { onMount } from 'svelte';

	if (browser) {
		document.body.setAttribute('class', '');
	}

	onMount(() => {
		var mainArray = Array.from(document.querySelectorAll('.countdownlist'));
		mainArray.forEach(function (item) {
			var countdownVal = new Date().getTime() + 12000; // item.getAttribute("data-countdown") // You may use specific date for long duration of coming soon time.

			// Set the date we're counting down to
			var countDownDate = new Date(countdownVal).getTime();

			// Update the count down every 1 second
			var countDown = setInterval(function () {
				// Get today's date and time
				var currentTime = new Date().getTime();
				// Find the distance between currentTime and the count down date
				var distance = countDownDate - currentTime;

				// Time calculations for days, hours, minutes and seconds
				var days = Math.floor(distance / (1000 * 60 * 60 * 24));
				var hours = Math.floor((distance % (1000 * 60 * 60 * 24)) / (1000 * 60 * 60));
				var minutes = Math.floor((distance % (1000 * 60 * 60)) / (1000 * 60));
				var seconds = Math.floor((distance % (1000 * 60)) / 1000);

				var countDownBlock =
					'<div class="flex flex-col items-center justify-center size-24 p-2 text-center bg-white rounded dark:bg-zink-600/50 countdownlist-item">' +
					'<div class="count-title text-slate-500 dark:text-zink-200">Days</div>' +
					'<div class="mt-2 text-2xl font-semibold count-num dark:text-zink-50">' +
					days +
					'</div>' +
					'</div>' +
					'<div class="flex flex-col items-center justify-center size-24 p-2 text-center bg-white rounded dark:bg-zink-600/50 countdownlist-item">' +
					'<div class="count-title text-slate-500 dark:text-zink-200">Hours</div>' +
					'<div class="mt-2 text-2xl font-semibold count-num dark:text-zink-50">' +
					hours +
					'</div>' +
					'</div>' +
					'<div class="flex flex-col items-center justify-center size-24 p-2 text-center bg-white rounded dark:bg-zink-600/50 countdownlist-item">' +
					'<div class="count-title text-slate-500 dark:text-zink-200">Minutes</div>' +
					'<div class="mt-2 text-2xl font-semibold count-num dark:text-zink-50">' +
					minutes +
					'</div>' +
					'</div>' +
					'<div class="flex flex-col items-center justify-center size-24 p-2 text-center bg-white rounded dark:bg-zink-600/50 countdownlist-item">' +
					'<div class="count-title text-slate-500 dark:text-zink-200">Seconds</div>' +
					'<div class="mt-2 text-2xl font-semibold count-num dark:text-zink-50">' +
					seconds +
					'</div>' +
					'</div>';

				// Output the result in an element with id="countDownBlock"
				if (item) {
					item.innerHTML = countDownBlock;
				}
				// If the count down is over, write some text
				if (distance < 0) {
					clearInterval(countDown);
					document.getElementById('countDownText').innerHTML = `<div class="text-center">
                <i class="text-4xl text-green-500 ri-verified-badge-line"></i>
                <div class="mt-5 mb-4">
                    <h4 class="mb-2">We've Launched our new website</h4>
                    <p class="text-slate-500 dark:text-zink-200">Click the below button to visit our website.</p>
                </div>
                <a href="/" class="text-white btn bg-custom-500 border-custom-500 hover:text-white hover:bg-custom-600 hover:border-custom-600 focus:text-white focus:bg-custom-600 focus:border-custom-600 focus:ring focus:ring-custom-100 active:text-white active:bg-custom-600 active:border-custom-600 active:ring active:ring-custom-100 dark:ring-custom-400/20">Back to Home</a>
            </div>`;
				}
			}, 1000);
		});
	});
</script>

<svelte:body
	use:classList={'flex items-center justify-center min-h-screen py-16 bg-cover bg-auth-pattern dark:bg-auth-pattern-dark font-public'}
/>

<HeadTitle title="Coming Soon" />

<div class="mb-0 border-none shadow-none lg:w-[500px] card bg-white/70 dark:bg-zink-500/70">
	<div class="!px-10 !py-12 card-body">
		<div id="countDownText">
			<a href="/">
				<img src="/assets/images/logo-light.png" alt="" class="hidden h-6 mx-auto dark:block" />
				<img src="/assets/images/logo-dark.png" alt="" class="block h-6 mx-auto dark:hidden" />
			</a>

			<div class="mt-8 text-center">
				<h4 class="mb-2 text-purple-500 dark:text-purple-500">Coming Soon ...</h4>
				<p class="text-slate-500 dark:text-zink-200">We'll be here in a brief moment.</p>
			</div>

			<div>
				<div class="mt-8">
					<div class="flex items-center justify-between mt-14">
						<div data-countdown="Oct 30, 2025" class="flex flex-wrap gap-3 countdownlist"></div>
					</div>
				</div>

				<div class="mt-10 text-center">
					<h5 class="mb-2">Be alerted when our launch happens.</h5>
					<p class="mb-5 text-slate-500 dark:text-zink-200">
						Don't worry, we won't inundate your inbox 😊
					</p>
					<form action="#!" class="relative">
						<input
							type="email"
							id="inputEmail"
							class="py-2.5 form-input dark:bg-zink-600/50 border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200"
							placeholder="Enter your email"
							required
						/>
						<button
							type="submit"
							class="px-2.5 py-1.5 text-sm text-white btn bg-custom-500 border-custom-500 hover:text-white hover:bg-custom-600 hover:border-custom-600 focus:text-white focus:bg-custom-600 focus:border-custom-600 focus:ring focus:ring-custom-100 active:text-white active:bg-custom-600 active:border-custom-600 active:ring active:ring-custom-100 dark:ring-custom-400/20 absolute top-[5px] ltr:right-1 rtl:left-1"
							>Send</button
						>
					</form>
				</div>
			</div>
		</div>
	</div>
</div>
