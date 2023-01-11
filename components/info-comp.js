import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';

export default function InfoComp({ title, value, icon }) {

    return (
        <div className="flex flex-col p-4 bg-white">
            <div className="flex items-center justify-between mx-4">
                <div className="flex items-center">
                    <FontAwesomeIcon
                        icon={icon}
                        size="2x"
                        className="w-16 h-16 rounded-2xl p-3 border border-red-100 text-red-400 bg-red-50"
                    />
                    <div className="flex flex-col ml-3">
                        <div className="font-medium leading-none">{title}</div>
                        <p className="text-sm text-gray-600 leading-none mt-1">{value}
                        </p>
                    </div>
                </div>
            </div>
        </div>
    )
}