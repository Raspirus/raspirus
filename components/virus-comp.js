export default function VirusComp({title, value}) {

    return (
      <div class="flex mb-4 items-center shadow-md hover:shadow-lg p-2 bg-white rounded-xl">
        <p class="w-full text-grey-darkest overflow-hidden">{title}</p>
        <button class="flex-no-shrink p-2 ml-4 mr-2 border-2 rounded hover:text-white text-green border-green hover:bg-green">Done</button>
        <button class="flex-no-shrink p-2 ml-2 border-2 rounded text-red border-red hover:text-white hover:bg-red">Remove</button>
        <button  class="flex-no-shrink bg-red-500 px-5 ml-4 py-2 text-sm shadow-sm hover:shadow-lg font-medium tracking-wider border-2 border-red-500 text-white rounded-full">Delete</button>
      </div>
    )
}