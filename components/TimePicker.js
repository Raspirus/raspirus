import TimePicker from 'react-time-picker';
import 'react-time-picker/dist/TimePicker.css';
import 'react-clock/dist/Clock.css';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { faChevronUp, faChevronDown } from '@fortawesome/free-solid-svg-icons';
import { addMinutes, format } from 'date-fns';

export default function DateTimeSelector({ time, setTime }) {
  const onTimeChange = (newTime) => {
    setTime(newTime);
  };

  const onArrowClick = (increment) => {
    const [hours, minutes] = time.split(':');
    const tempdate = new Date().setHours(hours, minutes);
    const newdate = addMinutes(tempdate, increment);
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
      <FontAwesomeIcon icon={faChevronUp} className="cursor-pointer text-gray-500 hover:text-gray-700 mx-1 px-2 border" onClick={() => onArrowClick(15)} />
      <FontAwesomeIcon icon={faChevronDown} className="cursor-pointer text-gray-500 hover:text-gray-700 mx-1 px-2 border" onClick={() => onArrowClick(-15)} />
    </div>
  );
}
