<script>
    import HeadTitle from "../../../../../common/components/HeadTitle.svelte";
	import Breadcrumb from "../../../../../common/components/Breadcrumb.svelte";
	import LucideIcon from "../../../../../common/components/LucideIcon.svelte";
	import Modal from "../../../../../common/components/Modal.svelte";
    import { FriendsData } from "../../../../../common/data/socialMedia"
    import Flatpickr from "svelte-flatpickr";
    import 'flatpickr/dist/flatpickr.css';
	import Dropdown from "../../../../../common/components/Dropdown.svelte";
	import DropdownToggle from "../../../../../common/components/DropdownToggle.svelte";
	import DropdownMenu from "../../../../../common/components/DropdownMenu.svelte";
	import Sidebar from "../Sidebar.svelte";

    let isAddModal = false;
    const toggleAdd = () => isAddModal = !isAddModal;

    let isDeleteModal = false;
    const toggleDelete = () => isDeleteModal = !isDeleteModal;

    function toggleFollow(e){
        if(e){
            e.target.parentElement.classList.toggle("active")
        }
    }
</script>
<HeadTitle title="Friends" />

<div class="container-fluid group-data-[content=boxed]:max-w-boxed mx-auto relative">
    <Breadcrumb title="Friends" pagetitle="Social Media" />
    <div class="grid grid-cols-1 xl:grid-cols-12 gap-x-5">
        <Sidebar link="friends"/>

        <div class="xl:col-span-9" id="friendList">
            <div class="flex items-center gap-3">
                <h6 class="text-15 grow">Followers (254)</h6>
                <div class="shrink-0">
                    <Dropdown className="relative" direction="bottom-start">
                        <DropdownToggle
                            className="inline-block py-3">
                            <span class="dropdown-title">Following</span> <LucideIcon name="ChevronDown" class="inline-block size-4 ml-1"/>
                        </DropdownToggle>
                        <DropdownMenu tag="ul"
                            class="absolute z-50 py-2 mt-1 ltr:text-left rtl:text-right list-none bg-white rounded-md shadow-md dropdown-menu min-w-[10rem] dark:bg-zink-600">
                            <li>
                                <a data-sort="friend_name" class="block dropdown-item px-4 py-1.5 text-base font-medium transition-all duration-200 ease-linear text-slate-600 dropdown-item hover:bg-slate-100 hover:text-slate-500 focus:bg-slate-100 focus:text-slate-500 dark:text-zink-100 dark:hover:bg-zink-500 dark:hover:text-zink-200 dark:focus:bg-zink-500 dark:focus:text-zink-200" href="#!">Name</a>
                            </li>
                            <li>
                                <a data-sort="username" class="block dropdown-item px-4 py-1.5 text-base font-medium transition-all duration-200 ease-linear text-slate-600 dropdown-item hover:bg-slate-100 hover:text-slate-500 focus:bg-slate-100 focus:text-slate-500 dark:text-zink-100 dark:hover:bg-zink-500 dark:hover:text-zink-200 dark:focus:bg-zink-500 dark:focus:text-zink-200" href="#!">Username</a>
                            </li>
                            <li>
                                <a data-sort="date" class="block dropdown-item px-4 py-1.5 text-base font-medium transition-all duration-200 ease-linear text-slate-600 dropdown-item hover:bg-slate-100 hover:text-slate-500 focus:bg-slate-100 focus:text-slate-500 dark:text-zink-100 dark:hover:bg-zink-500 dark:hover:text-zink-200 dark:focus:bg-zink-500 dark:focus:text-zink-200" href="#!">Date</a>
                            </li>
                            <li>
                                <a data-sort="status" class="block dropdown-item px-4 py-1.5 text-base font-medium transition-all duration-200 ease-linear text-slate-600 dropdown-item hover:bg-slate-100 hover:text-slate-500 focus:bg-slate-100 focus:text-slate-500 dark:text-zink-100 dark:hover:bg-zink-500 dark:hover:text-zink-200 dark:focus:bg-zink-500 dark:focus:text-zink-200" href="#!">Status</a>
                            </li>
                        </DropdownMenu>
                    </Dropdown>
                </div>
            </div>

            <div class="overflow-x-auto">
                <table class="w-full border-separate table-custom border-spacing-y-2 whitespace-nowrap">
                    <thead class="ltr:text-left rtl:text-right">
                        <tr class="relative bg-white rounded-md dark:bg-zink-700">
                            <th class="px-3.5 py-2.5 font-semibold sort" data-sort="friend_name">Friend Name</th>
                            <th class="px-3.5 py-2.5 font-semibold sort" data-sort="username">Username</th>
                            <th class="px-3.5 py-2.5 font-semibold sort" data-sort="date">Joining Date</th>
                            <th class="px-3.5 py-2.5 font-semibold sort" data-sort="status">Status</th>
                        </tr>
                    </thead>
                    <tbody class="list">
                        {#each FriendsData as row}
                        <tr class="relative bg-white rounded-md dark:bg-zink-700">
                            <td class="px-3.5 py-2.5 friend_name">{row.name}</td>
                            <td class="px-3.5 py-2.5"><a href="#!" class="text-custom-500 username">{row.username}</a></td>
                            <td class="px-3.5 py-2.5 date">{row.joiningDate}</td>
                            <td class="px-3.5 py-2.5">
                                <button type="button" class="bg-white border-dashed group/item status toggle-button text-sky-500 btn border-sky-500 hover:text-sky-500 hover:bg-sky-50 hover:border-sky-600 focus:text-sky-600 focus:bg-sky-50 focus:border-sky-600 active:text-sky-600 active:bg-sky-50 active:border-sky-600 dark:bg-zink-700 dark:ring-sky-400/20 dark:hover:bg-sky-800/20 dark:focus:bg-sky-800/20 dark:active:bg-sky-800/20 px-2 py-1.5 text-xs {row.isFollow ? "active":""}" on:click={(e) => toggleFollow(e)}>
                                    <span class="group-[.active]/item:hidden block">
                                        <LucideIcon name="Plus" class="inline-block size-3 mr-1"/> Follow</span>
                                    <span class="group-[.active]/item:block hidden">
                                        <LucideIcon name="UserX2" class="inline-block size-3 mr-1" /> Unfollow</span>
                                </button>
                            </td>
                        </tr>
                        {/each}
                    </tbody>
                </table>
            </div>
            <div class="flex flex-col items-center mt-4 mb-5 md:flex-row">
                <div class="mb-4 grow md:mb-0">
                    <p class="text-slate-500 dark:text-zink-200">Showing <b>12</b> of <b>44</b> Results</p>
                </div>
                <ul class="flex flex-wrap items-center gap-2 shrink-0">
                    <li>
                        <a href="#!" class="inline-flex items-center justify-center bg-white dark:bg-zink-700 h-8 px-3 transition-all duration-150 ease-linear border rounded border-slate-200 dark:border-zink-500 text-slate-500 dark:text-zink-200 hover:text-custom-500 dark:hover:text-custom-500 hover:bg-custom-100 dark:hover:bg-custom-500/10 focus:bg-custom-50 dark:focus:bg-custom-500/10 focus:text-custom-500 dark:focus:text-custom-500 [&.active]:text-white dark:[&.active]:text-white [&.active]:bg-custom-500 dark:[&.active]:bg-custom-500 [&.active]:border-custom-500 dark:[&.active]:border-custom-500 [&.active]:hover:text-custom-700 dark:[&.active]:hover:text-custom-700 [&.disabled]:text-slate-400 dark:[&.disabled]:text-zink-300 [&.disabled]:cursor-auto"><LucideIcon class="size-4 mr-1 rtl:rotate-180" name="ChevronLeft"/> Prev</a>
                    </li>
                    <li>
                        <a href="#!" class="inline-flex items-center justify-center bg-white dark:bg-zink-700 size-8 transition-all duration-150 ease-linear border rounded border-slate-200 dark:border-zink-500 text-slate-500 dark:text-zink-200 hover:text-custom-500 dark:hover:text-custom-500 hover:bg-custom-100 dark:hover:bg-custom-500/10 focus:bg-custom-50 dark:focus:bg-custom-500/10 focus:text-custom-500 dark:focus:text-custom-500 [&.active]:text-white dark:[&.active]:text-white [&.active]:bg-custom-500 dark:[&.active]:bg-custom-500 [&.active]:border-custom-500 dark:[&.active]:border-custom-500 [&.active]:hover:text-custom-700 dark:[&.active]:hover:text-custom-700 [&.disabled]:text-slate-400 dark:[&.disabled]:text-zink-300 [&.disabled]:cursor-auto">1</a>
                    </li>
                    <li>
                        <a href="#!" class="inline-flex items-center justify-center bg-white dark:bg-zink-700 size-8 transition-all duration-150 ease-linear border rounded border-slate-200 dark:border-zink-500 text-slate-500 dark:text-zink-200 hover:text-custom-500 dark:hover:text-custom-500 hover:bg-custom-100 dark:hover:bg-custom-500/10 focus:bg-custom-50 dark:focus:bg-custom-500/10 focus:text-custom-500 dark:focus:text-custom-500 [&.active]:text-white dark:[&.active]:text-white [&.active]:bg-custom-500 dark:[&.active]:bg-custom-500 [&.active]:border-custom-500 dark:[&.active]:border-custom-500 [&.active]:hover:text-custom-700 dark:[&.active]:hover:text-custom-700 [&.disabled]:text-slate-400 dark:[&.disabled]:text-zink-300 [&.disabled]:cursor-auto active">2</a>
                    </li>
                    <li>
                        <a href="#!" class="inline-flex items-center justify-center bg-white dark:bg-zink-700 size-8 transition-all duration-150 ease-linear border rounded border-slate-200 dark:border-zink-500 text-slate-500 dark:text-zink-200 hover:text-custom-500 dark:hover:text-custom-500 hover:bg-custom-100 dark:hover:bg-custom-500/10 focus:bg-custom-50 dark:focus:bg-custom-500/10 focus:text-custom-500 dark:focus:text-custom-500 [&.active]:text-white dark:[&.active]:text-white [&.active]:bg-custom-500 dark:[&.active]:bg-custom-500 [&.active]:border-custom-500 dark:[&.active]:border-custom-500 [&.active]:hover:text-custom-700 dark:[&.active]:hover:text-custom-700 [&.disabled]:text-slate-400 dark:[&.disabled]:text-zink-300 [&.disabled]:cursor-auto">3</a>
                    </li>
                    <li>
                        <a href="#!" class="inline-flex items-center justify-center bg-white dark:bg-zink-700 size-8 transition-all duration-150 ease-linear border rounded border-slate-200 dark:border-zink-500 text-slate-500 dark:text-zink-200 hover:text-custom-500 dark:hover:text-custom-500 hover:bg-custom-100 dark:hover:bg-custom-500/10 focus:bg-custom-50 dark:focus:bg-custom-500/10 focus:text-custom-500 dark:focus:text-custom-500 [&.active]:text-white dark:[&.active]:text-white [&.active]:bg-custom-500 dark:[&.active]:bg-custom-500 [&.active]:border-custom-500 dark:[&.active]:border-custom-500 [&.active]:hover:text-custom-700 dark:[&.active]:hover:text-custom-700 [&.disabled]:text-slate-400 dark:[&.disabled]:text-zink-300 [&.disabled]:cursor-auto">4</a>
                    </li>
                    <li>
                        <a href="#!" class="inline-flex items-center justify-center bg-white dark:bg-zink-700 size-8 transition-all duration-150 ease-linear border rounded border-slate-200 dark:border-zink-500 text-slate-500 dark:text-zink-200 hover:text-custom-500 dark:hover:text-custom-500 hover:bg-custom-100 dark:hover:bg-custom-500/10 focus:bg-custom-50 dark:focus:bg-custom-500/10 focus:text-custom-500 dark:focus:text-custom-500 [&.active]:text-white dark:[&.active]:text-white [&.active]:bg-custom-500 dark:[&.active]:bg-custom-500 [&.active]:border-custom-500 dark:[&.active]:border-custom-500 [&.active]:hover:text-custom-700 dark:[&.active]:hover:text-custom-700 [&.disabled]:text-slate-400 dark:[&.disabled]:text-zink-300 [&.disabled]:cursor-auto">5</a>
                    </li>
                    <li>
                        <a href="#!" class="inline-flex items-center justify-center bg-white dark:bg-zink-700 h-8 px-3 transition-all duration-150 ease-linear border rounded border-slate-200 dark:border-zink-500 text-slate-500 dark:text-zink-200 hover:text-custom-500 dark:hover:text-custom-500 hover:bg-custom-100 dark:hover:bg-custom-500/10 focus:bg-custom-50 dark:focus:bg-custom-500/10 focus:text-custom-500 dark:focus:text-custom-500 [&.active]:text-white dark:[&.active]:text-white [&.active]:bg-custom-500 dark:[&.active]:bg-custom-500 [&.active]:border-custom-500 dark:[&.active]:border-custom-500 [&.active]:hover:text-custom-700 dark:[&.active]:hover:text-custom-700 [&.disabled]:text-slate-400 dark:[&.disabled]:text-zink-300 [&.disabled]:cursor-auto">Next <LucideIcon class="size-4 ml-1 rtl:rotate-180" name="ChevronRight"/></a>
                    </li>
                </ul>
            </div>
        </div>
    </div>
</div>

<Modal modal-center className="-translate-y-2/4" isOpen={isAddModal} toggle={toggleAdd}>
    <div class="w-screen xl:w-[55rem] bg-white shadow rounded-md dark:bg-zink-600">
        <div class="flex items-center justify-between p-5 border-b dark:border-zink-500">
            <h5 class="text-16" id="addNewNoteLabel">Add Note</h5>
            <button data-modal-close="addNotesModal" id="notesModal-close" class="transition-all duration-200 ease-linear text-slate-400 hover:text-red-500" on:click={toggleAdd}><LucideIcon name="X" class="size-5"/></button>
        </div>
        <div class="max-h-[calc(theme('height.screen')_-_180px)] overflow-y-auto p-5">
            <form novalidate class="create-form">   
                <input type="hidden" value="" name="id" id="id">
                <input type="hidden" value="add" name="action" id="action">
                <input type="hidden" id="id-field">
                <div id="alert-error-msg" class="hidden px-4 py-3 text-sm text-red-500 border border-transparent rounded-md bg-red-50 dark:bg-red-400/20"></div>
                <div class="grid grid-cols-1 gap-5 xl:grid-cols-12">
                    <div class="xl:col-span-4">
                        <label for="createDateInput" class="inline-block mb-2 text-base font-medium">Create Date</label>
                        <Flatpickr type="text" options={{
                            dateFormat:"d M, Y"
                        }} id="fromInput" class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200" placeholder="Select date" />
                    </div>
                    <div class="xl:col-span-4">
                        <label for="notesTitleInput" class="inline-block mb-2 text-base font-medium">Note Title</label>
                        <input type="text" id="notesTitleInput" class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200" placeholder="Title" required>
                    </div>
                    <div class="xl:col-span-4">
                        <div>
                            <label for="categorySelect" class="inline-block mb-2 text-base font-medium">Category</label>
                            <select class="form-select border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200" data-choices data-choices-search-false name="categorySelect" id="statusSelect">
                                <option value="">Select Category</option>
                                <option value="business">Business</option>
                                <option value="personal">Personal</option>
                                <option value="home">Home</option>
                                <option value="social">Social</option>
                            </select>
                        </div>
                    </div>
                    <div class="xl:col-span-12">
                        <div>
                            <label for="textArea" class="inline-block mb-2 text-base font-medium">Description</label>
                            <textarea class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200" id="textArea" rows="6"></textarea>
                        </div>
                    </div>
                </div>
                
                <div class="flex justify-end gap-2 mt-4">
                    <button type="reset" data-modal-close="addNotesModal" class="text-red-500 bg-white btn hover:text-red-500 hover:bg-red-100 focus:text-red-500 focus:bg-red-100 active:text-red-500 active:bg-red-100 dark:bg-zink-600 dark:hover:bg-red-500/10 dark:focus:bg-red-500/10 dark:active:bg-red-500/10" on:click={toggleAdd}>Cancel</button>
                    <button type="submit" id="addNew" class="text-white btn bg-custom-500 border-custom-500 hover:text-white hover:bg-custom-600 hover:border-custom-600 focus:text-white focus:bg-custom-600 focus:border-custom-600 focus:ring focus:ring-custom-100 active:text-white active:bg-custom-600 active:border-custom-600 active:ring active:ring-custom-100 dark:ring-custom-400/20">Add Note</button>
                </div>
            </form>
        </div>
    </div>
</Modal>

<Modal modal-center className="-translate-y-2/4" isOpen={isDeleteModal} toggle={toggleDelete}>
    <div class="w-screen md:w-[25rem] bg-white shadow rounded-md dark:bg-zink-600">
        <div class="max-h-[calc(theme('height.screen')_-_180px)] overflow-y-auto px-6 py-8">
            <div class="float-right">
                <button class="transition-all duration-200 ease-linear text-slate-500 hover:text-red-500" on:click={toggleDelete}><LucideIcon name="X" class="size-5"/></button>
            </div>
            <img src="/assets/images/delete.png" alt="" class="block h-12 mx-auto">
            <div class="mt-5 text-center">
                <h5 class="mb-1">Are you sure?</h5>
                <p class="text-slate-500 dark:text-zink-200">Are you certain you want to delete this record?</p>
                <div class="flex justify-center gap-2 mt-6">
                    <button type="reset" class="bg-white text-slate-500 btn hover:text-slate-500 hover:bg-slate-100 focus:text-slate-500 focus:bg-slate-100 active:text-slate-500 active:bg-slate-100 dark:bg-zink-600 dark:hover:bg-slate-500/10 dark:focus:bg-slate-500/10 dark:active:bg-slate-500/10" on:click={toggleDelete}>Cancel</button>
                    <button type="submit" class="text-white bg-red-500 border-red-500 btn hover:text-white hover:bg-red-600 hover:border-red-600 focus:text-white focus:bg-red-600 focus:border-red-600 focus:ring focus:ring-red-100 active:text-white active:bg-red-600 active:border-red-600 active:ring active:ring-red-100 dark:ring-custom-400/20">Yes, Delete It!</button>
                </div>
            </div>
        </div>
    </div>
</Modal>