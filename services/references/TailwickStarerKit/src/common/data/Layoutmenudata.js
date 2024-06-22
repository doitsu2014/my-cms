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
				link: "#",
			},
			{
				id: 'ecommerce',
				label: 'Ecommerce',
				link: "/",
			},
			{
				id: 'email',
				label: 'Email',
				link: "#",
			},
			{
				id: 'hr',
				label: 'HR',
				link: "#",
			},
			{
				id: 'social',
				label: 'Social',
				link: "#",
			},
		]
	},
	{
		"icon": "PictureInPicture2Icon",
		"label": "Landing Page",
		"link": "#",
		"id": "landingPage",
		"stateVariables": "islandingPage",
		"subMenu": [
			{
				id: 'onePage',
				label: 'One Page',
				link: "#",
			},
			{
				id: 'product',
				label: 'Product',
				link: "#",
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
		"link": "#",
		"id": "chat",
		"stateVariables": "isChat",
	},
	{
		"icon": "Mail",
		"label": "Email",
		"link": "#",
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
				link: "#",
			},
			{
				id: 'monthgrid',
				label: 'Month Grid',
				link: "#",
			},
			{
				id: 'multimonthstack',
				label: 'Multi Month Stack',
				link: "#",
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
				link: "#",
				stateVariables: "isProduct",
				isChildItem: true,
				childItems: [
					{
						id: 'listview',
						label: 'List view',
						link: "#",
					},
					{
						id: 'gridview',
						label: 'Grid view',
						link: "#",
					},
					{
						id: 'overview',
						label: 'Overview',
						link: "#",
					},
					{
						id: 'addnew',
						label: 'Add New',
						link: "#",
					},
				]
			},
			{
				id: 'shoppingcart',
				label: 'Shopping Cart',
				link: "#",
			},
			{
				id: 'checkout',
				label: 'Checkout',
				link: "#",
			},
			{
				id: 'orders',
				label: 'Orders',
				link: "#",
			},
			{
				id: 'orderoverview',
				label: 'Order Overview',
				link: "#",
			},
			{
				id: 'sellers',
				label: 'Sellers',
				link: "#",
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
				link: "#",
				stateVariables: "isEmplist",
			},
			{
				id: 'holidays',
				label: 'Holidays',
				link: "#",
			},
			{
				id: 'leavesmanage',
				label: 'Leaves Manage',
				link: "#",
				"stateVariables": "isLeavemanage",
				isChildItem: true,
				childItems: [
					{
						id: 'byEmp',
						label: 'By Employee',
						link: "#",
					},
					{
						id: 'addleave',
						label: 'Add Leave (Employee)',
						link: "#",
					},
					{
						id: 'byhr',
						label: 'By HR',
						link: "#",
					},
					{
						id: 'addleavehr',
						label: 'Add Leave (HR)',
						link: "#",
					},
				]
			},
			{
				id: 'attendance',
				label: 'Attendance',
				link: "#",
				"stateVariables": "isAttendance",
				isChildItem: true,
				childItems: [
					{
						id: 'attendancehr',
						label: 'Attendance (HR)',
						link: "#",
					},
					{
						id: 'mainattendance',
						label: 'Main Attendance',
						link: "#",
					}
				]
			},
			{
				id: 'department',
				label: 'Department',
				link: "#",
			},
			{
				id: 'sales',
				label: 'Sales',
				link: "#",
				"stateVariables": "isSales",
				isChildItem: true,
				childItems: [
					{
						id: 'esimates',
						label: 'Esimates',
						link: "#",
					},
					{
						id: 'payments',
						label: 'Payments',
						link: "#",
					},
					{
						id: 'expenses',
						label: 'Expenses',
						link: "#",
					}
				]
			},
			{
				id: 'payroll',
				label: 'Payroll',
				link: "#",
				"stateVariables": "isPayroll",
				isChildItem: true,
				childItems: [
					{
						id: 'employeeSalary',
						label: 'Employee Salary',
						link: "#",
					},
					{
						id: 'payslip',
						label: 'Payslip',
						link: "#",
					},
					{
						id: 'createpayslip',
						label: 'Create Payslip',
						link: "#",
					}
				]
			},
		]
	},
	{
		id: 'notes',
		label: 'Notes',
		link: "#",
		icon: "ScrollText"
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
				link: "#",
			},
			{
				id: 'events',
				label: 'Events',
				link: "#",
			},
			{
				id: 'watchvideo',
				label: 'Watch Video',
				link: "#",
			},
			{
				id: 'marketplace',
				label: 'Marketplace',
				link: "#",
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
				link: "#",
			},
			{
				id: 'addnew',
				label: 'Add New',
				link: "#",
			},
			{
				id: 'overview',
				label: 'Overview',
				link: "#",
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
				link: "#",
			},
			{
				id: 'gridview',
				label: 'Grid view',
				link: "#",
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
				link: "#",
				isChildItem: true,
				stateVariables: "isSignIn",
				childItems: [
					{ id: 'signin-basic', label: 'Basic', link: "#"},
					{ id: 'signin-cover', label: 'Cover', link: "#"},
					{ id: 'signin-boxed', label: 'boxed', link: "#"},
					{ id: 'signin-modern', label: 'Modern', link: "#"},
				]
			},
			{
				id: 'signUp',
				label: 'Register',
				link: "#",
				isChildItem: true,
				stateVariables: "isSignUp",
				childItems: [
					{ id: 'signup-basic', label: 'Basic', link: "#",},
					{ id: 'signup-cover', label: 'Cover', link: "#"},
					{ id: 'signup-boxed', label: 'boxed', link: "#"},
					{ id: 'signup-modern', label: 'Modern', link: "#"},
				]
			},
			{
				id: 'verifyEmail',
				label: 'Verify Email',
				link: "#",
				isChildItem: true,
				stateVariables: "isVerifyEmail",
				childItems: [
					{ id: 'verifyemail-basic', label: 'Basic', link: "#"},
					{ id: 'verifyemail-cover', label: 'Cover', link: "#"},
					{ id: 'verifyemail-modern', label: 'Modern', link: "#"},
				]
			},
			{
				id: 'twoStepVerification',
				label: 'Two Step',
				link: "#",
				isChildItem: true,
				stateVariables: "isVerification",
				childItems: [
					{ id: 'twostep-basic', label: 'Basic', link: "#"},
					{ id: 'twostep-cover', label: 'Cover', link: "#"},
					{ id: 'twostep-boxed', label: 'Boxed', link: "#"},
					{ id: 'twostep-modern', label: 'Modern', link: "#"},
				]
			},
			{
				id: 'logout',
				label: 'Logout',
				link: "#",
				isChildItem: true,
				stateVariables: "isLogout",
				childItems: [
					{ id: 'logout-basic', label: 'Basic', link: "#"},
					{ id: 'logout-cover', label: 'Cover', link: "#"},
					{ id: 'logout-boxed', label: 'Boxed', link: "#"},
					{ id: 'logout-modern', label: 'Modern', link: "#"},
				]
			},
			{
				id: 'passwordReset',
				label: 'Reset Password',
				link: "#",
				isChildItem: true,
				stateVariables: "isPasswordReset",
				childItems: [
					{ id: 'pass-reset-basic', label: 'Basic', link: "#"},
					{ id: 'pass-reset-cover', label: 'Cover', link: "#"},
					{ id: 'pass-reset-boxed', label: 'Boxed', link: "#"},
					{ id: 'pass-reset-modern', label: 'Modern', link: "#"},
				]
			},
			{
				id: "passwordCreate",
				label: "Create Password",
				link: "#",
				isChildItem: true,
				parentId: "authentication",
				stateVariables: 'isPasswordCreate',
				childItems: [
					{ id: '1', label: "Basic", link: "#"},
					{ id: '2', label: "Cover", link: "#"},
					{ id: '3', label: "Boxed", link: "#"},
					{ id: '4', label: "Modern", link: "#"},
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
			{ "label": "Account", "link": "#" },
			{
				"label": "Settings", "link": "#", "id": "settings",
			},
			{ "label": "Pricing", "link": "#" },
			{ "label": "FAQs", "link": "#" },
			{ "label": "Contact Us", "link": "#" },
			{ "label": "Coming Soon", "link": "/pages/coming-soon" },
			{
				id: 'errors',
				label: 'Error Pages',
				link: "#",
				isChildItem: true,
				stateVariables: "isError",
				childItems: [
					{ id: '404', label: '404', link: "/pages/404"},
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
			{ "id": "alerts", "label": "Alerts", "link": "#", "parentId": "baseUi" },
			{ "id": "avatar", "label": "Avatar", "link": "#", "parentId": "baseUi" },
			{ "id": "buttons", "label": "Buttons", "link": "#", "parentId": "baseUi" },
			{ "id": "label", "label": "Label", "link": "#", "parentId": "baseUi" },
			{ "id": "cards", "label": "Cards", "link": "#", "parentId": "baseUi" },
			{ "id": "collapse", "label": "Collapse", "link": "#", "parentId": "baseUi" },
			{ "id": "countdown", "label": "Countdown", "link": "#", "parentId": "baseUi" },
			{ "id": "drawer", "label": "Drawer", "link": "#", "parentId": "baseUi" },
			{ "id": "dropdown", "label": "Dropdown", "link": "#", "parentId": "baseUi" },
			{ "id": "gallery", "label": "Gallery", "link": "#", "parentId": "baseUi" },
			{ "id": "lists", "label": "Lists", "link": "#", "parentId": "baseUi" },
			{ "id": "modal", "label": "Modal", "link": "#", "parentId": "baseUi" },
			{ "id": "spinners", "label": "Spinners", "link": "#", "parentId": "baseUi" },
			{ "id": "timeline", "label": "Timeline", "link": "#", "parentId": "baseUi" },
			{ "id": "progress-bar", "label": "Progress Bar", "link": "#","parentId": "baseUi" },
			{ "id": "tooltip", "label": "Tooltip", "link": "#", "parentId": "baseUi" },
			{ "id": "video", "label": "Video", "link": "#", "parentId": "baseUi" }
		]
	},
	{
		"label": "Plugins",
		"icon": "PackagePlus",
		"id": "plugins",
		"stateVariables": "plugins",
		"subMenu": [
			{ "id": "sweetalert", "label": "Sweetalert", "link": "#", "parentId": "plugins" },
			{ "id": "lightbox", "label": "Lightbox", "link": "#", "parentId": "plugins" },
			{ "id": "swiper-slider", "label": "Swiper Slider", "link": "#", "parentId": "plugins" },
			{ "id": "video-player", "label": "Video Player", "link": "#", "parentId": "plugins" }
		]
	},
	{
		"label": "Navigation",
		"icon": "LocateFixedIcon",
		"id": "navigation",
		"stateVariables": "navigation",
		"subMenu": [
			{ "id": "navbar", "label": "Navbar", "link": "#", "parentId": "navigation" },
			{ "id": "tabs", "label": "Tabs", "link": "#", "parentId": "navigation" },
			{ "id": "breadcrumb", "label": "Breadcrumb", "link": "#", "parentId": "navigation" },
			{ "id": "pagination", "label": "Pagination", "link": "#", "parentId": "navigation" }
		]
	},
	{
		"label": "Forms",
		"icon": "FileText",
		"id": "forms",
		"stateVariables": "forms",
		"subMenu": [
			{ "label": "Basic", "id": "basic", "link": "#", "parentId": "forms" },
			{ "label": "Validation", "id": "validation", "link": "#", "parentId": "forms" },
			{ "label": "Inputmask", "id": "inputmask", "link": "#", "parentId": "forms" },
			{ "label": "Select", "id": "select", "link": "#", "parentId": "forms" },
			{ "label": "checkbox & Radio", "id": "checkboxradio", "link": "#", "parentId": "forms" },
			{ "label": "Switches", "id": "switches", "link": "#", "parentId": "forms" },
			{ "label": "Wizards", "id": "wizards", "link": "#", "parentId": "forms" },
			{ "label": "File Upload", "id": "Fileupload", "link": "#", "parentId": "forms" },
			{ "label": "Date Picker", "id": "datepicker", "link": "#", "parentId": "forms" },
			{ "label": "Time Picker", "id": "timepicker", "link": "#", "parentId": "forms" },
			{ "label": "Color Picker", "id": "colorpicker", "link": "#", "parentId": "forms" },
			{ "label": "Input Spin", "id": "inputspin", "link": "#", "parentId": "forms" },
			{ "label": "Clipboard", "id": "Clipboard", "link": "#", "parentId": "forms" },

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
				"link": "#",
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
				link: "#",
			},
			{
				id: "barChart",
				label: "Bar",
				link: "#",
			},
			{
				id: "boxplotChart",
				label: "Boxplot",
				link: "#",
			},
			{
				id: "bubbleChart",
				label: "Bubble",
				link: "#",
			},
			{
				id: "candlstickChart",
				label: "Candlstick",
				link: "#",
			},
			{
				id: "columnChart",
				label: "Column",
				link: "#",
			},
			{
				id: "funnelChart",
				label: "Funnel",
				link: "#",
			},
			{
				id: "heatmapChart",
				label: "Heatmap",
				link: "#",
			},
			{
				id: "lineChart",
				label: "Line",
				link: "#",
			},
			{
				id: "mixedChart",
				label: "Mixed",
				link: "#",
			},
			{
				id: "pieChart",
				label: "Pie",
				link: "#",
			},
			{
				id: "polarareaChart",
				label: "Polar Area",
				link: "#",
			},
			{
				id: "radarChart",
				label: "Radar",
				link: "#",
			},
			{
				id: "radialbarChart",
				label: "Radialbar",
				link: "#",
			},
			{
				id: "rangeareaChart",
				label: "Range Area",
				link: "#",
			},
			{
				id: "scatterChart",
				label: "Scatter",
				link: "#",
			},
			{
				id: "timelineChart",
				label: "Timeline",
				link: "#",
			},
			{
				id: "treemapChart",
				label: "Treemap",
				link: "#",
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
				link: "#",
			},
			{
				id: "lucide",
				label: "Lucide",
				link: "#",
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
				link: "#",
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