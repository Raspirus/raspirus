import { useTranslation } from 'next-i18next';

/**
 * Displays a dropdown in the form of HTML <select> tags that are dynamically generated and filled.
 * The data for the dropdown is passed through the params
 * @param {Array} dictionary List of found USB storage units on the device
 * @param {String} value Value ID for the USB storage unit
 * @param {Function} setValue A function from the parent to set the selected USB storage unit
 * @returns If USB storage units were found, it returns a list of them, else a div-tag with some helpful message.
 */
export default function Dropdown({dictionary, value, setValue}) {
    const {t} = useTranslation('common');

    return (
        <>
            {Array.isArray(dictionary) && dictionary.length > 0 ? (
                <select
                    placeholder={t('selection_placeholder')}
                    value={value}
                    onChange={(e) => {
                        console.log("Changed drive: " + e.target.value);
                        setValue(e.target.value);
                    }}
                    className="
                        px-3 py-1.5 text-gray-700 bg-white inline-block mr-2 w-full
                        border border-solid border-maingreen-light rounded"
                >
                    <option value="None">{t('selection_placeholder')}</option>
                    {dictionary.map((item, i) => (
                        <option key={i} value={item.path}>
                            {item.name}
                        </option>
                    ))}
                </select>
            ) : (
                <div
                    className="
                  m-auto px-3 py-1.5 text-gray-700 bg-white inline-block w-full
                  border border-solid border-maingreen-light rounded"
                >
                    {t('selection_empty_placeholder')}
                </div>
            )}
        </>
    );
}