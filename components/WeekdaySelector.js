import { useTranslation } from 'next-i18next';

/**
 * A custom selector to allow users to select a weekday in multiple languages
 * @param {Number} selectedWeekday Sets the selected weekday by number, -1 beeing daily, 0 being sunday and 6 being saturday
 * @param {Function} setSelectedWeekday A function to pass the selected weekday to the parent
 * @returns A div containing a selector with fixed options
 */
function WeekdaySelector({selectedWeekday, setSelectedWeekday}) {
    // Uses the translations module
    const {t} = useTranslation('common');

    // Updates the selected week on the parent on change detected
    const handleWeekdayChange = (event) => {
        setSelectedWeekday(parseInt(event.target.value));
    };

    return (
        <div className="inline-block">
            <label htmlFor="weekday-selector">{t('weekday')}:</label>
            <select id="weekday-selector" value={selectedWeekday} onChange={handleWeekdayChange}>
                <option value="-1">{t('daily')}</option>
                <option value="0">{t('sunday')}</option>
                <option value="1">{t('monday')}</option>
                <option value="2">{t('tuesday')}</option>
                <option value="3">{t('wednesday')}</option>
                <option value="4">{t('thursday')}</option>
                <option value="5">{t('friday')}</option>
                <option value="6">{t('saturday')}</option>
            </select>
        </div>
    );
}

export default WeekdaySelector;
