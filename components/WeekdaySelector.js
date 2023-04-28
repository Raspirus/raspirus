import { useTranslation } from 'next-i18next'

function WeekdaySelector({selectedWeekday, setSelectedWeekday}) {
    const {t} = useTranslation('common');

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
