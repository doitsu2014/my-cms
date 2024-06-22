
const DepartmentsListData = [
    { id: 1, departmentName: "Web Development", HOD: "Patricia Garcia", phone: "077 7317 7572", email: "PatriciaJGarcia@tailwick.com", employee: "15" },
    { id: 2, departmentName: "IOS Application Development", HOD: "Jonas Frederiksen", phone: "61 53 62 05", email: "jonas@tailwick.com", employee: "09" },
    { id: 3, departmentName: "Designing", HOD: "Juliette Fecteau", phone: "07231 96 25 88", email: "JulietteFecteau@tailwick.com", employee: "11" },
    { id: 4, departmentName: "HR Management", HOD: "Thomas Hatfield", phone: "0911 47 65 49", email: "thomas@tailwick.com", employee: "03" },
    { id: 5, departmentName: "Accounts Management", HOD: "Holly Kavanaugh", phone: "819 947 5846", email: "HollyKavanaugh@tailwick.com", employee: "02" }
];

const EmployeeListData = [
    {
        id: 1,
        employeeId: "#TWE1001528",
        name: "Willie Torres",
        img: "/assets/images/users/user-1.jpg",
        designation: "Nuxt JS Developer",
        email: "willie@tailwick.com",
        phone: "070 3715 3689",
        location: "United States",
        experience: "3 Year",
        joinDate: "05 Feb, 2020"
    },
    {
        id: 2,
        employeeId: "#TWE1001524",
        name: "Patricia Garcia",
        img: "/assets/images/users/user-2.jpg",
        designation: "ASP.Net Developer",
        email: "PatriciaJGarcia@tailwick.com",
        phone: "077 7317 7572",
        location: "Brazil",
        experience: "0.5 Year",
        joinDate: "12 Aug, 2023"
    },
    {
        id: 3,
        employeeId: "#TWE1001506",
        name: "Tonya Johnson",
        img: "/assets/images/users/user-3.jpg",
        designation: "Project Manager",
        email: "TonyaEJohnson@tailwick.com",
        phone: "079 2383 2340",
        location: "Denmark",
        experience: "0 Year",
        joinDate: "11 Nov, 2023"
    },
    {
        id: 4,
        employeeId: "#TWE1001502",
        name: "Jose White",
        img: "/assets/images/users/user-4.jpg",
        designation: "React Developer",
        email: "ameida@tailwick.com",
        phone: "03476 56 14 12",
        location: "Philippines",
        experience: "1.5 Year",
        joinDate: "09 Jun, 2022"
    },
    {
        id: 5,
        employeeId: "#TWE1001503",
        name: "Juliette Fecteau",
        img: "/assets/images/users/user-1.jpg",
        designation: "Sr. Angular Developer",
        email: "JulietteFecteau@tailwick.com",
        phone: "07231 96 25 88",
        location: "Belgium",
        experience: "1.9 Year",
        joinDate: "11 May, 2021"
    },
    {
        id: 6,
        employeeId: "#TWE1001504",
        name: "Jonas Frederiksen",
        img: "/assets/images/users/user-2.jpg",
        designation: "Team Leader",
        email: "jonas@tailwick.com",
        phone: "61 53 62 05",
        location: "France",
        experience: "2.9 Year",
        joinDate: "18 Jan, 2019"
    },
    {
        id: 7,
        employeeId: "#TWE1001505",
        name: "Kim Broberg",
        img: "/assets/images/users/user-4.jpg",
        designation: "UI / UX Designer",
        email: "KimBroberg@tailwick.com",
        phone: "040 382 2096",
        location: "Finland",
        experience: "1.2 Year",
        joinDate: "23 April, 2021"
    },
    {
        id: 8,
        employeeId: "#TWE1001507",
        name: "Nancy Reynolds",
        img: "/assets/images/users/user-1.jpg",
        designation: "Web Designer",
        email: "NancyM@tailwick.com",
        phone: "0391 13 79 21",
        location: "Germany",
        experience: "0.9 Year",
        joinDate: "01 July, 2022"
    },
    {
        id: 9,
        employeeId: "#TWE1001508",
        name: "Thomas Hatfield",
        img: "/assets/images/users/user-2.jpg",
        designation: "VueJs Developer",
        email: "thomas@tailwick.com",
        phone: "0911 47 65 49",
        location: "Mexico",
        experience: "1.6 Year",
        joinDate: "08 Aug, 2021"
    },
    {
        id: 10,
        employeeId: "#TWE1001509",
        name: "Holly Kavanaugh",
        img: "/assets/images/users/user-3.jpg",
        designation: "Laravel Developer",
        email: "HollyKavanaugh@tailwick.com",
        phone: "819 947 5846",
        location: "Canada",
        experience: "2.3 Year",
        joinDate: "23 Dec, 2020"
    },
    {
        id: 11,
        employeeId: "#TWE1001510",
        name: "Kim Broberg",
        img: "/assets/images/users/user-4.jpg",
        designation: "UI / UX Designer",
        email: "KimBroberg@tailwick.com",
        phone: "040 382 2096",
        location: "Finland",
        experience: "1.2 Year",
        joinDate: "23 April, 2021"
    },
    {
        id: 12,
        employeeId: "#TWE1001511",
        name: "Juliette Fecteau",
        img: "/assets/images/users/user-1.jpg",
        designation: "Sr. Angular Developer",
        email: "JulietteFecteau@tailwick.com",
        phone: "07231 96 25 88",
        location: "Belgium",
        experience: "1.9 Year",
        joinDate: "11 May, 2021"
    }
];

const HolidaysData = [
    { id: 1, day: "Monday", date: "15 Jan", holidayName: "Makara Sankranti", type: "Gazetted Holiday" },
    { id: 2, day: "Friday", date: "26 Jan", holidayName: "Republic Day", type: "Gazetted Holiday" },
    { id: 3, day: "Monday", date: "25 Mar", holidayName: "Holi", type: "Gazetted Holiday" },
    { id: 4, day: "Friday", date: "29 Mar", holidayName: "Good Friday", type: "Gazetted Holiday" },
    { id: 5, day: "Sunday", date: "4 Aug", holidayName: "Friendship Day", type: "Observance" },
    { id: 6, day: "Thursday", date: "15 Aug", holidayName: "Independence Day", type: "Gazetted Holiday" },
    { id: 7, day: "Sunday", date: "15 Sep", holidayName: "Onam", type: "Restricted Holiday" },
    { id: 8, day: "Thursday", date: "31 Oct", holidayName: "Halloween", type: "Observance" },
    { id: 9, day: "Wednesday", date: "25 Dec", holidayName: "Christmas", type: "Gazetted Holiday" },
    { id: 10, day: "Tuesday", date: "31 Dec", holidayName: "New Year", type: "Observance" }
];

const AttendanceData = [
    { id: 1, day: "Fri", date: "13 Oct, 2023", checkIn: "08:23 AM", checkOut: "07:00 PM", mealBreak: "1.00 Hrs", workHours: "8.00 Hrs", overtime: "0.00 Hrs" },
    { id: 2, day: "Thu", date: "12 Oct, 2023", checkIn: "08:28 AM", checkOut: "05:46 PM", mealBreak: "0.45 Hrs", workHours: "8.00 Hrs", overtime: "0.15 Hrs" },
    { id: 3, day: "Wed", date: "11 Oct, 2023", checkIn: "09:00 AM", checkOut: "06:00 PM", mealBreak: "0.50 Hrs", workHours: "8.00 Hrs", overtime: "0.35 Hrs" },
    { id: 4, day: "Tue", date: "10 Oct, 2023", checkIn: "09:15 AM", checkOut: "05:00 PM", mealBreak: "0.35 Hrs", workHours: "7.55 Hrs", overtime: "0.15 Hrs" },
    { id: 5, day: "Mon", date: "09 Oct, 2023", checkIn: "08:28 AM", checkOut: "05:46 PM", mealBreak: "0.45 Hrs", workHours: "8.00 Hrs", overtime: "0.15 Hrs" },
    { id: 6, day: "Fri", date: "06 Oct, 2023", checkIn: "09:00 AM", checkOut: "06:00 PM", mealBreak: "0.50 Hrs", workHours: "8.00 Hrs", overtime: "0.35 Hrs" },
    { id: 7, day: "Thu", date: "05 Oct, 2023", checkIn: "08:28 AM", checkOut: "05:46 PM", mealBreak: "0.45 Hrs", workHours: "8.00 Hrs", overtime: "0.15 Hrs" },
    { id: 8, day: "Wed", date: "04 Oct, 2023", checkIn: "08:28 AM", checkOut: "05:46 PM", mealBreak: "0.45 Hrs", workHours: "8.00 Hrs", overtime: "0.15 Hrs" },
    { id: 9, day: "Tue", date: "03 Oct, 2023", checkIn: "08:23 AM", checkOut: "07:00 PM", mealBreak: "1.00 Hrs", workHours: "8.00 Hrs", overtime: "0.00 Hrs" },
    { id: 10, day: "Mon", date: "02 Oct, 2023", checkIn: "09:15 AM", checkOut: "05:00 PM", mealBreak: "0.35 Hrs", workHours: "7.55 Hrs", overtime: "0.15 Hrs" }
];

const MainAttendanceData = [
    { id: 1, employeeName: "Patricia Garcia", Day1: false, Day2: false, Day3: "-", Day4: "-", Day5: true, Day6: true, Day7: true, Day8: true, Day9: false, Day10: "-", Day11: "-", Day12: true, Day13: false, Day14: true, Day15: true, Day16: true, Day17: "-", Day18: "-", Day19: false, Day20: true, Day21: true, Day22: true, Day23: true, Day24: "-", Day25: "-", Day26: true, Day27: true, Day28: true, Day29: true, Day30: true },
    { id: 2, employeeName: "Tonya Johnson", Day1: true, Day2: true, Day3: "-", Day4: "-", Day5: true, Day6: true, Day7: true, Day8: false, Day9: true, Day10: "-", Day11: "-", Day12: false, Day13: true, Day14: true, Day15: true, Day16: true, Day17: "-", Day18: "-", Day19: true, Day20: true, Day21: false, Day22: true, Day23: true, Day24: "-", Day25: "-", Day26: true, Day27: true, Day28: true, Day29: true, Day30: false },
    { id: 3, employeeName: "Willie Torres", Day1: true, Day2: false, Day3: "-", Day4: "-", Day5: true, Day6: false, Day7: false, Day8: true, Day9: true, Day10: "-", Day11: "-", Day12: true, Day13: true, Day14: true, Day15: true, Day16: true, Day17: "-", Day18: "-", Day19: true, Day20: true, Day21: true, Day22: true, Day23: true, Day24: "-", Day25: "-", Day26: true, Day27: true, Day28: false, Day29: true, Day30: true },
    { id: 4, employeeName: "Jose White", Day1: true, Day2: true, Day3: "-", Day4: "-", Day5: true, Day6: true, Day7: true, Day8: false, Day9: true, Day10: "-", Day11: "-", Day12: true, Day13: true, Day14: true, Day15: true, Day16: true, Day17: "-", Day18: "-", Day19: true, Day20: true, Day21: true, Day22: true, Day23: false, Day24: "-", Day25: "-", Day26: true, Day27: true, Day28: true, Day29: true, Day30: true },
    { id: 5, employeeName: "Juliette Fecteau", Day1: false, Day2: true, Day3: "-", Day4: "-", Day5: true, Day6: true, Day7: true, Day8: true, Day9: true, Day10: "-", Day11: "-", Day12: false, Day13: false, Day14: true, Day15: true, Day16: true, Day17: "-", Day18: "-", Day19: true, Day20: true, Day21: true, Day22: true, Day23: true, Day24: "-", Day25: "-", Day26: true, Day27: true, Day28: true, Day29: true, Day30: true },
    { id: 6, employeeName: "Jonas Frederiksen", Day1: true, Day2: true, Day3: "-", Day4: "-", Day5: false, Day6: false, Day7: false, Day8: true, Day9: true, Day10: "-", Day11: "-", Day12: true, Day13: true, Day14: true, Day15: true, Day16: false, Day17: "-", Day18: "-", Day19: true, Day20: true, Day21: true, Day22: true, Day23: true, Day24: "-", Day25: "-", Day26: false, Day27: true, Day28: true, Day29: true, Day30: true },
    { id: 7, employeeName: "Kim Broberg", Day1: true, Day2: true, Day3: "-", Day4: "-", Day5: true, Day6: true, Day7: true, Day8: false, Day9: true, Day10: "-", Day11: "-", Day12: true, Day13: true, Day14: true, Day15: false, Day16: true, Day17: "-", Day18: "-", Day19: true, Day20: false, Day21: true, Day22: true, Day23: true, Day24: "-", Day25: "-", Day26: true, Day27: true, Day28: true, Day29: true, Day30: true },
    { id: 8, employeeName: "Nancy Reynolds", Day1: true, Day2: true, Day3: "-", Day4: "-", Day5: false, Day6: true, Day7: true, Day8: true, Day9: true, Day10: "-", Day11: "-", Day12: true, Day13: true, Day14: true, Day15: false, Day16: true, Day17: "-", Day18: "-", Day19: true, Day20: true, Day21: true, Day22: true, Day23: true, Day24: "-", Day25: "-", Day26: false, Day27: true, Day28: true, Day29: true, Day30: true },
    { id: 9, employeeName: "Thomas Hatfield", Day1: false, Day2: true, Day3: "-", Day4: "-", Day5: true, Day6: true, Day7: false, Day8: true, Day9: true, Day10: "-", Day11: "-", Day12: true, Day13: true, Day14: false, Day15: true, Day16: true, Day17: "-", Day18: "-", Day19: true, Day20: false, Day21: true, Day22: true, Day23: true, Day24: "-", Day25: "-", Day26: true, Day27: true, Day28: false, Day29: true, Day30: true },
    { id: 10, employeeName: "Holly Kavanaugh", Day1: true, Day2: true, Day3: "-", Day4: "-", Day5: true, Day6: true, Day7: true, Day8: false, Day9: true, Day10: "-", Day11: "-", Day12: true, Day13: false, Day14: true, Day15: true, Day16: true, Day17: "-", Day18: "-", Day19: true, Day20: true, Day21: false, Day22: true, Day23: true, Day24: "-", Day25: "-", Day26: true, Day27: true, Day28: true, Day29: false, Day30: true }
];

const LeaveManageEmployeeData = [
    { id: "01", leaveType: "Medical Leave", reason: "Going to Hospital", noOfDays: "02", from: "11 Oct, 2023", to: "12 Oct, 2023", approvedBy: "Paula Keenan", status: "Approved" },
    { id: "02", leaveType: "Casual Leave", reason: "Going to Family Function", noOfDays: "01", from: "07 Sept, 2023", to: "07 Sept, 2023", approvedBy: "Admin", status: "Pending" },
    { id: "03", leaveType: "Casual Leave", reason: "Going to Holiday", noOfDays: "06", from: "11 Jun, 2023", to: "16 Jun, 2023", approvedBy: "Admin", status: "Declined" },
    { id: "04", leaveType: "Sick Leave", reason: "Attend Birthday party", noOfDays: "01", from: "15 July, 2023", to: "15 July, 2023", approvedBy: "Paula Keenan", status: "Approved" },
    { id: "05", leaveType: "Sick Leave", reason: "Personal", noOfDays: "02", from: "19 Aug, 2023", to: "20 Aug, 2023", approvedBy: "Paula Keenan", status: "Declined" },
    { id: "06", leaveType: "Casual Leave", reason: "Going to Family Function", noOfDays: "01", from: "14 Feb, 2022", to: "14 Feb, 2022", approvedBy: "Admin", status: "Approved" },
    { id: "07", leaveType: "Medical Leave", reason: "Medical Emergency", noOfDays: "04", from: "23 Jan, 2023", to: "26 Jan, 2023", approvedBy: "Paula Keenan", status: "Pending" },
    { id: "08", leaveType: "Casual Leave", reason: "Personal", noOfDays: "02", from: "16 Dec, 2023", to: "17 Dec, 2023", approvedBy: "Paula Keenan", status: "Declined" },
    { id: "09", leaveType: "Casual Leave", reason: "Going to Holiday", noOfDays: "05", from: "29 Nov, 2023", to: "03 Dec, 2023", approvedBy: "Admin", status: "Approved" },
    { id: "10", leaveType: "Sick Leave", reason: "Going to Hospital", noOfDays: "01", from: "15 Oct, 2023", to: "15 Oct, 2023", approvedBy: "Paula Keenan", status: "Approved" }
];

const LeaveManageHRData = [
    { id: "01", employeeName: "Willie Torres", leaveType: "Medical Leave", reason: "Going to Hospital", noOfDays: "02", from: "11 Oct, 2023", to: "12 Oct, 2023", status: "Approved" },
    { id: "02", employeeName: "Patricia Garcia", leaveType: "Casual Leave", reason: "Going to Family Function", noOfDays: "01", from: "07 Sept, 2023", to: "07 Sept, 2023", status: "Pending" },
    { id: "03", employeeName: "Juliette Fecteau", leaveType: "Casual Leave", reason: "Going to Holiday", noOfDays: "06", from: "11 Jun, 2023", to: "16 Jun, 2023", status: "New" },
    { id: "04", employeeName: "Thomas Hatfield", leaveType: "Sick Leave", reason: "Attend Birthday party", noOfDays: "01", from: "15 July, 2023", to: "15 July, 2023", status: "Approved" },
    { id: "05", employeeName: "Willie Torres", leaveType: "Sick Leave", reason: "Personal", noOfDays: "02", from: "19 Aug, 2023", to: "20 Aug, 2023", status: "Declined" },
    { id: "06", employeeName: "Juliette Fecteau", leaveType: "Casual Leave", reason: "Going to Family Function", noOfDays: "01", from: "14 Feb, 2022", to: "14 Feb, 2022", status: "Approved" },
    { id: "07", employeeName: "Nancy Reynolds", leaveType: "Medical Leave", reason: "Medical Emergency", noOfDays: "04", from: "23 Jan, 2023", to: "26 Jan, 2023", status: "Pending" },
    { id: "08", employeeName: "Holly Kavanaugh", leaveType: "Casual Leave", reason: "Personal", noOfDays: "02", from: "16 Dec, 2023", to: "17 Dec, 2023", status: "New" },
    { id: "09", employeeName: "Jonas Frederiksen", leaveType: "Casual Leave", reason: "Going to Holiday", noOfDays: "05", from: "29 Nov, 2023", to: "03 Dec, 2023", status: "Approved" },
    { id: "10", employeeName: "Nancy Reynolds", leaveType: "Sick Leave", reason: "Going to Hospital", noOfDays: "01", from: "15 Oct, 2023", to: "15 Oct, 2023", status: "Approved" }
];

const EmployeeSalaryData = [
    { id: 1, employeeId: "#TWE1001501", joiningDate: "05 Feb, 2020", name: "Willie Torres", img: "/assets/images/users/user-1.jpg", designation: "Nuxt JS Developer", emailId: "willie@tailwick.com", phoneNumber: "070 3715 3689", salary: "$8,500", bonus: "$500" },
    { id: 2, employeeId: "#TWE1001524", joiningDate: "12 Aug, 2023", name: "Patricia Garcia", img: "/assets/images/users/avatar-3.png", designation: "ASP.Net Developer", emailId: "PatriciaJGarcia@tailwick.com", phoneNumber: "077 7317 7572", salary: "$3,800", bonus: "-" },
    { id: 3, employeeId: "#TWE1001506", joiningDate: "11 Nov, 2023", name: "Tonya Johnson", img:  "/assets/images/users/avatar-6.png", designation: "Project Manager", emailId: "TonyaEJohnson@tailwick.com", phoneNumber: "079 2383 2340", salary: "$6,000", bonus: "$1,500" },
    { id: 4, employeeId: "#TWE1001502", joiningDate: "09 Jun, 2022", name: "Jose White", img: "/assets/images/users/user-dummy-img.jpg", designation: "React Developer", emailId: "ameida@tailwick.com", phoneNumber: "03476 56 14 12", salary: "$7,150", bonus: "$300" },
    { id: 5, employeeId: "#TWE1001503", joiningDate: "11 May, 2021", name: "Juliette Fecteau", img: "/assets/images/users/user-3.jpg", designation: "Sr. Angular Developer", emailId: "JulietteFecteau@tailwick.com", phoneNumber: "07231 96 25 88", salary: "$7,900", bonus: "-" },
    { id: 6, employeeId: "#TWE1001504", joiningDate: "18 Jan, 2019", name: "Jonas Frederiksen", img: "/assets/images/users/user-2.jpg", designation: "Team Leader", emailId: "jonas@tailwick.com", phoneNumber: "61 53 62 05", salary: "$5,399", bonus: "-" },
    { id: 7, employeeId: "#TWE1001505", joiningDate: "23 April, 2021", name: "Kim Broberg", img: "/assets/images/users/avatar-8.png", designation: "UI / UX Designer", emailId: "KimBroberg@tailwick.com", phoneNumber: "040 382 2096", salary: "$6,983", bonus: "$1,800" },
    { id: 8, employeeId: "#TWE1001507", joiningDate: "01 July, 2022", name: "Nancy Reynolds", img: "/assets/images/users/avatar-10.png", designation: "Web Designer", emailId: "NancyM@tailwick.com", phoneNumber: "0391 13 79 21", salary: "$3,300", bonus: "$1,000" },
    { id: 9, employeeId: "#TWE1001508", joiningDate: "08 Aug, 2021", name: "Thomas Hatfield", img: "/assets/images/users/avatar-9.png", designation: "VueJs Developer", emailId: "thomas@tailwick.com", phoneNumber: "0911 47 65 49", salary: "$9,145", bonus: "$760" },
    { id: 10, employeeId: "#TWE1001509", joiningDate: "23 Dec, 2020", name: "Holly Kavanaugh", img: "/assets/images/users/user-4.jpg", designation: "Laravel Developer", emailId: "HollyKavanaugh@tailwick.com", phoneNumber: "819 947 5846", salary: "$8,750", bonus: "$2,300" }
];

const EstimatesData = [
    { id: 1, estimateNumber: "#TWE20015420", clientName: "Infra Teach", estimateBy: "HR", estimateDate: "02 July, 2023", expiryDate: "05 July, 2023", amount: "$2,403", status: "Accepted" },
    { id: 2, estimateNumber: "#TWE20015421", clientName: "Themesdesign", estimateBy: "Admin", estimateDate: "08 Nov, 2023", expiryDate: "08 Nov, 2023", amount: "$1,749", status: "Declined" },
    { id: 3, estimateNumber: "#TWE20015422", clientName: "Judil Fashion", estimateBy: "Admin", estimateDate: "11 Aug, 2023", expiryDate: "03 Aug, 2023", amount: "$816", status: "Expired" },
    { id: 4, estimateNumber: "#TWE20015423", clientName: "4xM Infotech", estimateBy: "HR", estimateDate: "20 Sep, 2023", expiryDate: "21 Sep, 2023", amount: "$1,184", status: "Accepted" },
    { id: 5, estimateNumber: "#TWE20015424", clientName: "Digitech Galaxy", estimateBy: "HR", estimateDate: "07 Oct, 2023", expiryDate: "09 Oct, 2023", amount: "$5,463", status: "Accepted" },
    { id: 6, estimateNumber: "#TWE20015425", clientName: "Zoetic Fashion", estimateBy: "HR", estimateDate: "18 Dec, 2023", expiryDate: "20 Dec, 2023", amount: "$3,463", status: "Declined" }
];

const ExpensesData = [
    { id: 1, purchaseDate: "02 July, 2023", item: "Mac System", purchasedBy: "HR", paidBy: "HR", amount: "$5,478" },
    { id: 2, purchaseDate: "08 Nov, 2023", item: "HP Desktop", purchasedBy: "Louisa Howe", paidBy: "HR", amount: "$4,342" },
    { id: 3, purchaseDate: "11 Aug, 2023", item: "i5 PC", purchasedBy: "Everett Moore", paidBy: "Admin", amount: "$8,327" },
    { id: 4, purchaseDate: "20 Sep, 2023", item: "Decoration", purchasedBy: "Omari Welch", paidBy: "HR", amount: "$563" },
    { id: 5, purchaseDate: "07 Oct, 2023", item: "Cake", purchasedBy: "HR", paidBy: "Susana Dooley", amount: "$148" },
    { id: 6, purchaseDate: "18 Dec, 2023", item: "Salary to Employees", purchasedBy: "Omari Welch", paidBy: "Paula Keenan", amount: "$4,500" },
    { id: 7, purchaseDate: "21 Jan, 2024", item: "Salary to Employees", purchasedBy: "Everett Moore", paidBy: "Paula Keenan", amount: "$3,000" },
    { id: 8, purchaseDate: "02 Feb, 2023", item: "HP & Lenovo Desktop", purchasedBy: "HR", paidBy: "HR", amount: "$4,177" },
    { id: 9, purchaseDate: "11 Dec, 2023", item: "Birthday Party", purchasedBy: "HR", paidBy: "HR", amount: "$2,307" },
    { id: 10, purchaseDate: "01 Jan, 2023", item: "Festival Decoration", purchasedBy: "Lucie Beahan", paidBy: "HR", amount: "$543.99" }
];

const PaymentsData = [
    { id: 1, paymentId: "#TWP10021320", membershipPlan: "Regular License", date: "02 July, 2023", paymentType: "PayPal", username: "Infra Teach", amount: "$2,403", status: "Paid" },
    { id: 2, paymentId: "#TWP10021321", membershipPlan: "Extended License", date: "08 Nov, 2023", paymentType: "Credit Card", username: "Kim Broberg", amount: "$1,341", status: "Pending" },
    { id: 3, paymentId: "#TWP10021322", membershipPlan: "Extended License", date: "11 Aug, 2023", paymentType: "Bank Transfer", username: "Everett Moore", amount: "$816.21", status: "Failed" },
    { id: 4, paymentId: "#TWP10021323", membershipPlan: "Regular License", date: "20 Sep, 2023", paymentType: "PayPal", username: "Omari Welch", amount: "$1,184", status: "Paid" },
    { id: 5, paymentId: "#TWP10021324", membershipPlan: "Regular License", date: "07 Oct, 2023", paymentType: "Debit Card", username: "Susana Dooley", amount: "$5,463", status: "Paid" },
    { id: 6, paymentId: "#TWP10021325", membershipPlan: "Regular License", date: "18 Dec, 2023", paymentType: "PayPal", username: "Paul Gerhold", amount: "$3,463", status: "Pending" },
    { id: 7, paymentId: "#TWP10021326", membershipPlan: "Extended License", date: "21 Jan, 2024", paymentType: "Back Transfer", username: "Lucie Beahan", amount: "$1,543", status: "Failed" },
    { id: 8, paymentId: "#TWP10021327", membershipPlan: "Regular License", date: "02 Feb, 2023", paymentType: "PayPal", username: "Jose White", amount: "$4,177", status: "Paid" },
    { id: 9, paymentId: "#TWP10021328", membershipPlan: "Extended License", date: "11 Dec, 2023", paymentType: "Debit Card", username: "Jonas Frederiksen", amount: "$2,307", status: "Pending" },
    { id: 10, paymentId: "#TWP10021329", membershipPlan: "Extended License", date: "01 Jan, 2023", paymentType: "PayPal", username: "Stephen Tillman", amount: "$543.99", status: "Paid" }
];

export { DepartmentsListData, EmployeeListData, HolidaysData, AttendanceData, MainAttendanceData, LeaveManageEmployeeData, LeaveManageHRData, EmployeeSalaryData, EstimatesData, ExpensesData, PaymentsData };