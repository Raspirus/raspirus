import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';

/**
 * Represents a single Information unit in the Information page. It acts like the Angular Material Card.
 * @param {String} title The title of the Information being displayed
 * @param {String} value A short description of the information
 * @param {FontawesomeObject} icon An appropriate icon
 * @returns A <div> tag containing an FontawesomeIcon, a div for the title for custom HTML and a p for the value (desc)
 */
export default function InfoComp({ title, value, icon }) {

    return (
        <div className="flex flex-col m-6 p-2 bg-white rounded-2xl shadow-md">
            <div className="flex items-center justify-between mx-4">
                <div className="flex items-center">
                    <FontAwesomeIcon
                        icon={icon}
                        size="2x"
                        className="w-16 h-16 rounded-2xl p-3 border border-maingreen-light text-maingreen-light bg-green-50"
                    />
                    <div className="flex flex-col ml-3">
                        <div className="font-medium">{title}</div>
                        <p className="text-sm text-gray-600 mt-1">{value}
                        </p>
                    </div>
                </div>
            </div>
        </div>
    )
}