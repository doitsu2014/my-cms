<script>
    import HeadTitle from "../../../../common/components/HeadTitle.svelte";
	import { OverlayScrollbars } from 'overlayscrollbars';
	import { onMount } from "svelte";
	import Breadcrumb from "../../../../common/components/Breadcrumb.svelte";
    import * as FullCalendar from "fullcalendar";
	import CalendarEventsList from "../../../../common/data/calender";
	import Modal from "../../../../common/components/Modal.svelte";
	import LucideIcon from "../../../../common/components/LucideIcon.svelte";


    let calendar;
    let options;
	let calendarEl;
    let eventsList = CalendarEventsList;
    let setSelectedDay = new Date();
	let isedit = false;
    let setEvent = {};
    
    let isEventOpen = false;
    const toggleEventModal = () => {
        isEventOpen = !isEventOpen;
    };

    onMount(() => {
    const checkbox = document.getElementById('drop-remove');

		calendarEl = document.getElementById("calendar");
		options = {
			timeZone: "local",
			editable: true,
			droppable: true,
			selectable: true,
			navLinks: true,
			initialView: "multiMonthYear",
			themeSystem: "tailwindcss",
			headerToolbar: {
				left: "prev,next,today",
				center: "title",
				right: "dayGridMonth,timeGridWeek,timeGridDay,listMonth",
			},
			events: eventsList.events,
			windowResize: function (view) {

			},
			eventClick: (event) => handleEventClick(event),
			dateClick: (event) => handleDateClick(event.dateStr),
			eventReceive: function (info) {
				var newid = parseInt(info.event.id);
				var newEvent = {
					id: newid,
					title: info.event.title,
					start: info.event.start,
					allDay: info.event.allDay,
					className: info.event.classNames,
				};
				eventsList.events = [...eventsList.events, newEvent];
				updateCalendarEvents();
			},
            drop: (info) => checkbox.checked && info.draggedEl.parentNode.removeChild(info.draggedEl),

		};
		// Initialize FullCalendar
		calendar = new FullCalendar.Calendar(calendarEl, options);

		calendar.render(); // Render the calendar
    })

    function updateCalendarEvents() {
		// Reassign the events property with the updated events data
		options.events = eventsList.events;
		calendar.destroy(); // Destroy the existing instance
		calendar = new FullCalendar.Calendar(calendarEl, options); // Reinitialize with updated options
		calendar.render();
		calendar.refetchEvents(); // Refresh the calendar with the updated events
	}

    function handleDateClick(arg) {
        isedit = false;
		setSelectedDay = new Date(arg);
		toggleEventModal();
	}

    const handleValidEventSubmit = async ({
		target: {
			elements: { category, title },
		},
	}) => {
		if (isedit) {
			if (
				title.value == null ||
				title.value == undefined ||
				title.value == ""
			) {
				document.getElementById("divAlert").style.display = "block";
				return false;
			}
			const updateEvent = {
				id: setEvent.id,
				title: title.value,
				className: category.value,
				start: setSelectedDay,
				allDay: false,
			};

			const i = eventsList.events.findIndex(
				(t) => t.id === updateEvent.id
			);
			// update event
			eventsList.events[i] = updateEvent;
			eventsList.events = [...eventsList.events];

			updateCalendarEvents();
		} else {
			if (
				title.value == null ||
				title.value == undefined ||
				title.value == ""
			) {
				document.getElementById("divAlert").style.display = "block";
				return false;
			}
			let newEvent = {
				id: Math.floor(Math.random() * 100),
				title: title.value,
				start: setSelectedDay ? setSelectedDay : new Date(),
				allDay: false,
				className: category.value,
			};
			eventsList.events = [...eventsList.events, newEvent];
            calendar.addEvent(newEvent)
			// updateCalendarEvents();
		}

		setSelectedDay = "";
		toggleEventModal();
	};

    const handleEventClick = (arg) => {
		const event = arg.event;
		setEvent = {
			id: parseInt(event.id),
			title: event.title,
			title_category: event.title_category,
			start: event.start,
			className: event.classNames,
			category: event.classNames[0],
			// event_category: event.className[0],
		};
        setSelectedDay = setEvent.start;
		isedit = true;
		toggleEventModal();
	};

    const setDeleteModal = (status) => {
		isEventOpen = false;

		var calendarEvents = CalendarEventsList.events.filter((e, i) => {
			return e.id !== setEvent.id;
		});

		eventsList.events = calendarEvents;
		updateCalendarEvents();
		setTimeout(() => {
			setEvent = {};
			isedit = false;
		}, 500);
	};
    
</script>
<HeadTitle title="Calendar Month Grid" />

<div class="container-fluid group-data-[content=boxed]:max-w-boxed mx-auto relative">
    <Breadcrumb title="Monthly Calendar" pagetitle="Calendar"/>

    <div class="grid grid-cols-1 gap-x-5 xl:grid-cols-12">
        <div class="xl:col-span-12">
            <div class="card">
                <div class="card-body">
                    <div cursor-pointerid='calendar-container'>
                        <button type="hidden" id="calendarBtn"></button>
                        <div id='calendar'></div>
                    </div>
                </div>
            </div>
        </div><!--end col-->
       
        <!--end col-->
    </div>
</div>


<Modal modal-center className="-translate-y-2/4" isOpen={isEventOpen} toggle={toggleEventModal}>
    <div class="w-screen md:w-[30rem] bg-white shadow rounded-md dark:bg-zink-600">
        <div class="flex items-center justify-between p-4 border-b dark:border-zink-500">
            <h5 class="text-16" id="modal-title">{ isedit ? "Edit Event" : "Add Event"}</h5>
            <button on:click={toggleEventModal} id="eventModal-close" class="transition-all duration-200 ease-linear text-slate-400 hover:text-red-500"><LucideIcon name="X" class="size-5"/></button>
        </div>
        <div class="max-h-[calc(theme('height.screen')_-_180px)] p-4 overflow-y-auto">
            <form class="needs-validation" name="event-form" id="form-event" autocomplete="off" on:submit|preventDefault={handleValidEventSubmit}>
                <div class="grid grid-cols-1 gap-4 xl:grid-cols-12">
                    <div class="xl:col-span-12">
                        <label for="event-title" class="inline-block mb-2 text-base font-medium">Event Name</label>
                        <input type="text" name="title" id="event-title" value={isedit && setEvent && setEvent.title
                            ? setEvent.title
                            : ""} class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200" placeholder="Event name" required>
                    </div>
                    <div class="xl:col-span-12">
                        <label for="event-category" class="inline-block mb-2 text-base font-medium">Category:</label>
                        <select required name="category" value={setEvent ? setEvent.category : ""} class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200" id="event-category">
                            <option >Select Category</option>
                            <option selected value="fc-event fc-h-event fc-daygrid-event fc-daygrid-block-event transition-all w-[100%] text-custom-500 !bg-custom-100 dark:!bg-custom-500/20 border-none rounded-md py-1.5 px-3">Primary</option>
                            <option value="fc-event fc-h-event fc-daygrid-event fc-daygrid-block-event text-green-500 w-[100%] !bg-green-100 dark:!bg-green-500/20 border-none rounded-md py-1.5 px-3">Success</option>
                            <option value="fc-event fc-h-event fc-daygrid-event fc-daygrid-block-event text-sky-500 w-[100%] !bg-sky-100 dark:!bg-sky-500/20 border-none rounded-md py-1.5 px-3">Info</option>
                            <option value="fc-event fc-h-event fc-daygrid-event fc-daygrid-block-event text-yellow-500 w-[100%] !bg-yellow-100 dark:!bg-yellow-500/20 border-none rounded-md py-1.5 px-3">Warning</option>
                            <option value="fc-event fc-h-event fc-daygrid-event fc-daygrid-block-event w-[100%] text-purple-500 !bg-purple-100 dark:!bg-purple-500/20 border-none rounded-md py-1.5 px-3">Purple</option>
                        </select>
                    </div>
                </div>
                <div class="flex justify-end gap-2 mt-4">
                    <button type="reset" class="text-red-500 bg-white btn hover:text-red-500 hover:bg-red-100 focus:text-red-500 focus:bg-red-100 active:text-red-500 active:bg-red-100 dark:bg-zink-600 dark:hover:bg-red-500/10 dark:focus:bg-red-500/10 dark:active:bg-red-500/10" on:click={toggleEventModal}>Cancel</button>
                    <button type="button" on:click={() => setDeleteModal(true)} id="btn-delete-event" class="{ !isedit ? "hidden":""} text-white bg-red-500 border-red-500 btn hover:text-white hover:bg-red-600 hover:border-red-600 focus:text-white focus:bg-red-600 focus:border-red-600 focus:ring focus:ring-red-100 active:text-white active:bg-red-600 active:border-red-600 active:ring active:ring-red-100 dark:ring-custom-400/20">Delete</button>
                    <button type="submit" id="btn-save-event" class="text-white btn bg-custom-500 border-custom-500 hover:text-white hover:bg-custom-600 hover:border-custom-600 focus:text-white focus:bg-custom-600 focus:border-custom-600 focus:ring focus:ring-custom-100 active:text-white active:bg-custom-600 active:border-custom-600 active:ring active:ring-custom-100 dark:ring-custom-400/20">Save</button>
                </div>
            </form>
        </div>
    </div>
</Modal>