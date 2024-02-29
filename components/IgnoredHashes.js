import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { useTranslation } from 'next-i18next';
import { useState } from 'react';

/**
 * Represents a card that allows users to interact with settings of the application
 * @param {String} title Title of the component, can also include HTML, like a Paragraph
 * @param {String} short A short description of what the setting does
 * @param {Array} hashes Array containing the list of hashes to display
 * @param {Icon} icon A FontAwesomeIcon for the component
 * @param {Function} addHash Action to perform when a new hash should be added
 * @param {Function} removeHash Action to perform when a given hash should be deleted
 * @returns A div-tag representing a single Card-like component with a title, an icon, a description and a button
 */
export default function IgnoredHashComp({ title, short, hashes, icon, setHashes }) {
    const { t } = useTranslation('common');
    const [newHash, setNewHash] = useState('');
    const [isAdding, setIsAdding] = useState(false);

    const handleRemoveHash = (hashToRemove) => {
        const updatedHashes = hashes.filter((hash) => hash !== hashToRemove);
        // Update the hashes array
        setHashes(updatedHashes);
    };

    const handleAddHash = () => {
        if (isAdding) {
            if (newHash.trim() !== '') {
                // Add the newHash to the hashes array
                setHashes([...hashes, newHash]);
                setNewHash('');
                setIsAdding(false);
            }
        } else {
            // Enable input for adding a new hash
            setIsAdding(true);
        }
    };

    const handleCancelAdd = () => {
        setNewHash('');
        setIsAdding(false);
    };

    return (
        <div className="flex flex-col m-6 p-2 bg-white rounded-2xl shadow-md">
            <div className="flex items-center justify-between mx-4">
                <div className="flex items-center w-full">
                    <FontAwesomeIcon
                        icon={icon}
                        size="2x"
                        className="w-16 h-16 rounded-2xl p-3 border border-maingreen-light text-maingreen-light bg-green-50"
                    />
                    <div className="flex flex-col ml-3 w-full">
                        <div className="font-medium">{title}</div>
                        <p className="text-sm text-gray-600 leading-none mt-1">{short}</p>
                        {/* List of Hashes */}
                        <div className="mt-2">
                            <ul className="space-y-2 justify-between">
                                {hashes.map((hash, index) => (
                                    <li key={index} className="flex items-center border-b pb-2">
                                        <span>{hash}</span>
                                        <button
                                            onClick={() => handleRemoveHash(hash)}
                                            className="ml-auto text-red-500 hover:text-red-700 uppercase"
                                        >
                                            {t('remove')}
                                        </button>
                                    </li>
                                ))}
                            </ul>
                        </div>
                        {/* Input field for adding a new hash */}
                        {isAdding && (
                            <div className="flex items-center mt-2">
                                <input
                                    type="text"
                                    className="flex-grow p-1 border border-gray-400 rounded"
                                    placeholder={t('add_hash_placeholder')}
                                    value={newHash}
                                    onChange={(e) => setNewHash(e.target.value)}
                                />
                                <button
                                    onClick={handleCancelAdd}
                                    className="ml-2 text-red-500 hover:text-red-700 uppercase"
                                >
                                    {t('cancel')}
                                </button>
                                <button
                                    onClick={handleAddHash}
                                    className="ml-2 text-blue-500 hover:text-blue-700 uppercase"
                                >
                                    {t('confirm')}
                                </button>
                            </div>
                        )}

                        {!isAdding && (
                            <button
                                onClick={handleAddHash}
                                className="mt-2 text-blue-500 hover:text-blue-700 uppercase"
                            >
                                {t('add')}
                            </button>
                        )}
                    </div>
                </div>
            </div>
        </div>
    );
}