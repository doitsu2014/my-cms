<script>
	import HeadTitle from '../../../../common/components/HeadTitle.svelte';
	import LucideIcon from '../../../../common/components/LucideIcon.svelte';

	import { classList } from 'svelte-body';
	import { browser } from '$app/environment';

	if (browser) {
		document.body.setAttribute('class', '');
	}
</script>

<svelte:body
	use:classList={'flex items-center justify-center min-h-screen py-16 bg-cover bg-auth-pattern dark:bg-auth-pattern-dark dark:text-zink-100 font-public'}
/>

<HeadTitle title="Log Out" />

<div class="mb-0 border-none lg:w-[500px] card bg-white/70 shadow-none dark:bg-zink-500/70">
	<div class="!px-10 !py-12 card-body">
		<a href="/">
			<img src="/assets/images/logo-light.png" alt="" class="hidden h-6 mx-auto dark:block" />
			<img src="/assets/images/logo-dark.png" alt="" class="block h-6 mx-auto dark:hidden" />
		</a>

		<div class="mt-8 text-center">
			<div class="mb-4 text-center">
				<LucideIcon name="LogOut" class="size-6 mx-auto text-purple-500 fill-purple-100" />
			</div>
			<h4 class="mb-2 text-custom-500 dark:text-custom-500">You are Logged Out</h4>
			<p class="mb-8 text-slate-500 dark:text-zink-200">
				Thank you for using tailwick admin template
			</p>
		</div>

		<a
			href="/auth/signin-basic"
			class="w-full text-white transition-all duration-200 ease-linear btn bg-custom-500 border-custom-500 hover:text-white hover:bg-custom-600 hover:border-custom-600 focus:text-white focus:bg-custom-600 focus:border-custom-600 focus:ring focus:ring-custom-100 active:text-white active:bg-custom-600 active:border-custom-600 active:ring active:ring-custom-100 dark:ring-custom-400/20"
			>Sign In</a
		>
	</div>
</div>
