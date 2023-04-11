import useTranslation from 'next-translate/useTranslation';

export default function Dropdown({dictionary, value, setValue}) {
    const t = useTranslation('common').t;

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
                        px-3 py-1.5 text-base font-normal text-gray-700 bg-white inline-block mr-2 w-full
                        border border-solid border-maingreen-light rounded transition ease-in-out
                        focus:text-gray-700 focus:bg-white focus:border-maingreen focus:outline-none"
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
                  m-auto px-3 py-1.5 text-base font-normal text-gray-700 bg-white inline-block w-full
                  border border-solid border-maingreen-light rounded transition ease-in-out mr-2
                  focus:text-gray-700 focus:bg-white focus:border-maingreen focus:outline-none"
                >
                    {t('selection_empty_placeholder')}
                </div>
            )}
        </>
    );
}