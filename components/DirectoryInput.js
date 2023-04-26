export default function DirectoryInput({value}) {

    return (
        <div
            className="
                  m-auto px-3 py-1.5 text-base font-normal text-gray-700 bg-white inline-block w-full
                  border border-solid border-maingreen-light rounded transition ease-in-out mr-2
                  focus:text-gray-700 focus:bg-white focus:border-maingreen focus:outline-none overflow-hidden max-w-lg max-h-9"
        >
            {value}
        </div>
    );
}