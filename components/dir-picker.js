import Image from 'next/image';

export default function DirectoryPickerButton({ onSelectDirectory }) {

  const handleButtonClick = async () => {
    const { open } = await import('@tauri-apps/api/dialog');
    const selected = await open({
      directory: true,
      multiple: false,
      defaultPath: "/",
    })
    if (selected === null) {
      // No dir selected
    } else {
      onSelectDirectory(selected);
    }
  }

  return (
    <button
      onClick={handleButtonClick}
      className="ml-1 inline-block p-3 bg-orange-400 rounded shadow-md hover:bg-orange-500 hover:shadow-lg focus:bg-orange-500 focus:shadow-lg focus:outline-none focus:ring-0 active:orange-500 active:shadow-lg transition duration-150 ease-in-out"
    >
      <Image
        id="folder-icon"
        className="h-full w-4"
        src="images/folder.svg"
        alt="Folder"
        width={500}
        height={500}
      />
    </button>
  )
}