export default function VirusComp({title, value}) {

    return (
      <div class="flex mb-4 items-center">
        <p class="w-full text-grey-darkest overflow-hidden">{title}</p>
        <button class="flex-no-shrink p-2 ml-4 mr-2 border-2 rounded hover:text-white text-green border-green hover:bg-green">Done</button>
        <button class="flex-no-shrink p-2 ml-2 border-2 rounded text-red border-red hover:text-white hover:bg-red">Remove</button>
      </div>
    )
}