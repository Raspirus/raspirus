import React from "react";

/**
 * This component visually represents a found virus by displaying its filename and path in a card
 * @param {String} title The name of the file without extension
 * @param {String} text The absolute path to that file 
 * @returns A div-tag shaped like a card and displayable in a list
 */
export default function VirusComp({ title, text }) {

  return (
    <div className="flex mb-4 items-center shadow-md p-2 bg-white rounded-xl">
      <p className="whitespace-nowrap text-grey-darkest w-1/5 overflow-hidden">{title}</p>
      <div className="inline-block min-h-[1em] w-0.5 self-stretch bg-maingreen opacity-100 mx-2"></div>
      <p className="w-full text-grey-darkest overflow-x-auto">{text}</p>
    </div>
  )
}