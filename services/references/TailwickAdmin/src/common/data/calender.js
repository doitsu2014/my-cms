var date = new Date();
var d = date.getDate();
var m = date.getMonth();
var y = date.getFullYear();

const defaultevent = [
  { id: 1,title: 'All Day Event', start: new Date(y, m, 1), className: 'w-[100%] text-purple-500 !bg-purple-100 dark:!bg-purple-500/20 border-none rounded-md py-1.5 px-3' },
  { id: 2,title: 'Long Event', start: new Date(y, m, d - 5), end: new Date(y, m, d - 2), className: '!text-sky-500 w-[100%] !bg-sky-100 dark:!bg-sky-500/20 border-none rounded-md py-1.5 px-3' },
  { id: 3, title: 'Repeating Event', start: new Date(y, m, d - 3, 16, 0), allDay: false, className: 'text-yellow-500 w-[100%] !bg-yellow-100 dark:!bg-yellow-500/20 border-none rounded-md py-1.5 px-3' },
  { id: 4,title: 'Meeting', start: new Date(y, m, d, 10, 30), allDay: false, className: 'text-green-500 w-[100%] !bg-green-100 dark:!bg-green-500/20 border-none rounded-md py-1.5 px-3' },
  { id: 5,title: 'Lunch', start: new Date(y, m, d, 12, 0), end: new Date(y, m, d, 14, 0), allDay: false, className: 'text-purple-500 w-[100%] !bg-purple-100 dark:!bg-purple-500/20 border-none rounded-md py-1.5 px-3' },
  { id: 6,title: 'Birthday Party', start: new Date(y, m, d + 1, 19, 0), end: new Date(y, m, d + 1, 22, 30), allDay: false, className: 'w-[100%] text-sky-500 !bg-sky-100 dark:!bg-sky-500/20 border-none rounded-md py-1.5 px-3' },
  { id: 7,title: 'Click for Google', start: new Date(y, m, 28), end: new Date(y, m, 29), url: 'http://google.com/', className: 'w-[100%] text-custom-500 !bg-custom-100 dark:!bg-custom-500/20 border-none rounded-md py-1.5 px-3' }

];

const events = [
  { id: 1,title: 'All Day Event', start: new Date(y, m, 1), className: 'w-[100%] text-purple-500 !bg-purple-100 dark:!bg-purple-500/20 border-none rounded-md py-1.5 px-3' },
  { id: 2,title: 'Long Event', start: new Date(y, m, d - 5), end: new Date(y, m, d - 2), className: '!text-sky-500 w-[100%] !bg-sky-100 dark:!bg-sky-500/20 border-none rounded-md py-1.5 px-3' },
  { id: 3, title: 'Repeating Event', start: new Date(y, m, d - 3, 16, 0), allDay: false, className: 'text-yellow-500 w-[100%] !bg-yellow-100 dark:!bg-yellow-500/20 border-none rounded-md py-1.5 px-3' },
  { id: 4,title: 'Meeting', start: new Date(y, m, d, 10, 30), allDay: false, className: 'text-green-500 w-[100%] !bg-green-100 dark:!bg-green-500/20 border-none rounded-md py-1.5 px-3' },
  { id: 5,title: 'Lunch', start: new Date(y, m, d, 12, 0), end: new Date(y, m, d, 14, 0), allDay: false, className: 'text-purple-500 w-[100%] !bg-purple-100 dark:!bg-purple-500/20 border-none rounded-md py-1.5 px-3' },
  { id: 6,title: 'Birthday Party', start: new Date(y, m, d + 1, 19, 0), end: new Date(y, m, d + 1, 22, 30), allDay: false, className: 'w-[100%] text-sky-500 !bg-sky-100 dark:!bg-sky-500/20 border-none rounded-md py-1.5 px-3' },
  { id: 7,title: 'Click for Google', start: new Date(y, m, 28), end: new Date(y, m, 29), url: 'http://google.com/', className: 'w-[100%] text-custom-500 !bg-custom-100 dark:!bg-custom-500/20 border-none rounded-md py-1.5 px-3' }

];

const calenderDefaultCategories = [
  {
    id: 1,
    title: "New Event Planning",
    type: "success",

  },
  {
    id: 2,
    title: "Meeting",
    type: "info",
  },
  {
    id: 3,
    title: "Generating Reports",
    type: "warning",
  },
  {
    id: 4,
    title: "Create New theme",
    type: "danger",
  },
];

const MOCK_DATA = {
  calenderDefaultCategories, events, defaultevent
}

export default MOCK_DATA;