function WeekdaySelector({selectedWeekday, setSelectedWeekday}) {

    const handleWeekdayChange = (event) => {
        console.log("Unparsed: ", event.target.value)
        console.log("Parsed: ", parseInt(event.target.value))
        setSelectedWeekday(parseInt(event.target.value));
    };

    return (
        <div className="inline-block">
            <label htmlFor="weekday-selector">Weekday:</label>
            <select id="weekday-selector" value={selectedWeekday} onChange={handleWeekdayChange}>
                <option value="-1">Daily</option>
                <option value="0">Sunday</option>
                <option value="1">Monday</option>
                <option value="2">Tuesday</option>
                <option value="3">Wednesday</option>
                <option value="4">Thursday</option>
                <option value="5">Friday</option>
                <option value="6">Saturday</option>
            </select>
        </div>
    );
}

export default WeekdaySelector;
