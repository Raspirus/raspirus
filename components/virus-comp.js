export default function VirusComp({title, value}) {

    return (
        <div className="p-5 bg-white rounded-lg flex items-center justify-between space-x-8">
        <div className="flex-1 flex justify-between items-center">
          <div className="h-6 w-3/5 bg-gray-300 rounded">{title}</div>
          <div className="w-2/5 h-8 rounded-lg bg-red-300">{value}</div>
        </div>
      </div>
    )
}