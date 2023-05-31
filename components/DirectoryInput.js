/**
 * A simple component that shows the path of the selected directory if one has been selected
 * @param {String} value Path of the directory as string
 * @returns A div tag containing the path of the directory beautified
 */
export default function DirectoryInput({value}) {

    return (
        <div
            className="
                  m-auto px-3 py-1.5 text-gray-700 bg-white inline-block w-full
                  border border-solid border-maingreen-light rounded overflow-hidden max-w-lg max-h-9"
        >
            {value}
        </div>
    );
}