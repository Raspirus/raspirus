import Image from 'next/image';

/**
 * A button that once pressed opens an os-specific window that lets the user select a folder from the file-system.
 * The button uses the Tauri API to choose how to open the file-system window.
 * @param {Function} onSelectDirectory Function to trigger when a directory has been selected
 * @returns A button tag with a folder-icon Image
 */
export default function DirectoryPickerButton({ onSelectDirectory, scanDirectory }) {

  /** Handles the button click */
  const handleButtonClick = async () => {
    const { open } = await import('@tauri-apps/plugin-dialog');
    // Set user selection restrictions
    const selected = await open({
      directory: scanDirectory,
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
      className="ml-1 inline-block p-3 bg-orange-400 rounded shadow-md"
    >
      <Image
        id="folder-icon"
        className="h-full w-4"
        src="/images/folder.svg"
        alt="Folder"
        width={500}
        height={500}
      />
    </button>
  )
}