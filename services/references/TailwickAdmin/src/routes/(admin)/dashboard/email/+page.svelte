<script>
    import HeadTitle from "../../../../common/components/HeadTitle.svelte";
    import Breadcrumb from "../../../../common/components/Breadcrumb.svelte";
    import CounterPage from "../../../../common/components/CounterPage.svelte";
	import Widget from "./Widget.svelte";
	import Widget2 from "./Widget2.svelte";
	import EmailData from "./EmailData.svelte";
	import EmailMarketing from "./EmailMarketing.svelte";
	import EmailPerformance from "./EmailPerformance.svelte";
</script>

<HeadTitle title="Email Analytics" />
<CounterPage />

<div class="container-fluid group-data-[content=boxed]:max-w-boxed mx-auto">
    <Breadcrumb title="Email Analytics" pagetitle="Dashboards"/>
    <div class="grid grid-cols-12 2xl:grid-cols-12 gap-x-5">
        <Widget />
        <Widget2 />
        <EmailData />
        <EmailMarketing />
        <EmailPerformance />
    </div>
</div>