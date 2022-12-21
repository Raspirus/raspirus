export default function SettingComp({ title, short, action}) {
    return (
        <div className="flex flex-col p-8 bg-white">
        <div className="flex items-center justify-between">
            <div className="flex items-center">
                <svg xmlns="http://www.w3.org/2000/svg"
                    className="w-16 h-16 rounded-2xl p-3 border border-blue-100 text-blue-400 bg-blue-50" fill="none"
                    viewBox="0 0 24 24" stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                        d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path>
                </svg>
                <div className="flex flex-col ml-3">
                    <div className="font-medium leading-none">{title}</div>
                    <p className="text-sm text-gray-600 leading-none mt-1">{short}
                    </p>
                </div>
            </div>
            <button className="flex-no-shrink bg-red-500 px-5 ml-4 py-2 text-sm shadow-sm hover:shadow-lg font-medium tracking-wider border-2 border-red-500 text-white rounded-full">{action}</button>
        </div>
    </div>
    )
}