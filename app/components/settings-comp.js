import { useState, useContext, useEffect } from 'react';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { SettingsContext } from '../state/context';


export default function SettingComp({ title, short, icon, isOn: initialIsOn }) {
    const titleWithoutSpaces = title.replace(/ /g, "");
    const {settings, setSettings} = useContext(SettingsContext);
    const [isOn, setIsOn] = useState(initialIsOn || settings[titleWithoutSpaces]);

    useEffect(() => {
        setSettings((prev) => ({ ...prev, [titleWithoutSpaces]: isOn }));
    }, [isOn, titleWithoutSpaces, setSettings])

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
                        <p className="text-sm text-gray-600 leading-none mt-1">{short}
                        </p>
                    </div>
                </div>
                <button
                    onClick={() => setIsOn(!isOn)}
                    className={`flex-no-shrink px-5 ml-4 py-2 text-sm shadow-sm hover:shadow-lg font-medium tracking-wider border-2 text-white rounded-full ${isOn ? 'bg-green-500 border-green-500' : 'bg-red-500 border-red-500'}`}>
                    {isOn ? 'ON' : 'OFF'}
                </button>
            </div>
        </div>
    );
}