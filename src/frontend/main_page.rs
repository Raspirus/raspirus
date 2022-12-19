/* Is the main page of the program and gets immediately displayed when the program starts.
It has the following components:
- A button to start the scan
- A button that opens a new window that displays some information about the project
- A button that opens a page to set some settings
- A dropdown menu that automatically displays all connected USB drives with their path and name

When the start button is clicked, the path of the selected USB drive is given to the filescanner
and the scan is started. While the scan is going, a small loading animation is shown (On a new page)

The application has a fixe size of 800x480, which is equal to the size of the Raspberry touchscreen
*/