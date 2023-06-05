import TimePicker from 'react-time-picker';
import 'react-time-picker/dist/TimePicker.css';
import 'react-clock/dist/Clock.css';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { faChevronUp, faChevronDown } from '@fortawesome/free-solid-svg-icons';
import { addMinutes, format } from 'date-fns';

/**
 * Represents a custom Selector component to select a specific time in hours and minutes
 * @param {String} time A string defining time in the format HH:mm
 * @param {Function} setTime A function to update the time on parent
 * @returns a div tag containing the selectable time with a Timepicker and two arrows
 */
export default function DateTimeSelector({ time, setTime }) {
  // Function to set the new time when changed
  const onTimeChange = (newTime) => {
    setTime(newTime);
  };

  // Function to increment or decrement the given time by a set amount. Set amount as number
  const onArrowClick = (increment) => {
    // Split the time string in two numbers: hours and minutes
    const [hours, minutes] = time.split(':');
    // Create a temporary date that contains the hours
    const tempdate = new Date().setHours(hours, minutes);
    // Create the new date by also adding or decreasing the temporary date with the given amount
    const newdate = addMinutes(tempdate, increment);
    // Set the new time for the parent
    setTime(format(newdate, 'HH:mm'));
  };

  return (
    <div className='inline-block'>
      <TimePicker
        clearIcon={null}
        disableClock={true}
        format="HH:mm"
        value={time}
        onChange={onTimeChange}
        className={'py-2'}
      />
      <FontAwesomeIcon icon={faChevronUp} className="cursor-pointer text-gray-500 mx-1 px-2 border" onClick={() => onArrowClick(15)} />
      <FontAwesomeIcon icon={faChevronDown} className="cursor-pointer text-gray-500 mx-1 px-2 border" onClick={() => onArrowClick(-15)} />
    </div>
  );
}
