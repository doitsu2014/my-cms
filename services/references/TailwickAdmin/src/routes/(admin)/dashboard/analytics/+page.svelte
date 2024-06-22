<script>
	import HeadTitle from '../../../../common/components/HeadTitle.svelte';
	import Breadcrumb from '../../../../common/components/Breadcrumb.svelte';
	import Widget from './Widget.svelte';
	import CounterPage from "../../../../common/components/CounterPage.svelte";
	import LocationBasedResponse from './LocationBasedResponse.svelte';
	import PagesInteractionChart from './PagesInteractionChart.svelte';
	import UserDevice from './UserDevice.svelte';
	import SatisfactionLevel from './SatisfactionLevel.svelte';
	import DailyVisitInsights from './DailyVisitInsights.svelte';
	import ProductsStatistics from './ProductsStatistics.svelte';
	import AnalyticsReports from './AnalyticsReports.svelte';
	import MonthlyCampaignStatus from './MonthlyCampaignStatus.svelte';
	import SubscriptionDistribution from './SubscriptionDistribution.svelte';
	import TrafficSource from './TrafficSource.svelte';
</script>

<HeadTitle title="Analytics" />
<CounterPage />

<div class="container-fluid group-data-[content=boxed]:max-w-boxed mx-auto">
	<Breadcrumb title="Analytics" pagetitle="Dashboards" />

    <div class="grid grid-cols-12 gap-x-5">
        <Widget />

        <LocationBasedResponse />

        <PagesInteractionChart />

        <UserDevice />

        <SatisfactionLevel />

        <DailyVisitInsights />

        <ProductsStatistics />

        <AnalyticsReports chartColors={["bg-custom-500", "bg-green-500"]}/>

        <MonthlyCampaignStatus />

        <SubscriptionDistribution chartColors={["bg-custom-500", "bg-orange-500", "bg-green-500", "bg-yellow-500", "bg-purple-500"]}/>

        <TrafficSource />
    </div>
</div>
