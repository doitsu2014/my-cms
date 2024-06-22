const Navdata = [
	{
		label: 'Menu',
		isHeader: true
	},
	{
		"icon": "MonitorDot",
		"label": "Dashboards",
		"link": "/",
		"id": "dashboard",
		"stateVariables": "isdashboard",
		"subMenu": [
			{
				id: 'analytics',
				label: 'Analytics',
				link: '/dashboard/analytics',
			},
			{
				id: 'ecommerce',
				label: 'Ecommerce',
				link: '/',
			},
			{
				id: 'email',
				label: 'Email',
				link: '/dashboard/email',
			},
			{
				id: 'hr',
				label: 'HR',
				link: '/dashboard/hr',
			},
			{
				id: 'social',
				label: 'Social',
				link: '/dashboard/social-media',
			},
		]
	},
	{
		"icon": "PictureInPicture2Icon",
		"label": "Landing Page",
		"link": "/",
		"id": "landingPage",
		"stateVariables": "islandingPage",
		"subMenu": [
			{
				id: 'onePage',
				label: 'One Page',
				link: '/landing/one',
			},
			{
				id: 'product',
				label: 'Product',
				link: '/landing/product',
			},
		]
	},
	{
		label: 'Apps',
		isHeader: true
	},
	{
		"icon": "MessagesSquare",
		"label": "Chat",
		"link": "/apps/chat",
		"id": "chat",
		"stateVariables": "isChat",
	},
	{
		"icon": "Mail",
		"label": "Email",
		"link": "/apps/mailbox",
		"id": "email",
		"stateVariables": "isEmail",
	},
	{
		"icon": "CalendarDaysIcon",
		"label": "Calender",
		"link": "#",
		"id": "calender",
		"stateVariables": "isCalender",
		"subMenu": [
			{
				id: 'default',
				label: 'Default',
				link: '/apps/calendar',
			},
			{
				id: 'monthgrid',
				label: 'Month Grid',
				link: '/apps/calendar-month-grid',
			},
			{
				id: 'multimonthstack',
				label: 'Multi Month Stack',
				link: '/apps/calendar-multi-month-stack',
			},
		]
	},
	{
		"icon": "ShoppingBag",
		"label": "Ecommerce",
		"link": "javascript:void(0)",
		"id": "ecommerce",
		"stateVariables": "isEcommerce",
		"subMenu": [
			{
				id: 'product',
				label: 'Product',
				link: 'javascript:void(0)',
				stateVariables: "isProduct",
				isChildItem: true,
				childItems: [
					{
						id: 'listview',
						label: 'List view',
						link: '/apps/ecommerce/products-list',
					},
					{
						id: 'gridview',
						label: 'Grid view',
						link: '/apps/ecommerce/products-grid',
					},
					{
						id: 'overview',
						label: 'Overview',
						link: '/apps/ecommerce/products-overview',
					},
					{
						id: 'addnew',
						label: 'Add New',
						link: '/apps/ecommerce/products-new',
					},
				]
			},
			{
				id: 'shoppingcart',
				label: 'Shopping Cart',
				link: '/apps/ecommerce/cart',
			},
			{
				id: 'checkout',
				label: 'Checkout',
				link: '/apps/ecommerce/checkout',
			},
			{
				id: 'orders',
				label: 'Orders',
				link: '/apps/ecommerce/orders',
			},
			{
				id: 'orderoverview',
				label: 'Order Overview',
				link: '/apps/ecommerce/order-overview',
			},
			{
				id: 'sellers',
				label: 'Sellers',
				link: '/apps/ecommerce/sellers',
			},
		]
	},
	{
		"icon": "CircuitBoard",
		"label": "HR Management",
		"link": "#",
		"id": "hrmanagement",
		"stateVariables": "isHRManagement",
		"subMenu": [
			{
				id: 'emplist',
				label: 'Employee List',
				link: '/apps/hr/employees-list',
				stateVariables: "isEmplist",
			},
			{
				id: 'holidays',
				label: 'Holidays',
				link: '/apps/hr/holidays',
			},
			{
				id: 'leavesmanage',
				label: 'Leaves Manage',
				link: '#',
				"stateVariables": "isLeavemanage",
				isChildItem: true,
				childItems: [
					{
						id: 'byEmp',
						label: 'By Employee',
						link: '/apps/hr/leave-employee',
					},
					{
						id: 'addleave',
						label: 'Add Leave (Employee)',
						link: '/apps/hr/add-employee-leave',
					},
					{
						id: 'byhr',
						label: 'By HR',
						link: '/apps/hr/leave',
					},
					{
						id: 'addleavehr',
						label: 'Add Leave (HR)',
						link: '/apps/hr/add-leave',
					},
				]
			},
			{
				id: 'attendance',
				label: 'Attendance',
				link: '#',
				"stateVariables": "isAttendance",
				isChildItem: true,
				childItems: [
					{
						id: 'attendancehr',
						label: 'Attendance (HR)',
						link: '/apps/hr/attendance',
					},
					{
						id: 'mainattendance',
						label: 'Main Attendance',
						link: '/apps/hr/main-attendance',
					}
				]
			},
			{
				id: 'department',
				label: 'Department',
				link: '/apps/hr/department',
			},
			{
				id: 'sales',
				label: 'Sales',
				link: '#',
				"stateVariables": "isSales",
				isChildItem: true,
				childItems: [
					{
						id: 'esimates',
						label: 'Esimates',
						link: '/apps/hr/sales-estimates',
					},
					{
						id: 'payments',
						label: 'Payments',
						link: '/apps/hr/sales-payments',
					},
					{
						id: 'expenses',
						label: 'Expenses',
						link: '/apps/hr/sales-expenses',
					}
				]
			},
			{
				id: 'payroll',
				label: 'Payroll',
				link: '#',
				"stateVariables": "isPayroll",
				isChildItem: true,
				childItems: [
					{
						id: 'employeeSalary',
						label: 'Employee Salary',
						link: '/apps/hr/employee-salary',
					},
					{
						id: 'payslip',
						label: 'Payslip',
						link: '/apps/hr/payslip',
					},
					{
						id: 'createpayslip',
						label: 'Create Payslip',
						link: '/apps/hr/add-payslip',
					}
				]
			},
		]
	},
	{
		id: 'notes',
		label: 'Notes',
		link: '/apps/notes',
		icon:"ScrollText"
	},
	{
		"icon": "RadioTower",
		"label": "Social",
		"link": "#",
		"id": "social",
		"stateVariables": "isSocial",
		"subMenu": [
			{
				id: 'friends',
				label: 'Friends',
				link: '/apps/social/friends',
			},
			{
				id: 'events',
				label: 'Events',
				link: '/apps/social/events',
			},
			{
				id: 'watchvideo',
				label: 'Watch Video',
				link: '/apps/social/video',
			},
			{
				id: 'marketplace',
				label: 'Marketplace',
				link: '/apps/social/marketplace',
			},
		]
	},
	{
		"icon": "FileText",
		"label": "Invoices",
		"link": "#",
		"id": "invoices",
		"stateVariables": "isInvoices",
		"subMenu": [
			{
				id: 'listview',
				label: 'List view',
				link: '/apps/invoice/list',
			},
			{
				id: 'addnew',
				label: 'Add New',
				link: '/apps/invoice/addnew',
			},
			{
				id: 'overview',
				label: 'Overview',
				link: '/apps/invoice/overview',
			},
		]
	},
	{
		"icon": "User2",
		"label": "Users",
		"link": "#",
		"id": "users",
		"stateVariables": "isUses",
		"subMenu": [
			{
				id: 'listview',
				label: 'List view',
				link: '/apps/users/list',
			},
			{
				id: 'gridview',
				label: 'Grid view',
				link: '/apps/users/grid',
			},
		]
	},
	{
		label: 'Pages',
		isHeader: true
	},
	{
		"label": "Authentication",
		"icon": "AwardIcon",
		"id": "isAuth",
		"stateVariables": "isAuth",
		"subMenu": [
			{
				id: 'signIn',
				label: 'Login',
				link: '#',
				isChildItem: true,
				stateVariables: "isSignIn",
				childItems: [
					{ id: 'signin-basic', label: 'Basic', link: '/auth/signin-basic' },
					{ id: 'signin-cover', label: 'Cover', link: '/auth/signin-cover' },
					{ id: 'signin-boxed', label: 'boxed', link: '/auth/signin-boxed' },
					{ id: 'signin-modern', label: 'Modern', link: '/auth/signin-modern' },
				]
			},
			{
				id: 'signUp',
				label: 'Register',
				link: '#',
				isChildItem: true,
				stateVariables: "isSignUp",
				childItems: [
					{ id: 'signup-basic', label: 'Basic', link: '/auth/signup-basic' },
					{ id: 'signup-cover', label: 'Cover', link: '/auth/signup-cover' },
					{ id: 'signup-boxed', label: 'boxed', link: '/auth/signup-boxed' },
					{ id: 'signup-modern', label: 'Modern', link: '/auth/signup-modern' },
				]
			},
			{
				id: 'verifyEmail',
				label: 'Verify Email',
				link: '#',
				isChildItem: true,
				stateVariables: "isVerifyEmail",
				childItems: [
					{ id: 'verifyemail-basic', label: 'Basic', link: '/auth/verifyemail-basic' },
					{ id: 'verifyemail-cover', label: 'Cover', link: '/auth/verifyemail-cover' },
					{ id: 'verifyemail-modern', label: 'Modern', link: '/auth/verifyemail-modern' },
				]
			},
			{
				id: 'twoStepVerification',
				label: 'Two Step',
				link: '/#',
				isChildItem: true,
				stateVariables: "isVerification",
				childItems: [
					{ id: 'twostep-basic', label: 'Basic', link: '/auth/twostep-basic' },
					{ id: 'twostep-cover', label: 'Cover', link: '/auth/twostep-cover' },
					{ id: 'twostep-boxed', label: 'Boxed', link: '/auth/twostep-boxed' },
					{ id: 'twostep-modern', label: 'Modern', link: '/auth/twostep-modern' },
				]
			},
			{
				id: 'logout',
				label: 'Logout',
				link: '/#',
				isChildItem: true,
				stateVariables: "isLogout",
				childItems: [
					{ id: 'logout-basic', label: 'Basic', link: '/auth/logout-basic' },
					{ id: 'logout-cover', label: 'Cover', link: '/auth/logout-cover' },
					{ id: 'logout-boxed', label: 'Boxed', link: '/auth/logout-boxed' },
					{ id: 'logout-modern', label: 'Modern', link: '/auth/logout-modern' },
				]
			},
			{
				id: 'passwordReset',
				label: 'Reset Password',
				link: '#',
				isChildItem: true,
				stateVariables: "isPasswordReset",
				childItems: [
					{ id: 'pass-reset-basic', label: 'Basic', link: '/auth/pass-reset-basic' },
					{ id: 'pass-reset-cover', label: 'Cover', link: '/auth/pass-reset-cover' },
					{ id: 'pass-reset-boxed', label: 'Boxed', link: '/auth/pass-reset-boxed' },
					{ id: 'pass-reset-modern', label: 'Modern', link: '/auth/pass-reset-modern' },
				]
			},
			{
				id: "passwordCreate",
				label: "Create Password",
				link: "/#",
				isChildItem: true,
				parentId: "authentication",
				stateVariables: 'isPasswordCreate',
				childItems: [
					{ id: '1', label: "Basic", link: "/auth/pass-create-basic" },
					{ id: '2', label: "Cover", link: "/auth/pass-create-cover" },
					{ id: '3', label: "Boxed", link: "/auth/pass-create-boxed" },
					{ id: '4', label: "Modern", link: "/auth/pass-create-modern" },
				]
			},
		]

	},
	{
		"label": "Pages",
		"icon": "CodesandboxIcon",
		"id": "pages",
		"stateVariables": "pages",
		"subMenu": [
			{ "label": "Account", "link": "/pages/account" },
			{
				"label": "Settings", "link": "/pages/account-settings", "id": "settings",
			},
			{ "label": "Pricing", "link": "/pages/pricing" },
			{ "label": "FAQs", "link": "/pages/faqs" },
			{ "label": "Contact Us", "link": "/pages/contact-us" },
			{ "label": "Coming Soon", "link": "/pages/coming-soon" },
			{
				id: 'errors',
				label: 'Error Pages',
				link: '#',
				isChildItem: true,
				stateVariables: "isError",
				childItems: [
					{ id: '404', label: '404', link: '/pages/404' },
					{ id: 'auth-offline', label: 'Offline', link: '/pages/offline' }
				]
			},
			{ "label": "Maintenance", "link": "/pages/maintenance" },
		]
	},
	{
		label: 'Components',
		isHeader: true
	},
	{
		"label": "UI Elements",
		"icon": "LifeBuoyIcon",
		"id": "baseUi",
		"stateVariables": "baseUi",
		"subMenu": [
			{ "id": "alerts", "label": "Alerts", "link": "/ui/alerts", "parentId": "baseUi" },
			{ "id": "avatar", "label": "Avatar", "link": "/ui/avatar", "parentId": "baseUi" },
			{ "id": "buttons", "label": "Buttons", "link": "/ui/buttons", "parentId": "baseUi" },
			{ "id": "label", "label": "Label", "link": "/ui/label", "parentId": "baseUi" },
			{ "id": "cards", "label": "Cards", "link": "/ui/cards", "parentId": "baseUi" },
			{ "id": "collapse", "label": "Collapse", "link": "/ui/collapse", "parentId": "baseUi" },
			{ "id": "countdown", "label": "Countdown", "link": "/ui/countdown", "parentId": "baseUi" },
			{ "id": "drawer", "label": "Drawer", "link": "/ui/drawer", "parentId": "baseUi" },
			{ "id": "dropdown", "label": "Dropdown", "link": "/ui/dropdown", "parentId": "baseUi" },
			{ "id": "gallery", "label": "Gallery", "link": "/ui/gallery", "parentId": "baseUi" },
			{ "id": "lists", "label": "Lists", "link": "/ui/lists", "parentId": "baseUi" },
			{ "id": "modal", "label": "Modal", "link": "/ui/modal", "parentId": "baseUi" },
			{ "id": "spinners", "label": "Spinners", "link": "/ui/spinners", "parentId": "baseUi" },
			{ "id": "timeline", "label": "Timeline", "link": "/ui/timeline", "parentId": "baseUi" },
			{ "id": "progress-bar", "label": "Progress Bar", "link": "progress-bar", "parentId": "baseUi" },
			{ "id": "tooltip", "label": "Tooltip", "link": "/ui/tooltip", "parentId": "baseUi" },
			{ "id": "video", "label": "Video", "link": "/ui/video", "parentId": "baseUi" }
		]
	},
	{
		"label": "Plugins",
		"icon": "PackagePlus",
		"id": "plugins",
		"stateVariables": "plugins",
		"subMenu": [
			{ "id": "sweetalert", "label": "Sweetalert", "link": "/plugins/sweetalert", "parentId": "plugins" },
			{ "id": "lightbox", "label": "Lightbox", "link": "/plugins/lightbox", "parentId": "plugins" },
			{ "id": "swiper-slider", "label": "Swiper Slider", "link": "/plugins/swiper-slider", "parentId": "plugins" },
			{ "id": "video-player", "label": "Video Player", "link": "/plugins/video-player", "parentId": "plugins" }
		]
	},
	{
		"label": "Navigation",
		"icon": "LocateFixedIcon",
		"id": "navigation",
		"stateVariables": "navigation",
		"subMenu": [
			{ "id": "navbar", "label": "Navbar", "link": "/navigation/navbars", "parentId": "navigation" },
			{ "id": "tabs", "label": "Tabs", "link": "/navigation/tabs", "parentId": "navigation" },
			{ "id": "breadcrumb", "label": "Breadcrumb", "link": "/navigation/breadcrumb", "parentId": "navigation" },
			{ "id": "pagination", "label": "Pagination", "link": "/navigation/pagination", "parentId": "navigation" }
		]
	},
	{
		"label": "Forms",
		"icon": "FileText",
		"id": "forms",
		"stateVariables": "forms",
		"subMenu": [
			{ "label": "Basic", "id": "basic", "link": "/forms/basic", "parentId": "forms" },
			{ "label": "Validation", "id": "validation", "link": "/forms/validation", "parentId": "forms" },
			{ "label": "Inputmask", "id": "inputmask", "link": "/forms/inputmask", "parentId": "forms" },
			{ "label": "Select", "id": "select", "link": "/forms/select", "parentId": "forms" },
			{ "label": "checkbox & Radio", "id": "checkboxradio", "link": "/forms/checkboxradio", "parentId": "forms" },
			{ "label": "Switches", "id": "switches", "link": "/forms/switches", "parentId": "forms" },
			{ "label": "Wizards", "id": "wizards", "link": "/forms/wizards", "parentId": "forms" },
			{ "label": "File Upload", "id": "Fileupload", "link": "/forms/fileupload", "parentId": "forms" },
			{ "label": "Date Picker", "id": "datepicker", "link": "/forms/datepicker", "parentId": "forms" },
			{ "label": "Time Picker", "id": "timepicker", "link": "/forms/timepicker", "parentId": "forms" },
			{ "label": "Color Picker", "id": "colorpicker", "link": "/forms/colorpicker", "parentId": "forms" },
			{ "label": "Input Spin", "id": "inputspin", "link": "/forms/inputspin", "parentId": "forms" },
			{ "label": "Clipboard", "id": "Clipboard", "link": "/forms/clipboard", "parentId": "forms" },
			
		]
	},
	{
		"label": "Tables",
		"icon": "Table",
		"id": "tables",
		"stateVariables": "tables",
		"subMenu": [
			{
				id: "basictable",
				"label": "Basic Table",
				"link": "/tables/basic",
			},
		]
	},
	{
		"label": "Apexcharts",
		"icon": "FilePieChart",
		"id": "apexcharts",
		"stateVariables": "apexcharts",
		"subMenu": [
			{
				id: "areaChart",
				label: "Area",
				link: "/apexcharts/area",
			},
			{
				id: "barChart",
				label: "Bar",
				link: "/apexcharts/bar",
			},
			{
				id: "boxplotChart",
				label: "Boxplot",
				link: "/apexcharts/boxplot",
			},
			{
				id: "bubbleChart",
				label: "Bubble",
				link: "/apexcharts/bubble",
			},
			{
				id: "candlstickChart",
				label: "Candlstick",
				link: "/apexcharts/candlstick",
			},
			{
				id: "columnChart",
				label: "Column",
				link: "/apexcharts/column",
			},
			{
				id: "funnelChart",
				label: "Funnel",
				link: "/apexcharts/funnel",
			},
			{
				id: "heatmapChart",
				label: "Heatmap",
				link: "/apexcharts/heatmap",
			},
			{
				id: "lineChart",
				label: "Line",
				link: "/apexcharts/line",
			},
			{
				id: "mixedChart",
				label: "Mixed",
				link: "/apexcharts/mixed",
			},
			{
				id: "pieChart",
				label: "Pie",
				link: "/apexcharts/pie",
			},
			{
				id: "polarareaChart",
				label: "Polar Area",
				link: "/apexcharts/polararea",
			},
			{
				id: "radarChart",
				label: "Radar",
				link: "/apexcharts/radar",
			},
			{
				id: "radialbarChart",
				label: "Radialbar",
				link: "/apexcharts/radialbar",
			},
			{
				id: "rangeareaChart",
				label: "Range Area",
				link: "/apexcharts/rangearea",
			},
			{
				id: "scatterChart",
				label: "Scatter",
				link: "/apexcharts/scatter",
			},
			{
				id: "timelineChart",
				label: "Timeline",
				link: "/apexcharts/timeline",
			},
			{
				id: "treemapChart",
				label: "Treemap",
				link: "/apexcharts/treemap",
			},
		]
	},
	{
		"label": "Icons",
		"icon": "TrophyIcon",
		"id": "icons",
		"stateVariables": "icons",
		"subMenu": [
			{
				id: "remix",
				label: "remix",
				link: "/icons/remix",
			},
			{
				id: "lucide",
				label: "Lucide",
				link: "/icons/lucide",
			},
		]
	},
	{
		"label": "Maps",
		"icon": "Map",
		"id": "icons",
		"stateVariables": "icons",
		"subMenu": [
			{
				id: "googlemap",
				label: "Google Map",
				link: "/maps/googlemap",
			},
		]
	},
	{
		"label": "Multi Level",
		"icon": "Share2",
		"id": "multilevel",
		stateVariables: "multilevel",
		"subMenu": [
			{ "label": "Level 1.1", "link": "javascript:void(0);" },
			{
				"label": "Level 1.2",
				"link": "javascript:void(0);",
				"id": "level1",
				stateVariables: "level1",
				isChildItem: true,
				childItems: [
					{ "label": "Level 2.1", "link": "javascript:void(0);" },
					{
						"label": "Level 2.2", "link": "javascript:void(0);",
						"link": "javascript:void(0);",
						"id": "level2",
						stateVariables: "level2",
						isChildItem: true,
						childItems: [
							{ "label": "Level 3.1", "link": "javascript:void(0);" },
							{ "label": "Level 3.2", "link": "javascript:void(0);" }
						]
					}
				]
			}
		]
	}
]

const MOCK_DATA = {
	Navdata
}

export default MOCK_DATA;