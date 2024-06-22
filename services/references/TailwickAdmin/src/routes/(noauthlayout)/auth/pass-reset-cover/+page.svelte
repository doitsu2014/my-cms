<script>
    import HeadTitle from "../../../../common/components/HeadTitle.svelte"
    import Dropdown from "../../../../common/components/Dropdown.svelte"
    import DropdownToggle from "../../../../common/components/DropdownToggle.svelte"

    import { classList } from 'svelte-body';
	import { browser } from '$app/environment';

	if (browser) {
		document.body.setAttribute("class","");
	}
</script>
<svelte:body use:classList={'flex items-center justify-center min-h-screen py-16 bg-cover bg-auth-pattern dark:bg-auth-pattern-dark dark:text-zink-100 font-public'} />

<HeadTitle title="Password Reset" />

<div class="mb-0 border-none lg:w-[500px] card bg-white/70 shadow-none dark:bg-zink-500/70">
    <div class="!px-10 !py-12 card-body">
        <a href="/">
            <img src="/assets/images/logo-light.png" alt="" class="hidden h-6 mx-auto dark:block">
            <img src="/assets/images/logo-dark.png" alt="" class="block h-6 mx-auto dark:hidden">
        </a>

        <div class="mt-8 text-center">
            <h4 class="mb-2 text-custom-500 dark:text-custom-500">Forgot Password?</h4>
            <p class="mb-8 text-slate-500 dark:text-zink-200">Reset your Tailwick password</p>
        </div>
        
        <div class="px-4 py-3 mb-6 text-sm text-yellow-500 border border-transparent rounded-md bg-yellow-50 dark:bg-yellow-400/20">
            Provide your email address, and instructions will be sent to you
        </div>
        
        <form autocomplete="off" action="/">
            <div>
                <label for="emailInput" class="inline-block mb-2 text-base font-medium">Email</label>
                <input type="text" class="form-input dark:bg-zink-600/50 border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200" required placeholder="Enter email" id="emailInput">
            </div>
            <div class="mt-8">
                <button type="submit" class="w-full text-white btn bg-custom-500 border-custom-500 hover:text-white hover:bg-custom-600 hover:border-custom-600 focus:text-white focus:bg-custom-600 focus:border-custom-600 focus:ring focus:ring-custom-100 active:text-white active:bg-custom-600 active:border-custom-600 active:ring active:ring-custom-100 dark:ring-custom-400/20">Send Reset Link</button>
            </div>
            <div class="mt-4 text-center">
                <p class="mb-0">Wait, I remember my password... <a href="/auth/signin-cover" class="underline fw-medium text-custom-500"> Click here </a> </p>
            </div>
        </form>
    </div>
</div>