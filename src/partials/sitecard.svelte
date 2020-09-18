<script lang="ts">
  import { timer } from '../timer';
  import { interval } from 'rxjs';
  import { map, startWith } from 'rxjs/operators';

 const rxTimer = interval(1000).pipe(
    startWith(0),
    map(v => v * 2)
  );
</script>

<div>
  <h2 class="text-gray-500 text-xs font-medium uppercase tracking-wide">
    Pinned Projects
  </h2>
  <ul
    class="mt-3 grid grid-cols-1 gap-5 sm:gap-6 sm:grid-cols-2 lg:grid-cols-4"
  >
    <li
      class="col-span-1 flex items-center bg-white border border-gray-200
        rounded-md shadow-sm overflow-hidden"
    >
      <div
        class="flex-shrink-0 flex items-center justify-center w-16 h-16
          text-white text-center text-sm leading-5 font-medium bg-pink-600"
      >
        GA
      </div>
      <div class="flex-1 px-4 py-2 truncate">
        <a
          href="/#"
          class="text-gray-900 text-sm leading-5 font-medium hover:text-gray-600
            transition ease-in-out duration-150"
        >Graph API</a>
        <p class="text-sm leading-5 text-gray-500">16 Members</p>
      </div>
      <div class="flex-shrink-0 pr-2">
        <button
          class="w-8 h-8 inline-flex items-center justify-center text-gray-400
            rounded-full bg-transparent hover:text-gray-500 focus:outline-none
            focus:text-gray-500 focus:bg-gray-100 transition ease-in-out
            duration-150"
        >
          <svg class="w-5 h-5" viewBox="0 0 20 20" fill="currentColor">
            <path
              d="M10 6a2 2 0 110-4 2 2 0 010 4zM10 12a2 2 0 110-4 2 2 0 010 4zM10 18a2 2 0 110-4 2 2 0 010 4z"
            ></path>
          </svg>
        </button>
      </div>
    </li>
    <li
      class="col-span-1 flex items-center bg-white border border-gray-200
        rounded-md shadow-sm overflow-hidden"
    >
      <div
        class="flex-shrink-0 flex items-center justify-center w-16 h-16
          text-white text-center text-sm leading-5 font-medium bg-purple-600"
      >
        CD
      </div>
      <div class="flex-1 px-4 py-2 truncate">
        <a
          href="/#"
          class="text-gray-900 text-sm leading-5 font-medium hover:text-gray-600
            transition ease-in-out duration-150"
        >Component Design</a>
        <p class="text-sm leading-5 text-gray-500">12 Members</p>
      </div>
      <div class="flex-shrink-0 pr-2">
        <button
          class="w-8 h-8 inline-flex items-center justify-center text-gray-400
            rounded-full bg-transparent hover:text-gray-500 focus:outline-none
            focus:text-gray-500 focus:bg-gray-100 transition ease-in-out
            duration-150"
        >
          <svg class="w-5 h-5" viewBox="0 0 20 20" fill="currentColor">
            <path
              d="M10 6a2 2 0 110-4 2 2 0 010 4zM10 12a2 2 0 110-4 2 2 0 010 4zM10 18a2 2 0 110-4 2 2 0 010 4z"
            ></path>
          </svg>
        </button>
      </div>
    </li>
    <li
      class="col-span-1 flex items-center bg-white border border-gray-200
        rounded-md shadow-sm overflow-hidden"
    >
      <div
        class="flex-shrink-0 flex items-center justify-center w-16 h-16
          text-white text-center text-sm leading-5 font-medium bg-orange-500"
      >
        TSC
      </div>
      <div class="flex-1 px-4 py-2 truncate">
        <a
          href="/#"
          class="text-gray-900 text-sm leading-5 font-medium hover:text-gray-600
            transition ease-in-out duration-150"
        >Timer</a>
        <p class="text-sm leading-5 text-gray-500">{$timer}</p>
      </div>
      <div class="flex-shrink-0 pr-2">
        <button
          class="w-8 h-8 inline-flex items-center justify-center text-gray-400
            rounded-full bg-transparent hover:text-gray-500 focus:outline-none
            focus:text-gray-500 focus:bg-gray-100 transition ease-in-out
            duration-150"
        >
          <svg class="w-5 h-5" viewBox="0 0 20 20" fill="currentColor">
            <path
              d="M10 6a2 2 0 110-4 2 2 0 010 4zM10 12a2 2 0 110-4 2 2 0 010 4zM10 18a2 2 0 110-4 2 2 0 010 4z"
            ></path>
          </svg>
        </button>
      </div>
    </li>
    <li
      class="col-span-1 flex items-center bg-white border border-gray-200
        rounded-md shadow-sm overflow-hidden"
    >
      <div
        class="flex-shrink-0 flex items-center justify-center w-16 h-16
          text-white text-center text-sm leading-5 font-medium bg-green-400"
      >
        Rx
      </div>
      <div class="flex-1 px-4 py-2 truncate">
        <a
          href="/#"
          class="text-gray-900 text-sm leading-5 font-medium hover:text-gray-600
            transition ease-in-out duration-150"
        >Timer</a>
        <p class="text-sm leading-5 text-gray-500">{$rxTimer}</p>
      </div>
      <div class="flex-shrink-0 pr-2">
        <button
          class="w-8 h-8 inline-flex items-center justify-center text-gray-400
            rounded-full bg-transparent hover:text-gray-500 focus:outline-none
            focus:text-gray-500 focus:bg-gray-100 transition ease-in-out
            duration-150"
        >
          <svg class="w-5 h-5" viewBox="0 0 20 20" fill="currentColor">
            <path
              d="M10 6a2 2 0 110-4 2 2 0 010 4zM10 12a2 2 0 110-4 2 2 0 010 4zM10 18a2 2 0 110-4 2 2 0 010 4z"
            ></path>
          </svg>
        </button>
      </div>
    </li>
  </ul>
</div>
