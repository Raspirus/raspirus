import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { useTranslation } from 'next-i18next';

/**
 * Represents a card that allows users to interact with settings of the application
 * @param {String} title Title of the component, can also include HTML, like a Paragraph
 * @param {String} short A short description of what the setting does
 * @param {String | null} short2 Optional: Extension of the short description with possibility to add HTML
 * @param {Icon} icon A FontAwesomeIcon for the component
 * @param {Boolean} isOn Defines the initial state of the switch button
 * @param {Function | null} setIsOn Optional: If set, represents the function to trigger on button state switch with the state as parameter
 * @param {Function | null} action Optional: Instead of a switch function, you can also set a different function without params
 * @param {String | null} action_val The value text that is written on the Button
 * @returns A div-tag representing a single Card-like component with a title, an icon, a description and a button
 */
export default function SettingComp({ title, short, short2 = null, icon, isOn = false, setIsOn = null, action = null, action_val = null }) {
    const t = useTranslation('common').t;

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
                        <p className="text-sm text-gray-600 leading-none mt-1">{short}</p>
                        {short2 && <div className="text-sm text-gray-600 leading-none mt-1">{short2}</div>}
                    </div>
                </div>
                {action == null && 
                    <button onClick={() => setIsOn(!isOn)}
                    className={`flex-no-shrink px-5 ml-4 py-2 text-sm shadow-sm font-medium border-2 text-white rounded-full ${isOn ? 'bg-green-500 border-green-500' : 'bg-red-500 border-red-500'}`}>
                    {isOn ? t('settings_on') : t('settings_off')}
                </button>
                }

                {action != null &&
                    <button onClick={action}
                    className={'flex-no-shrink px-5 ml-4 py-2 text-sm shadow-sm font-medium border-2 text-white rounded-full bg-blue-500 border-blue-500'}>
                    {action_val}
                  </button>
                }
            </div>
        </div>
    );
}