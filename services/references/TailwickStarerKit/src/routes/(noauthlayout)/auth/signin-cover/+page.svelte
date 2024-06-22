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

<HeadTitle title="Sign In" />

<div class="mb-0 border-none lg:w-[500px] card bg-white/70 shadow-none dark:bg-zink-500/70">
	<div class="!px-10 !py-12 card-body">
		<a href="/">
			<img src="/assets/images/logo-light.png" alt="" class="hidden h-6 mx-auto dark:block" />
			<img src="/assets/images/logo-dark.png" alt="" class="block h-6 mx-auto dark:hidden" />
		</a>

		<div class="mt-8 text-center">
			<h4 class="mb-2 text-purple-500 dark:text-purple-500">Welcome Back !</h4>
			<p class="text-slate-500 dark:text-zink-200">Sign in to continue to Tailwick.</p>
		</div>

		<form action="#!" class="mt-10" id="signInForm">
			<div
				class="hidden px-4 py-3 mb-3 text-sm text-green-500 border border-green-200 rounded-md bg-green-50 dark:bg-green-400/20 dark:border-green-500/50"
				id="successAlert"
			>
				You have <b>successfully</b> signed in.
			</div>
			<div class="mb-3">
				<label for="username" class="inline-block mb-2 text-base font-medium"
					>UserName/ Email ID</label
				>
				<input
					type="text"
					id="username"
					class="form-input dark:bg-zink-600/50 border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200"
					placeholder="Enter username or email"
				/>
				<div id="username-error" class="hidden mt-1 text-sm text-red-500">
					Please enter a valid email address.
				</div>
			</div>
			<div class="mb-3">
				<label for="password" class="inline-block mb-2 text-base font-medium">Password</label>
				<input
					type="password"
					id="password"
					class="form-input dark:bg-zink-600/50 border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200"
					placeholder="Enter password"
				/>
				<div id="password-error" class="hidden mt-1 text-sm text-red-500">
					Password must be at least 8 characters long and contain both letters and numbers.
				</div>
			</div>
			<div>
				<div class="flex items-center gap-2">
					<input
						id="checkboxDefault1"
						class="size-4 border rounded-sm appearance-none bg-slate-100 border-slate-200 dark:bg-zink-600/50 dark:border-zink-500 checked:bg-custom-500 checked:border-custom-500 dark:checked:bg-custom-500 dark:checked:border-custom-500 checked:disabled:bg-custom-400 checked:disabled:border-custom-400"
						type="checkbox"
						value=""
					/>
					<label
						for="checkboxDefault1"
						class="inline-block text-base font-medium align-middle cursor-pointer"
						>Remember me</label
					>
				</div>
				<div id="remember-error" class="hidden mt-1 text-sm text-red-500">
					Please check the "Remember me" before submitting the form.
				</div>
			</div>
			<div class="mt-10">
				<button
					type="submit"
					class="w-full text-white btn bg-custom-500 border-custom-500 hover:text-white hover:bg-custom-600 hover:border-custom-600 focus:text-white focus:bg-custom-600 focus:border-custom-600 focus:ring focus:ring-custom-100 active:text-white active:bg-custom-600 active:border-custom-600 active:ring active:ring-custom-100 dark:ring-custom-400/20"
					>Sign In</button
				>
			</div>

			<div
				class="relative text-center my-9 before:absolute before:top-3 before:left-0 before:right-0 before:border-t before:border-t-slate-200 dark:before:border-t-zink-500"
			>
				<h5
					class="inline-block px-2 py-0.5 text-sm bg-white text-slate-500 dark:bg-zink-600 dark:text-zink-200 rounded relative"
				>
					Sign In with
				</h5>
			</div>

			<div class="flex flex-wrap justify-center gap-2">
				<button
					type="button"
					class="flex items-center justify-center size-[37.5px] transition-all duration-200 ease-linear p-0 text-white btn bg-custom-500 border-custom-500 hover:text-white hover:bg-custom-600 hover:border-custom-600 focus:text-white focus:bg-custom-600 focus:border-custom-600 active:text-white active:bg-custom-600 active:border-custom-600"
					><LucideIcon name="Facebook" class="size-4" /></button
				>
				<button
					type="button"
					class="flex items-center justify-center size-[37.5px] transition-all duration-200 ease-linear p-0 text-white btn bg-orange-500 border-orange-500 hover:text-white hover:bg-orange-600 hover:border-orange-600 focus:text-white focus:bg-orange-600 focus:border-orange-600 active:text-white active:bg-orange-600 active:border-orange-600"
					><LucideIcon name="Mail" class="size-4" /></button
				>
				<button
					type="button"
					class="flex items-center justify-center size-[37.5px] transition-all duration-200 ease-linear p-0 text-white btn bg-sky-500 border-sky-500 hover:text-white hover:bg-sky-600 hover:border-sky-600 focus:text-white focus:bg-sky-600 focus:border-sky-600 active:text-white active:bg-sky-600 active:border-sky-600"
					><LucideIcon name="Twitter" class="size-4" /></button
				>
				<button
					type="button"
					class="flex items-center justify-center size-[37.5px] transition-all duration-200 ease-linear p-0 text-white btn bg-slate-500 border-slate-500 hover:text-white hover:bg-slate-600 hover:border-slate-600 focus:text-white focus:bg-slate-600 focus:border-slate-600 active:text-white active:bg-slate-600 active:border-slate-600"
					><LucideIcon name="Github" class="size-4" /></button
				>
			</div>

			<div class="mt-10 text-center">
				<p class="mb-0 text-slate-500 dark:text-zink-200">
					Don't have an account ? <a
						href="/auth/signup-basic"
						class="font-semibold underline transition-all duration-150 ease-linear text-slate-500 dark:text-zink-200 hover:text-custom-500 dark:hover:text-custom-500"
					>
						SignUp</a
					>
				</p>
			</div>
		</form>
	</div>
</div>
