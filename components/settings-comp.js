export default function SettingComp({ title, short, action, color, clicked, icon}) {

    function setColor() {
        switch (color) {
            case "red":
                return (<button onClick={clicked} className="flex-no-shrink bg-red-500 px-5 ml-4 py-2 text-sm shadow-sm hover:shadow-lg font-medium tracking-wider border-2 border-red-500 text-white rounded-full">{action}</button>)
            case "blue":
                return (<button onClick={clicked} className="flex-no-shrink bg-blue-500 px-5 ml-4 py-2 text-sm shadow-sm hover:shadow-lg font-medium tracking-wider border-2 border-blue-500 text-white rounded-full">{action}</button>)
            case "green":
                return (<button onClick={clicked} className="flex-no-shrink bg-green-500 px-5 ml-4 py-2 text-sm shadow-sm hover:shadow-lg font-medium tracking-wider border-2 border-green-500 text-white rounded-full">{action}</button>)
            case "yellow":
                return (<button onClick={clicked} className="flex-no-shrink bg-yellow-500 px-5 ml-4 py-2 text-sm shadow-sm hover:shadow-lg font-medium tracking-wider border-2 border-yellow-500 text-white rounded-full">{action}</button>)
            default:
                return (<button onClick={clicked} className="flex-no-shrink bg-red-500 px-5 ml-4 py-2 text-sm shadow-sm hover:shadow-lg font-medium tracking-wider border-2 border-red-500 text-white rounded-full">{action}</button>)
        }
    }

    function setIcon() {
        switch (icon) {
            case "wrench":
                return (<svg xmlns="http://www.w3.org/2000/svg" className="w-16 h-16 rounded-2xl p-3 border border-blue-100 text-blue-400 bg-blue-50" viewBox="0 0 512 512"><path fill="currentColor" d="M352 320c88.4 0 160-71.6 160-160c0-15.3-2.2-30.1-6.2-44.2c-3.1-10.8-16.4-13.2-24.3-5.3l-76.8 76.8c-3 3-7.1 4.7-11.3 4.7H336c-8.8 0-16-7.2-16-16V118.6c0-4.2 1.7-8.3 4.7-11.3l76.8-76.8c7.9-7.9 5.4-21.2-5.3-24.3C382.1 2.2 367.3 0 352 0C263.6 0 192 71.6 192 160c0 19.1 3.4 37.5 9.5 54.5L19.9 396.1C7.2 408.8 0 426.1 0 444.1C0 481.6 30.4 512 67.9 512c18 0 35.3-7.2 48-19.9L297.5 310.5c17 6.2 35.4 9.5 54.5 9.5zM80 456c-13.3 0-24-10.7-24-24s10.7-24 24-24s24 10.7 24 24s-10.7 24-24 24z"/> </svg>)
            case "file-lines":
                return (<svg xmlns="http://www.w3.org/2000/svg" className="w-16 h-16 rounded-2xl p-3 border border-blue-100 text-blue-400 bg-blue-50" viewBox="0 0 384 512"><path fill="currentColor" d="M64 0C28.7 0 0 28.7 0 64V448c0 35.3 28.7 64 64 64H320c35.3 0 64-28.7 64-64V160H256c-17.7 0-32-14.3-32-32V0H64zM256 0V128H384L256 0zM112 256H272c8.8 0 16 7.2 16 16s-7.2 16-16 16H112c-8.8 0-16-7.2-16-16s7.2-16 16-16zm0 64H272c8.8 0 16 7.2 16 16s-7.2 16-16 16H112c-8.8 0-16-7.2-16-16s7.2-16 16-16zm0 64H272c8.8 0 16 7.2 16 16s-7.2 16-16 16H112c-8.8 0-16-7.2-16-16s7.2-16 16-16z"/></svg>)
            case "file-pen":
                return (<svg xmlns="http://www.w3.org/2000/svg" className="w-16 h-16 rounded-2xl p-3 border border-blue-100 text-blue-400 bg-blue-50" viewBox="0 0 576 512"><path fill="currentColor" d="M0 64C0 28.7 28.7 0 64 0H224V128c0 17.7 14.3 32 32 32H384V285.7l-86.8 86.8c-10.3 10.3-17.5 23.1-21 37.2l-18.7 74.9c-2.3 9.2-1.8 18.8 1.3 27.5H64c-35.3 0-64-28.7-64-64V64zm384 64H256V0L384 128zM549.8 235.7l14.4 14.4c15.6 15.6 15.6 40.9 0 56.6l-29.4 29.4-71-71 29.4-29.4c15.6-15.6 40.9-15.6 56.6 0zM311.9 417L441.1 287.8l71 71L382.9 487.9c-4.1 4.1-9.2 7-14.9 8.4l-60.1 15c-5.5 1.4-11.2-.2-15.2-4.2s-5.6-9.7-4.2-15.2l15-60.1c1.4-5.6 4.3-10.8 8.4-14.9z"/></svg>)
            case "console":
                return (<svg xmlns="http://www.w3.org/2000/svg" className="w-16 h-16 rounded-2xl p-3 border border-blue-100 text-blue-400 bg-blue-50" viewBox="0 0 640 512"><path fill="currentColor" d="M41.4 86.6c-12.5-12.5-12.5-32.8 0-45.3s32.8-12.5 45.3 0l192 192c12.5 12.5 12.5 32.8 0 45.3l-192 192c-12.5 12.5-32.8 12.5-45.3 0s-12.5-32.8 0-45.3L210.7 256 41.4 86.6zM288 416H576c17.7 0 32 14.3 32 32s-14.3 32-32 32H288c-17.7 0-32-14.3-32-32s14.3-32 32-32z"/></svg>)
            default:
                return (<svg xmlns="http://www.w3.org/2000/svg" className="w-16 h-16 rounded-2xl p-3 border border-blue-100 text-blue-400 bg-blue-50" viewBox="0 0 512 512"><path fill="currentColor" d="M352 320c88.4 0 160-71.6 160-160c0-15.3-2.2-30.1-6.2-44.2c-3.1-10.8-16.4-13.2-24.3-5.3l-76.8 76.8c-3 3-7.1 4.7-11.3 4.7H336c-8.8 0-16-7.2-16-16V118.6c0-4.2 1.7-8.3 4.7-11.3l76.8-76.8c7.9-7.9 5.4-21.2-5.3-24.3C382.1 2.2 367.3 0 352 0C263.6 0 192 71.6 192 160c0 19.1 3.4 37.5 9.5 54.5L19.9 396.1C7.2 408.8 0 426.1 0 444.1C0 481.6 30.4 512 67.9 512c18 0 35.3-7.2 48-19.9L297.5 310.5c17 6.2 35.4 9.5 54.5 9.5zM80 456c-13.3 0-24-10.7-24-24s10.7-24 24-24s24 10.7 24 24s-10.7 24-24 24z"/> </svg>)
        }
    }

    return (
        <div className="flex flex-col p-4 bg-white">
        <div className="flex items-center justify-between mx-4">
            <div className="flex items-center">
                {setIcon()}
                <div className="flex flex-col ml-3">
                    <div className="font-medium leading-none">{title}</div>
                    <p className="text-sm text-gray-600 leading-none mt-1">{short}
                    </p>
                </div>
            </div>
            {setColor()}
        </div>
    </div>
    )
}