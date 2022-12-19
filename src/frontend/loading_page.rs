/* Is the page that is displayed while the program is scanning the selected sub drive.
It has the following components:
- A gif that displayes an animation to entertain the user while its scanning
- (Optional) A button to stop the scan

When the scan has ended, we look at the returned list fo dirty files. If it is empty,
we redirect the user to a "cleanPage", where we congratulate ourselves for having a clean
usb drive. Else we redirect the user to a "viruspage", where we list all found dirty
files with their path and some suggested actions.
*/